import type { Express } from 'express';
import winston, { Logger } from 'winston';
import morgan, { type StreamOptions } from 'morgan';

class LoggerComponent {
  static #instance: LoggerComponent;

  private logger: Logger | undefined = undefined;

  private constructor() {}

  public static get instance(): LoggerComponent {
    if (!LoggerComponent.#instance) {
      LoggerComponent.#instance = new LoggerComponent();
    }
    return LoggerComponent.#instance;
  }

  public async init(app: Express) {
    /* --- Setup Winston --- */
    const levels = {
      error: 0,
      warn: 1,
      info: 2,
      http: 3,
      debug: 4,
    };

    const level = () => {
      const env = process.env.NODE_ENV || 'development';
      const isDev = env === 'development';
      return isDev ? 'debug' : 'warn';
    };

    const colors = {
      error: 'red',
      warn: 'yellow',
      info: 'green',
      http: 'magenta',
      debug: 'white'
    };

    winston.addColors(colors);

    const consoleFormat = winston.format.combine(
      winston.format.timestamp({ format: 'DD-MM-YYYY HH:mm:ss:ms' }),
      winston.format.colorize({ all: true }),
      winston.format.printf(
        (info) => `${info.timestamp} ${info.level}: ${info.message}`,
      ),
    );

    const fileFormat = winston.format.combine(
      winston.format.timestamp({ format: 'DD-MM-YYYY HH:mm:ss:ms' }),
      winston.format.printf(
        (info) => `${info.timestamp} ${info.level}: ${info.message}`,
      ),
    );

    const transports = [
      new winston.transports.Console({
        format: consoleFormat,
      }),
      new winston.transports.File({
        filename: 'logs/error.log',
        level: 'error',
        format: fileFormat,
        options: { flags: 'w' }
      }),
      new winston.transports.File({
        filename: 'logs/app.log',
        format: fileFormat,
        options: { flags: 'w' }
      }),
    ];

    this.logger = winston.createLogger({
      level: level(),
      levels,
      transports
    });

    /* Setup Morgan */
    const stream: StreamOptions = {
      write: (msg: string) => this.logger!.http(msg)
    };

    // This fn determines if we should skip Morgan HTTP log
    const skip = () => {
      return false; 
    };

    const morganMiddleware = morgan(
      ":method :url :status :res[content-length - :response-time ms]",
      { stream, skip }
    );

    app.use(morganMiddleware);
  }

  public error(msg: string, meta?: Record<string, any>): void {
    if (!this.logger) throw new Error('Logger has not been initialized. Call logger.init() first.');
    this.logger.error(msg, meta);
  }
 
  public warn(msg: string, meta?: Record<string, any>): void {
    if (!this.logger) throw new Error('Logger has not been initialized. Call logger.init() first.');
    this.logger.warn(msg, meta);
  }
  
  public info(msg: string, meta?: Record<string, any>): void {
    if (!this.logger) throw new Error('Logger has not been initialized. Call logger.init() first.');
    this.logger.info(msg, meta);
  }
  
  public http(msg: string, meta?: Record<string, any>): void {
    if (!this.logger) throw new Error('Logger has not been initialized. Call logger.init() first.');
    this.logger.http(msg, meta);
  }
  
  public debug(msg: string, meta?: Record<string, any>): void {
    if (!this.logger) throw new Error('Logger has not been initialized. Call logger.init() first.');
    this.logger.debug(msg, meta);
  }
};

export const logger = LoggerComponent.instance;