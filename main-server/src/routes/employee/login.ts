import { Router } from 'express';
import argon2 from 'argon2';
import { queries } from '@global/constants.js';
import { employeeSchema, publicEmployeeSchema } from '@schemas/db/people/index.js';
import { db, logger } from '@components/index.js';

const router = Router();

/**
 * @swagger
 * /employee/login:
 *  get:
 *    tags:
 *      - auth 
 *    description: Login endpoint.
 *    responses:
 *      200:
 *        description: User data
 *        content:
 *          application/json:
 *            schema:
 *              type: object
 *              properties:
 *                ci:
 *                  type: number
 *                  example: 9876543
 *                email:
 *                  type: string
 *                  example: 'elatla@elatla.com'
 *                name:
 *                  type: string
 *                  example: 'El Atla'
 *                phone_num:
 *                  type: string
 *                  example: '000-111-222'
 *      401:
 *        description: Invalid credentials.
 */
router.get('/login', async (req, res) => {
  try {
    const { ci, passwd } = req.body;

    logger.info(req.body);

    if (!ci || !passwd) {
      return res.status(400).json({ message: 'CI and password are required.' });
    }

    const employee = await db.fetchOne(
      queries.employee.getByCredentials,
      employeeSchema,
      [ci],
    );

    if (!employee || !argon2.verify(employee.passwd, passwd)) {
      return res.status(401).json({ message: 'Invalid CI or password.' });
    }

    const role = 'counter';

    const employeeWithRole = {
      ...employee,
      role,
    };

    const safeEmployee = publicEmployeeSchema.parse(employeeWithRole);

    return res.status(200).json(safeEmployee);

  } catch (err) {
    logger.error(`Error during login: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

export default router;