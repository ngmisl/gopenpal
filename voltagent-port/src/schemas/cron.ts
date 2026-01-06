import { z } from "zod";

// Cron/reminder schemas
export const cronJobSchema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  description: z.string().optional(),
  schedule: z.string().describe("Cron expression (e.g., '0 */2 * * *')"),
  enabled: z.boolean(),
  last_run: z.string().datetime().nullable(),
  next_run: z.string().datetime().nullable(),
  action: z.enum(["water_reminder", "task_reminder", "custom"]),
  metadata: z.record(z.unknown()).optional(),
});

export const createCronInputSchema = z.object({
  name: z.string().min(1).describe("Cron job name"),
  description: z.string().optional().describe("Cron job description"),
  schedule: z.string().describe("Cron expression"),
  action: z.enum(["water_reminder", "task_reminder", "custom"]).describe("Action to perform"),
  metadata: z.record(z.unknown()).optional().describe("Additional metadata"),
});

export const updateCronInputSchema = z.object({
  cron_id: z.string().uuid().describe("Cron job ID"),
  name: z.string().optional().describe("New name"),
  description: z.string().optional().describe("New description"),
  schedule: z.string().optional().describe("New schedule"),
  enabled: z.boolean().optional().describe("Enable or disable"),
});

export type CronJob = z.infer<typeof cronJobSchema>;
export type CreateCronInput = z.infer<typeof createCronInputSchema>;
export type UpdateCronInput = z.infer<typeof updateCronInputSchema>;
