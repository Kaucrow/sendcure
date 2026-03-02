import { Router } from 'express';
import { queries } from '@global/constants.js';
import {
  sendPackageSchema,
  clientExistsSchema,
  shipmentCreatedSchema,
} from '@schemas/db/counter/index.js';
import crypto from 'crypto';
import { db, logger } from '@components/index.js';

const router = Router();

/**
 * @swagger
 * /send-pkg/send:
 *  post:
 *    tags:
 *      - counter
 *    description: Sends a package
 *    requestBody:
 *      content:
 *        application/json:
 *          schema:
 *            type: object
 *            properties:
 *              sender_cid:
 *                type: integer
 *                description: Client CI of the person sending the package.
 *              receiver_cid:
 *                type: integer
 *                description: Client CI of the destination receiver.
 *              desc:
 *                type: string
 *                nullable: true
 *                description: Optional description of the package contents.
 *              weight:
 *                type: number
 *                description: Weight of the package in kg.
 *              width:
 *                type: integer
 *                description: Width in cm.
 *              length:
 *                type: integer
 *                description: Length in cm.
 *              height:
 *                type: integer
 *                description: Height in cm.
 *              destination_address:
 *                type: string
 *                description: Full delivery address.
 *    responses:
 *      201:
 *        description: Package sent successfully.
 *        content:
 *          application/json:
 *            schema:
 *              type: object
 *              properties:
 *                message:
 *                  type: string
 *                  example: "Package sent successfully."
 *                data:
 *                  type: object
 *                  properties:
 *                    shipmentId:
 *                      type: number
 *                      example: 101
 *                    guideNumber:
 *                      type: string
 *                      example: GD-8A2B9F
 *      400:
 *        description: Invalid package data provided.
 *      404:
 *        description: Destination client not found.
 *      500:
 *        description: A server error occurred.
 */
router.post('/send-pkg/send', async (req, res) => {
  try {
    // Validate payload
    const validation = sendPackageSchema.safeParse(req.body);

    if (!validation.success) {
      return res.status(400).json({ message: 'Invalid package data provided.' });
    }

    const data = validation.data;

    // Verify destination client exists
    const receiverExists = await db.fetchOne(
      queries.counter.client.exists,
      clientExistsSchema,
      [data.receiver_cid]
    );

    if (!receiverExists || !receiverExists.exists) {
      return res.status(404).json({ 
        message: `Destination client with CI ${data.receiver_cid} not found.` 
      });
    }

    // Generate a unique 8-character guide number (e.g., "GD-8A2B9F")
    const guideNum = `GD-${crypto.randomBytes(3).toString('hex').toUpperCase()}`;

    // Insert Package and Shipment
    const newShipment = await db.fetchOne(
      queries.counter.shipment.create,
      shipmentCreatedSchema,
      [
        data.desc || null,
        data.weight,
        data.width,
        data.length,
        data.height,
        data.sender_cid,
        data.receiver_cid,
        data.destination_address,
        guideNum
      ]
    );

    return res.status(201).json(newShipment);
  } catch (err) {
    logger.error(`Error sending package: ${err}`);
    return res.status(500).json({ message: 'A server error occurred while sending the package.' });
  }
});

export default router;