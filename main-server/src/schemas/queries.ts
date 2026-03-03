import { z } from 'zod';

export const queriesSchema = z.object({
  employee: z.object({
    getByCredentials: z.string(),
  }),

  counter: z.object({
    testQuery: z.string(),
    client: z.object({
      exists: z.string(),
    }),
    package: z.object({
      getReceived: z.string(),
    }),
    shipment: z.object({
      getStatus: z.string(),
      updateToPickedUp: z.string(),
      create: z.string(),
    })
  }),
});