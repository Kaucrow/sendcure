import { z } from 'zod';

export const queriesSchema = z.object({
  employee: z.object({
    getByCredentials: z.string(),
    getAllQuestions: z.string(),
    getAnsweredQuestions: z.string(),
    createQuestion: z.string(),
    updateQuestionResponse: z.string(),
  }),

  counter: z.object({
    testQuery: z.string(),
    client: z.object({
      exists: z.string(),
    }),
    package: z.object({
      getReceived: z.string(),
    }),
    shipment: z.object({
      getStatus: z.string(),
      getTrackingByGuideNum: z.string(),
      updateToPickedUp: z.string(),
      create: z.string(),
    })
  }),
});