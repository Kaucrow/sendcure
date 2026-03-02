import { z } from 'zod';

export const clientExistsSchema = z.object({
  exists: z.boolean()
});