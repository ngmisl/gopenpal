import { z } from "zod";

// Task schemas
export const taskPrioritySchema = z.enum(["low", "medium", "high", "urgent"]);

export const taskStatusSchema = z.enum(["pending", "in_progress", "completed", "cancelled"]);

export const taskSchema = z.object({
  id: z.string().uuid(),
  title: z.string(),
  description: z.string().optional(),
  priority: taskPrioritySchema,
  status: taskStatusSchema,
  created_at: z.string().datetime(),
  updated_at: z.string().datetime(),
  due_date: z.string().datetime().nullable(),
  completed_at: z.string().datetime().nullable(),
});

export const createTaskInputSchema = z.object({
  title: z.string().min(1).describe("Task title"),
  description: z.string().optional().describe("Task description"),
  priority: taskPrioritySchema.default("medium").describe("Task priority level"),
  due_date: z.string().datetime().optional().describe("Task due date"),
});

export const updateTaskInputSchema = z.object({
  task_id: z.string().uuid().describe("Task ID to update"),
  title: z.string().min(1).optional().describe("New task title"),
  description: z.string().optional().describe("New task description"),
  priority: taskPrioritySchema.optional().describe("New task priority"),
  status: taskStatusSchema.optional().describe("New task status"),
  due_date: z.string().datetime().nullable().optional().describe("New due date"),
});

export const listTasksInputSchema = z.object({
  status: taskStatusSchema.optional().describe("Filter by status"),
  priority: taskPrioritySchema.optional().describe("Filter by priority"),
  limit: z.number().min(1).max(100).default(50).describe("Maximum number of tasks"),
});

export type Task = z.infer<typeof taskSchema>;
export type TaskPriority = z.infer<typeof taskPrioritySchema>;
export type TaskStatus = z.infer<typeof taskStatusSchema>;
export type CreateTaskInput = z.infer<typeof createTaskInputSchema>;
export type UpdateTaskInput = z.infer<typeof updateTaskInputSchema>;
export type ListTasksInput = z.infer<typeof listTasksInputSchema>;
