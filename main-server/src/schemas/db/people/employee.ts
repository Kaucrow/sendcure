import { z } from 'zod';

export const employeeSchema = z.object({
  ci: z.number(),
  email: z.string(),
  name: z.string(),
  passwd: z.string(),
  phone_num: z.string()
});

export const publicEmployeeSchema = employeeSchema
  .omit({ passwd: true })
  .omit({ phone_num: true })
  .extend({
    phoneNum: z.string(),
    role: z.string()
  });