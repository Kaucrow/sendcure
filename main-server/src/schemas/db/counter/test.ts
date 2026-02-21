import { z } from 'zod';

export const testQuerySchema = z.object({
  value: z.number(),
});