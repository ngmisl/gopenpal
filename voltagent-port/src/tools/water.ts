import { createTool } from "@voltagent/core";
import {
  waterLogInputSchema,
  waterHistoryInputSchema,
  waterGoalInputSchema,
} from "../schemas/water";
import { db } from "../lib/db";

export const logWaterTool = createTool({
  name: "log_water",
  description: "Log water intake for tracking hydration",
  parameters: waterLogInputSchema,
  execute: async ({ amount_ml, notes }) => {
    const entry = await db.logWater(amount_ml, notes);
    return {
      success: true,
      entry,
      message: `Logged ${amount_ml}ml of water`,
    };
  },
});

export const getWaterStatsTool = createTool({
  name: "get_water_stats",
  description: "Get current water intake statistics for today",
  parameters: {},
  execute: async () => {
    const stats = await db.getWaterStats();
    return stats;
  },
});

export const getWaterHistoryTool = createTool({
  name: "get_water_history",
  description: "Get water intake history for the past N days",
  parameters: waterHistoryInputSchema,
  execute: async ({ days }) => {
    const history = await db.getWaterHistory(days);
    return {
      days,
      total_entries: history.length,
      history,
    };
  },
});

export const setWaterGoalTool = createTool({
  name: "set_water_goal",
  description: "Set the daily water intake goal in milliliters",
  parameters: waterGoalInputSchema,
  execute: async ({ goal_ml }) => {
    await db.setWaterGoal(goal_ml);
    return {
      success: true,
      goal_ml,
      message: `Daily water goal set to ${goal_ml}ml`,
    };
  },
});
