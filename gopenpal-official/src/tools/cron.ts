import { createTool } from "@voltagent/core";
import { createCronInputSchema, updateCronInputSchema } from "../schemas/cron";
import { db } from "../lib/db";
import { z } from "zod";

export const createCronJobTool = createTool({
  name: "create_cron_job",
  description:
    "Create a new cron job for scheduled reminders (e.g., water reminders every 2 hours)",
  parameters: createCronInputSchema,
  execute: async ({ name, description, schedule, action, metadata }) => {
    const job = await db.createCronJob(name, schedule, action, description, metadata);
    return {
      success: true,
      job,
      message: `Created cron job: ${name}`,
    };
  },
});

export const listCronJobsTool = createTool({
  name: "list_cron_jobs",
  description: "List all scheduled cron jobs",
  parameters: {},
  execute: async () => {
    const jobs = await db.listCronJobs();
    return {
      total: jobs.length,
      jobs,
    };
  },
});

export const updateCronJobTool = createTool({
  name: "update_cron_job",
  description: "Update a cron job's schedule or enable/disable it",
  parameters: updateCronInputSchema,
  execute: async ({ cron_id, ...updates }) => {
    await db.updateCronJob(cron_id, updates);
    return {
      success: true,
      cron_id,
      message: `Updated cron job ${cron_id}`,
    };
  },
});

export const deleteCronJobTool = createTool({
  name: "delete_cron_job",
  description: "Delete a cron job",
  parameters: z.object({
    cron_id: z.string().uuid().describe("Cron job ID to delete"),
  }),
  execute: async ({ cron_id }) => {
    // For now, we'll disable it since we don't have a delete method
    await db.updateCronJob(cron_id, { enabled: false });
    return {
      success: true,
      cron_id,
      message: `Disabled cron job ${cron_id}`,
    };
  },
});
