import { Router } from 'express';
import { z } from 'zod';
import { queries } from '@global/constants.js';
import { db, logger } from '@components/index.js';

const router = Router();

// Flexible para no romper por columnas/tipos extra
const questionSchema = z.object({}).passthrough();

/**
 * @swagger
 * /employee/questions:
 *  get:
 *    tags:
 *      - employee
 *    description: Get all questions.
 *    responses:
 *      200:
 *        description: List of questions.
 *      500:
 *        description: A server error occurred.
 */
router.get('/questions', async (_req, res) => {
  try {
    const questions = await db.fetch(
      queries.employee.getAllQuestions,
      questionSchema
    );

    return res.status(200).json(questions);
  } catch (err) {
    logger.error(`Error fetching questions: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

export default router;