import { Router } from 'express';
import { queries } from '@global/constants.js';
import { shipmentSchema } from '@schemas/db/shipment/index.js';
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
                queries.shipment.getByStatus,
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

export default router;
