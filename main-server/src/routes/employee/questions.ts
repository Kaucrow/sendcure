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
 * /employee/questions/answered:
 *  get:
 *    tags:
 *      - employee
 *    description: Get all answered questions.
 *    responses:
 *      200:
 *        description: List of answered questions.
 *      500:
 *        description: A server error occurred.
 */
router.get('/questions/answered', async (_req, res) => {
  try {
    const answeredQuestions = await db.fetch(
      queries.employee.getAnsweredQuestions,
      questionSchema
    );

    return res.status(200).json(answeredQuestions);
  } catch (err) {
    logger.error(`Error fetching answered questions: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

/**
 * @swagger
 * /employee/questions:
 *  post:
 *    tags:
 *      - employee
 *    description: Create a support question from a client.
 *    requestBody:
 *      required: true
 *      content:
 *        application/json:
 *          schema:
 *            type: object
 *            properties:
 *              client_cid:
 *                type: integer
 *              question_text:
 *                type: string
 *    responses:
 *      201:
 *        description: Question created.
 *      400:
 *        description: Invalid request data.
 *      500:
 *        description: A server error occurred.
 */
router.post('/questions', async (req, res) => {
  try {
    const payloadSchema = z.object({
      client_cid: z.coerce.number().int().positive(),
      question_text: z.string().trim().min(1)
    });

    const parsedPayload = payloadSchema.safeParse(req.body);
    if (!parsedPayload.success) {
      return res.status(400).json({ message: 'client_cid and question_text are required.' });
    }

    const createdQuestion = await db.fetchOne(
      queries.employee.createQuestion,
      questionSchema,
      [parsedPayload.data.client_cid, parsedPayload.data.question_text]
    );

    return res.status(201).json(createdQuestion);
  } catch (err) {
    logger.error(`Error creating question: ${err}`);
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