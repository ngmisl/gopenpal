import { createTool } from "@voltagent/core";
import {
  createTaskInputSchema,
  updateTaskInputSchema,
  listTasksInputSchema,
} from "../schemas/task";
import { db } from "../lib/db";
import { z } from "zod";

export const createTaskTool = createTool({
  name: "create_task",
  description: "Create a new task with title, description, priority, and optional due date",
  parameters: createTaskInputSchema,
  execute: async ({ title, description, priority, due_date }) => {
    const task = await db.createTask(title, description, priority, due_date);
    return {
      success: true,
      task,
      message: `Created task: ${title}`,
    };
  },
});

export const listTasksTool = createTool({
  name: "list_tasks",
  description: "List tasks with optional filters for status and priority",
  parameters: listTasksInputSchema,
  execute: async ({ status, priority, limit }) => {
    const tasks = await db.listTasks(status, priority, limit);
    return {
      total: tasks.length,
      tasks,
    };
  },
});

export const updateTaskTool = createTool({
  name: "update_task",
  description: "Update an existing task's properties",
  parameters: updateTaskInputSchema,
  execute: async ({ task_id, ...updates }) => {
    await db.updateTask(task_id, updates);
    return {
      success: true,
      task_id,
      message: `Updated task ${task_id}`,
    };
  },
});

export const completeTaskTool = createTool({
  name: "complete_task",
  description: "Mark a task as completed",
  parameters: z.object({
    task_id: z.string().uuid().describe("Task ID to complete"),
  }),
  execute: async ({ task_id }) => {
    await db.updateTask(task_id, {
      status: "completed",
      completed_at: new Date().toISOString(),
    });
    return {
      success: true,
      task_id,
      message: `Task ${task_id} marked as completed`,
    };
  },
});
