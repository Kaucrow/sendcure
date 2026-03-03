import { Router } from 'express';
import { queries } from '@global/constants.js';
import {
  receivedPackageSchema,
  clientExistsSchema,
  shipmentStatusSchema,
} from '@schemas/db/counter/index.js';
import { db, logger } from '@components/index.js';

const router = Router();

/**
 * @swagger
 * /recv-pkg/received/{client_cid}:
 *  get:
 *    tags:
 *      - counter
 *    description: Retrieves all packages for a specific client (receiver) that have a shipping status of 2.
 *    parameters:
 *      - in: path
 *        name: client_cid
 *        required: true
 *        schema:
 *          type: integer
 *          description: The client identification number (C.I.) acting as the receiver.
 *    responses:
 *      200:
 *        description: A list of received packages.
 *        content:
 *          application/json:
 *            schema:
 *              type: array
 *              items:
 *                type: object
 *                properties:
 *                  id:
 *                    type: integer
 *                    example: 101
 *                  desc:
 *                    type: string
 *                    example: "Electronics parts"
 *                  weight:
 *                    type: string
 *                    example: "2.50"
 *                  width:
 *                    type: integer
 *                    example: 10
 *                  length:
 *                    type: integer
 *                    example: 15
 *                  height:
 *                    type: integer
 *                    example: 5
 *                  guide_num:
 *                    type: string
 *                    example: "GD-123456789"
 *                  shipment_dt:
 *                    type: string
 *                    format: date-time
 *                    example: "2026-02-27T15:30:00Z"
 *                  destination_address:
 *                    type: string
 *                    example: "67 St., Zimbabwe"
 *      400:
 *        description: Invalid client ID parameter.
 *      404:
 *        description: Client not found.
 *      500:
 *        description: A server error occurred.
 */
router.get('/recv-pkg/received/:client_cid', async (req, res) => {
  try {
    const { client_cid } = req.params;

    // Enusure client_cid is a number
    if (!client_cid || isNaN(parseInt(client_cid, 10))) {
      return res.status(400).json({ message: 'Invalid client ID provided.' });
    }

    // Check if client exists in the database
    const clientExists = await db.fetchOne(
      queries.counter.client.exists,
      clientExistsSchema,
      [client_cid]
    );

    if (!clientExists || !clientExists.exists) {
      return res.status(404).json({ message: `Client with CI ${client_cid} not found.` });
    }

    // Fetch received packages
    const packages = await db.fetch(
      queries.counter.package.getReceived,
      receivedPackageSchema,
      [parseInt(client_cid, 10)],
    );

    return res.status(200).json(packages);
  } catch (err) {
    logger.error(`Error getting received packages for client ${req.params.client_cid}: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

/**
 * @swagger
 * /recv-pkg/pickup/{guide_num}:
 *  put:
 *    tags:
 *      - counter
 *    description: Updates a shipment status to "Picked Up" (ID 3) using its guide number.
 *    parameters:
 *      - in: path
 *        name: guide_num
 *        required: true
 *        description: The unique tracking/guide number of the package.
 *        schema:
 *          type: string
 *    responses:
 *      200:
 *        description: Shipment successfully updated to Picked Up.
 *      404:
 *        description: Guide number not found.
 *      500:
 *        description: Server error.
 */
router.put('/recv-pkg/pickup/:guide_num', async (req, res) => {
  try {
    const { guide_num } = req.params;

    const shipmentStatus = await db.fetchOne(
      queries.counter.shipment.getStatus,
      shipmentStatusSchema,
      [guide_num]
    );

    if (!shipmentStatus) {
      return res.status(404).json({ message: 'Shipment not found.' });
    }

    if (shipmentStatus.statusId === 3) {
      return res.status(400).json({ message: 'This package has already been picked up.' });
    }

    if (shipmentStatus.statusId === 0 || shipmentStatus.statusId === 1) {
      return res.status(400).json({ message: 'Package is not yet available for pickup (must be in status 2).' });
    }

    // Proceed with update
    await db.execute(
      queries.counter.shipment.updateToPickedUp,
      [guide_num]
    );

    return res.status(200).json({ message: 'Success: Package picked up.' });

  } catch (err) {
    logger.error(`Pickup error for ${req.params.guide_num}: ${err}`);
    return res.status(500).json({ message: 'Internal server error.' });
  }
});

export default router;