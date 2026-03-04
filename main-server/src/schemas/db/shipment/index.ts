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

export const shipmentTrackingSchema = z.object({
    guide_num: z.string(),
    status_id: z.number().int(),
    status_desc: z.string(),
    destination_address: z.string(),
    shipment_dt: z.coerce.date(),
    package_desc: z.string().nullable(),
});

export type Shipment = z.infer<typeof shipmentSchema>;
export type ShipmentTracking = z.infer<typeof shipmentTrackingSchema>;
