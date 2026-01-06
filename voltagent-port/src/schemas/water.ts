import { z } from "zod";

// Water intake schemas
export const waterIntakeSchema = z.object({
  id: z.string().uuid(),
  timestamp: z.string().datetime(),
  amount_ml: z.number().positive().int(),
  notes: z.string().optional(),
});

export const waterLogInputSchema = z.object({
  amount_ml: z.number().positive().int().describe("Amount of water in milliliters"),
  notes: z.string().optional().describe("Optional notes about the water intake"),
});

export const waterStatsSchema = z.object({
  today_ml: z.number(),
  goal_ml: z.number(),
  percentage: z.number(),
  entries_today: z.number(),
  last_log: z.string().datetime().nullable(),
});

export const waterHistoryInputSchema = z.object({
  days: z.number().min(1).max(90).default(7).describe("Number of days to retrieve"),
});

export const waterGoalInputSchema = z.object({
  goal_ml: z.number().positive().int().describe("Daily water goal in milliliters"),
});

export type WaterIntake = z.infer<typeof waterIntakeSchema>;
export type WaterLogInput = z.infer<typeof waterLogInputSchema>;
export type WaterStats = z.infer<typeof waterStatsSchema>;
export type WaterHistoryInput = z.infer<typeof waterHistoryInputSchema>;
export type WaterGoalInput = z.infer<typeof waterGoalInputSchema>;
