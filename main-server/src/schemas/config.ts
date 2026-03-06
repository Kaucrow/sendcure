import { z } from 'zod';

export const configSchema = z.object({
  server: z.object({
    host: z.string(),
    port: z.number()
  }),
  frontend: z.object({
    host: z.string(),
    port: z.number()
  }),
  database: z.object({
    type: z.enum(["postgresql", "mysql"]),
    host: z.string(),
    port: z.number(),
    name: z.string(),
    user: z.string(),
    pass: z.string()
  }),
  ldap: z.object({
    url: z.string(),
    baseDn: z.string(),
    bindDn: z.string().optional(),
    bindPassword: z.string().optional(),
    userFilter: z.string(),
    groupBaseDn: z.string().optional(),
    caPath: z.string().optional(),
    tlsRejectUnauthorized: z.boolean().optional()
  })
});