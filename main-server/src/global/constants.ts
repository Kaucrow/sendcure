import fs from 'fs';
import toml from 'toml';
import yaml from 'yaml';

import { configSchema } from '@schemas/config.js';

import {
  queriesSchema
} from '@schemas/queries.js';

export const config = configSchema.parse(
  toml.parse(
    fs.readFileSync('./src/config/config.toml', 'utf-8')
  )
);

export const server = config.server;

export const frontend = {
  host: config.frontend.host,
  port: config.frontend.port,
  url: `http://${config.frontend.host}:${config.frontend.port}`
};

export const database = {
  host: process.env.DB_HOST ?? config.database.host,
  port: process.env.DB_PORT ? Number(process.env.DB_PORT) : config.database.port,
  name: process.env.DB_NAME ?? config.database.name,
  user: process.env.DB_USER ?? config.database.user,
  pass: process.env.DB_PASS ?? config.database.pass,
  url: ''
};

export const queries = queriesSchema.parse(
  yaml.parse(fs.readFileSync('./src/config/queries.yaml', 'utf-8'))
);

export const ldap = {
  url: config.ldap.url,
  baseDn: config.ldap.baseDn,
  bindDn: config.ldap.bindDn,
  bindPassword: config.ldap.bindPassword,
  userFilter: config.ldap.userFilter,
  groupBaseDn: config.ldap.groupBaseDn,
  caPath: config.ldap.caPath,
  tlsRejectUnauthorized: config.ldap.tlsRejectUnauthorized,
};