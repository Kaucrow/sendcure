import { Router } from 'express';
import { queries } from '@global/constants.js';
import {
    shipmentSchema,
    shipmentTrackingSchema,
} from '@schemas/db/shipment/index.js';
import { db, logger } from '@components/index.js';

const router = Router();

/**
 * @swagger
 * /shipment:
 *  get:
 *    tags:
 *      - shipment 
 *    description: Get a list of shipments, optionally filtered by status.
 *    parameters:
 *      - in: query
 *        name: status_id
 *        schema:
 *          type: integer
 *        description: Filter shipments by their status ID
 *    responses:
 *      200:
 *        description: A list of shipments.
 *      500:
 *        description: Server error.
 */
router.get('/', async (req, res) => {
    try {
        const { status_id } = req.query;

        let rows;
        if (status_id) {
            rows = await db.fetch(
                queries.counter.shipment.getStatus,
                shipmentSchema,
                [Number(status_id)]
            );
        } else {
           
            return res.status(400).json({ message: 'status_id query parameter is required.' });
        }

        return res.status(200).json(rows);

    } catch (err) {
        logger.error(`Error fetching shipments: ${err}`);
        return res.status(500).json({ message: 'A server error occurred.' });
    }
});

/**
 * @swagger
 * /shipment/{guide_num}:
 *  get:
 *    tags:
 *      - shipment
 *    description: Get tracking information for one shipment by package/guide code.
 *    parameters:
 *      - in: path
 *        name: guide_num
 *        required: true
 *        schema:
 *          type: string
 *        description: Unique package guide number (for example, GD-789001)
 *    responses:
 *      200:
 *        description: Tracking information for the requested shipment.
 *      400:
 *        description: Invalid guide number.
 *      404:
 *        description: Shipment not found.
 *      500:
 *        description: Server error.
 */
router.get('/:guide_num', async (req, res) => {
    try {
        const { guide_num } = req.params;

        if (!guide_num || guide_num.trim().length === 0) {
            return res.status(400).json({ message: 'guide_num is required.' });
        }

        const shipment = await db.fetchOne(
            queries.counter.shipment.getTrackingByGuideNum,
            shipmentTrackingSchema,
            [guide_num.trim()]
        );

        if (!shipment) {
            return res.status(404).json({ message: 'Shipment not found.' });
        }

        return res.status(200).json(shipment);
    } catch (err) {
        logger.error(`Error fetching tracking for ${req.params.guide_num}: ${err}`);
        return res.status(500).json({ message: 'A server error occurred.' });
    }
});

export default router;
