import { z } from 'zod';

export const shipmentStatusSchema = z.object({
  status_id: z.number().int().positive(),
});

export const sendPackageSchema = z.object({
  sender_cid: z.number().int().positive(),
  receiver_cid: z.number().int().positive(),
  desc: z.string().nullable().optional(),
  weight: z.number().positive(),
  width: z.number().int().positive(),
  length: z.number().int().positive(),
  height: z.number().int().positive(),
  destination_address: z.string().min(5, "Address is too short")
});

export const shipmentCreatedSchema = z.object({
  shipment_id: z.number().int().positive(),
  guide_num: z.string(),
});