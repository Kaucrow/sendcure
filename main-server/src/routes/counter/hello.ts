import { Router } from 'express';
import { queries } from '@global/constants.js';
import { testQuerySchema } from '@schemas/db/counter/index.js';
import { db, logger } from '@components/index.js';

const router = Router();

/**
 * @swagger
 * /counter/test:
 *  get:
 *    tags:
 *      - counter
 *    description: Gets a "1" from postgres
 *    responses:
 *      200:
 *        description: Postgres 1
 *        content:
 *          application/json:
 *            schema:
 *                type: object
 *                properties:
 *                   value:
 *                       type: number
 *                       description: Postgres 1
 *                       example: 1
 *      500:
 *        description: Test failed.
 */
router.get('/test', async (req, res) => {
  try {
    const testResult = await db.fetchOne(
      queries.counter.testQuery,
      testQuerySchema,
    );

    return res.status(200).json(testResult);
  } catch (err) {
    logger.error(`Error getting user menus: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

export default router;