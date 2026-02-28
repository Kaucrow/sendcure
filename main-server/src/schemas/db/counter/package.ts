import { z } from 'zod';

export const receivedPackageSchema = z.object({
  id: z.number(),
  desc: z.string().nullable(),
  weight: z.string().nullable(),
  width: z.number().nullable(),
  length: z.number().nullable(),
  height: z.number().nullable(),
  guide_num: z.string(),
  shipment_dt: z.date(), 
  destination_address: z.string()
});