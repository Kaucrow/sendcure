import { z } from 'zod';

export const shipmentSchema = z.object({
    shipmentId: z.number().int(),
    guideNum: z.string(),
    clientCid: z.number().int().nullable().optional(),
    receiverCid: z.number().int().nullable().optional(),
    packageId: z.number().int().nullable().optional(),
    deliveryId: z.number().int().nullable().optional(),
    statusId: z.number().int().nullable().optional(),
    destinationBranch: z.number().int().nullable().optional(),
    destinationAddress: z.string(),
    shipmentDt: z.date().optional(),
});

export type Shipment = z.infer<typeof shipmentSchema>;
