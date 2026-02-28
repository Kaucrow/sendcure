import { Router } from 'express';
import { publicEmployeeSchema } from '@schemas/db/people/index.js';
import { logger } from '@components/index.js';
import { ldap } from '@components/ldap.js';

const router = Router();
const ROLE_GROUPS = new Set(['counter', 'dispatch', 'customer-service', 'admins']);

const asString = (value: unknown): string => {
  if (Array.isArray(value)) {
    const first = value[0];
    return first == null ? '' : String(first);
  }
  return value == null ? '' : String(value);
};

/**
 * @swagger
 * /employee/login:
 *  post:
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
router.post('/login', async (req, res) => {
  try {
    const { ci, passwd } = req.body;

    logger.info(req.body);

    if (!ci || !passwd) {
      return res.status(400).json({ message: 'CI and password are required.' });
    }

    const requestedRole = asString(req.body?.role ?? req.query?.role ?? 'counter').toLowerCase();
    if (!ROLE_GROUPS.has(requestedRole)) {
      return res.status(400).json({ message: 'Invalid requested role.' });
    }

    const ldapUser = await ldap.authenticate({ 
      uid: String(ci),
      password: passwd
    });
    if (!ldapUser) {
      return res.status(401).json({ message: 'Invalid CI or password.' });
    }

    const role = asString(ldapUser.role).toLowerCase();
    if (!role) {
      return res.status(403).json({ message: 'User has no assigned role.' });
    }
    if (role !== requestedRole) {
      return res.status(403).json({ message: `User is not authorized for role: ${requestedRole}.` });
    }


    const safeEmployee = publicEmployeeSchema.parse({
      ci: Number(asString(ldapUser.uidNumber) || ci),
      email: asString(ldapUser.mail),
      name: asString(ldapUser.cn),
      phoneNum: asString(ldapUser.telephoneNumber),
      role
    });

    return res.status(200).json(safeEmployee);

  } catch (err) {
    logger.error(`Error during login: ${err}`);
    return res.status(500).json({ message: 'A server error occurred.' });
  }
});

export default router;