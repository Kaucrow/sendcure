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
  host: config.database.host,
  port: config.database.port,
  name: config.database.name,
  user: config.database.user,
  pass: config.database.pass,
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