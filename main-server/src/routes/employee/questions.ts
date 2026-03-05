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

/**
 * @swagger
 * /employee/questions/{question_id}/response:
 *  put:
 *    tags:
 *      - employee
 *    description: Update response for a question.
 *    parameters:
 *      - in: path
 *        name: question_id
 *        required: true
 *        schema:
 *          type: integer
 *    requestBody:
 *      required: true
 *      content:
 *        application/json:
 *          schema:
 *            type: object
 *            properties:
 *              response:
 *                type: string
 *    responses:
 *      200:
 *        description: Question answered.
 *      400:
 *        description: Invalid request data.
 *      404:
 *        description: Question not found.
 *      500:
 *        description: A server error occurred.
 */
router.put('/questions/:question_id/response', async (req, res) => {
  try {
    const { question_id } = req.params;
    const payloadSchema = z.object({ response: z.string().trim().min(1) });

    if (!question_id || Number.isNaN(Number(question_id))) {
      return res.status(400).json({ message: 'Invalid question ID provided.' });
    }

    const parsedPayload = payloadSchema.safeParse(req.body);
    if (!parsedPayload.success) {
      return res.status(400).json({ message: 'Response is required.' });
    }

    const rowCount = await db.execute(
      queries.employee.updateQuestionResponse,
      [parsedPayload.data.response, Number(question_id)]
    );

    if (!rowCount) {
      return res.status(404).json({ message: `Question with ID ${question_id} not found.` });
    }

    return res.status(200).json({ message: 'Question answered successfully.' });
  } catch (err) {
    logger.error(`Error updating question response: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

export default router;