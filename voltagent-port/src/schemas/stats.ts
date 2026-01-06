import { z } from "zod";

// Analytics and stats schemas
export const timeRangeSchema = z.enum(["today", "week", "month", "all"]);

export const statsInputSchema = z.object({
  range: timeRangeSchema.default("today").describe("Time range for statistics"),
});

export const waterAnalyticsSchema = z.object({
  range: timeRangeSchema,
  total_ml: z.number(),
  average_per_day: z.number(),
  entries_count: z.number(),
  goal_achievement_rate: z.number(),
  daily_breakdown: z.array(
    z.object({
      date: z.string(),
      amount_ml: z.number(),
      percentage_of_goal: z.number(),
    })
  ),
});

export const taskAnalyticsSchema = z.object({
  range: timeRangeSchema,
  total_tasks: z.number(),
  completed_tasks: z.number(),
  pending_tasks: z.number(),
  in_progress_tasks: z.number(),
  completion_rate: z.number(),
  by_priority: z.object({
    urgent: z.number(),
    high: z.number(),
    medium: z.number(),
    low: z.number(),
  }),
});

export type TimeRange = z.infer<typeof timeRangeSchema>;
export type StatsInput = z.infer<typeof statsInputSchema>;
export type WaterAnalytics = z.infer<typeof waterAnalyticsSchema>;
export type TaskAnalytics = z.infer<typeof taskAnalyticsSchema>;
