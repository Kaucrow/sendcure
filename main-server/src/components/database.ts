import {
  Pool as PgPool,
  type PoolClient as PgPoolClient
} from 'pg';
import mysql, {
  type RowDataPacket,
  type Pool as MySQLPool,
  type PoolConnection as MySQLPoolConnection,
} from 'mysql2/promise';
import {
  DbError,
  DbConflictError,
  DbSchemaValidationError,
  DbNotNullViolationError
} from '@errors/index.js';
import { z } from 'zod';
import { objectToCamel, type ObjectToCamel } from 'ts-case-convert';
import { database as dbConfig } from '@global/constants.js';
import type { ZodType } from "zod";

const PG_UNIQUE_VIOLATION = '23505';
const PG_NOT_NULL_VIOLATION = '23502';
const MYSQL_DUPLICATE_ENTRY = 1062;
const MYSQL_BAD_NULL_ERROR = 1048;

class DatabaseComponent {
  static #instance: DatabaseComponent;

  private type: 'postgresql' | 'mysql' | undefined = undefined;
  private dbPool: PgPool | MySQLPool | undefined = undefined;

  private constructor() {}

  public static get instance(): DatabaseComponent {
    if (!DatabaseComponent.#instance) {
      DatabaseComponent.#instance = new DatabaseComponent();
    }
    return DatabaseComponent.#instance;
  }

  public async connect(type: 'postgresql' | 'mysql') {
    this.type = type;

    switch (type) {
      case 'postgresql': {
        this.dbPool = new PgPool({
          host: dbConfig.host,
          port: dbConfig.port,
          database: dbConfig.name,
          user: dbConfig.user,
          password: dbConfig.pass
        });
        break;
      }
      case 'mysql': {
        this.dbPool = mysql.createPool({
          host: dbConfig.host,
          port: dbConfig.port,
          database: dbConfig.name,
          user: dbConfig.user,
          password: dbConfig.pass
        });
      }
    }
  }

  public async fetchOne<T extends object>(
    sql: string,
    schema: ZodType<T>,
    args?: any[],
    client?: PgPoolClient | MySQLPoolConnection
  ): Promise<ObjectToCamel<T> | null>
  {
    if (!this.dbPool || !this.type) throw new Error("Database connection has not been initialized. Call db.connect() first.");

    let row: unknown;

    try {
      const executor = client || this.dbPool;
      switch (this.type) {
        case 'postgresql': {
          const result = await (executor as PgPool | PgPoolClient).query(sql, args);

          // Get the first row
          row = result.rows[0];
          break;
        }
        case 'mysql': {
          const [rows] = await (executor as MySQLPool | MySQLPoolConnection).query<RowDataPacket[]>(sql, args);

          // Get the first row
          row = rows[0];
          break;
        }
      }
    } catch (err: any) {
      // Handle NOT NULL database errors
      if (err.code === PG_NOT_NULL_VIOLATION || err.errno === MYSQL_BAD_NULL_ERROR) {
        throw new DbNotNullViolationError("Violation of NOT NULL constraint", err.detail);
      }
      // Handle other database errors
      throw new DbError(`Failed to fetch single row: ${err}`);
    }

    if (!row) return null;

    try {
      return objectToCamel(schema.parse(row));
    } catch (err) {
      throw new DbSchemaValidationError("Database record failed app schema validation.", row);
    }
  }

  public async fetch<T extends object>(
    sql: string,
    schema: ZodType<T>,
    args?: any[],
    client?: PgPoolClient | MySQLPoolConnection
  ): Promise<ObjectToCamel<T>[]>
  {
    if (!this.dbPool || !this.type) throw new Error("Database connection has not been initialized. Call db.connect() first.");

    let rows: unknown[];

    try {
      const executor = client || this.dbPool;
      switch (this.type) {
        case 'postgresql': {
          const result = await (executor as PgPool | PgPoolClient).query(sql, args);

          // Get all rows
          rows = result.rows;
          break;
        }
        case 'mysql': {
          const [rawRows] = await (executor as MySQLPool | MySQLPoolConnection).query<RowDataPacket[]>(sql, args);

          // Get all rows
          rows = rawRows;
          break;
        }
      }
    } catch (err: any) {
      // Handle NOT NULL database errors
      if (err.code === PG_NOT_NULL_VIOLATION || err.errno === MYSQL_BAD_NULL_ERROR) {
        throw new DbNotNullViolationError("Violation of NOT NULL constraint", err.detail);
      }
      // Handle other database errors
      throw new DbError(`Failed to fetch rows: ${err}`);
    }

    try {
      // Validate the entire array of rows
      return objectToCamel(z.array(schema).parse(rows)) as ObjectToCamel<T>[];
    } catch (err) {
      throw new DbSchemaValidationError("Database records failed app schema validation.", rows);
    }
  }

  public async execute(
    sql: string,
    args?: any[],
    client?: PgPoolClient | MySQLPoolConnection
  ): Promise<number | null>
  {
    if (!this.dbPool || !this.type) throw new Error("Database connection has not been initialized. Call db.connect() first.");

    let rowCount: number | null;

    try {
      const executor = client || this.dbPool;
      switch (this.type) {
        case 'postgresql': {
          const result = await (executor as PgPool | PgPoolClient).query(sql, args);

          rowCount = result.rowCount;
          break;
        }
        case 'mysql': {
          const [result] = await (executor as MySQLPool | MySQLPoolConnection).query(sql, args);

          rowCount = (result as mysql.ResultSetHeader).affectedRows;
          break;
        }
      }
    } catch (err: any) {
      // Handle conflict & NOT NULL database errors
      if (err.code === PG_UNIQUE_VIOLATION || err.errno === MYSQL_DUPLICATE_ENTRY) {
        throw new DbConflictError("A record with this unique value already exists.", err.detail);
      } else if (err.code === PG_NOT_NULL_VIOLATION || err.errno === MYSQL_BAD_NULL_ERROR) {
        throw new DbNotNullViolationError("Violation of NOT NULL constraint", err.detail);
      }
      // Handle other database errors
      throw new DbError(`Failed to execute query: ${err}`);
    }

    return rowCount;
  }

  public async withTransaction<T>(
    callback: (client: PgPoolClient | MySQLPoolConnection) => Promise<T>
  ): Promise<T>
  {
    let client: PgPoolClient | MySQLPoolConnection | undefined;

    try {
      if (this.type === 'postgresql') {
        client = await (this.dbPool as PgPool).connect();
        await client.query('BEGIN');
      } else {
        client = await (this.dbPool as MySQLPool).getConnection();
        await client.query('START TRANSACTION');
      }

      const result = await callback(client);

      await this.commit(client);
      return result;
    } catch (err) {
      if (client) await this.rollback(client);
      throw err;
    } finally {
      if (client && this.type === 'postgresql') {
        (client as PgPoolClient).release();
      }
    }
  }

  private async commit(client: PgPoolClient | MySQLPoolConnection) {
    this.queryClient(client, 'COMMIT');
  }

  private async rollback(client: PgPoolClient | MySQLPoolConnection) {
    this.queryClient(client, 'ROLLBACK');
  }

  private async queryClient(client: PgPoolClient | MySQLPoolConnection, query: string) {
    if (this.type === 'postgresql') {
      await (client as PgPoolClient).query(query);
    } else {
      await (client as MySQLPoolConnection).query(query);
    }
  }
}

export const db = DatabaseComponent.instance;