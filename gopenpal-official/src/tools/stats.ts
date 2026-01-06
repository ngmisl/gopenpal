import { createTool } from "@voltagent/core";
import { statsInputSchema } from "../schemas/stats";
import { db } from "../lib/db";

export const getWaterAnalyticsTool = createTool({
  name: "get_water_analytics",
  description: "Get detailed water intake analytics for a time range",
  parameters: statsInputSchema,
  execute: async ({ range }) => {
    const days = range === "today" ? 1 : range === "week" ? 7 : range === "month" ? 30 : 365;

    const history = await db.getWaterHistory(days);
    const stats = await db.getWaterStats();

    // Group by date
    const dailyMap = new Map<string, number>();
    for (const entry of history) {
      const date = entry.timestamp.split("T")[0];
      dailyMap.set(date, (dailyMap.get(date) || 0) + entry.amount_ml);
    }

    const daily_breakdown = Array.from(dailyMap.entries()).map(([date, amount_ml]) => ({
      date,
      amount_ml,
      percentage_of_goal: (amount_ml / stats.goal_ml) * 100,
    }));

    const total_ml = history.reduce((sum, entry) => sum + entry.amount_ml, 0);
    const average_per_day = daily_breakdown.length > 0 ? total_ml / daily_breakdown.length : 0;
    const goal_achievement_rate =
      daily_breakdown.length > 0
        ? (daily_breakdown.filter((d) => d.percentage_of_goal >= 100).length /
            daily_breakdown.length) *
          100
        : 0;

    return {
      range,
      total_ml,
      average_per_day,
      entries_count: history.length,
      goal_achievement_rate,
      daily_breakdown,
    };
  },
});

export const getTaskAnalyticsTool = createTool({
  name: "get_task_analytics",
  description: "Get detailed task analytics for a time range",
  parameters: statsInputSchema,
  execute: async ({ range }) => {
    const limit = range === "today" ? 50 : range === "week" ? 200 : range === "month" ? 500 : 1000;

    const tasks = await db.listTasks(undefined, undefined, limit);

    const total_tasks = tasks.length;
    const completed_tasks = tasks.filter((t) => t.status === "completed").length;
    const pending_tasks = tasks.filter((t) => t.status === "pending").length;
    const in_progress_tasks = tasks.filter((t) => t.status === "in_progress").length;
    const completion_rate = total_tasks > 0 ? (completed_tasks / total_tasks) * 100 : 0;

    const by_priority = {
      urgent: tasks.filter((t) => t.priority === "urgent").length,
      high: tasks.filter((t) => t.priority === "high").length,
      medium: tasks.filter((t) => t.priority === "medium").length,
      low: tasks.filter((t) => t.priority === "low").length,
    };

    return {
      range,
      total_tasks,
      completed_tasks,
      pending_tasks,
      in_progress_tasks,
      completion_rate,
      by_priority,
    };
  },
});
