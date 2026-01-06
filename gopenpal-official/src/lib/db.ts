import { createClient } from "@libsql/client";
import type { WaterIntake, WaterStats } from "../schemas/water";
import type { Task } from "../schemas/task";
import type { CronJob } from "../schemas/cron";
import type { AgentState } from "../schemas/agent";

export class Database {
  private client;

  constructor(url: string = "file:./.voltagent/gopenpal.db") {
    this.client = createClient({ url });
  }

  async initialize() {
    // Create tables
    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS water_intake (
        id TEXT PRIMARY KEY,
        timestamp TEXT NOT NULL,
        amount_ml INTEGER NOT NULL,
        notes TEXT
      )
    `);

    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS water_goals (
        id INTEGER PRIMARY KEY,
        goal_ml INTEGER NOT NULL,
        date TEXT NOT NULL UNIQUE
      )
    `);

    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS tasks (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        description TEXT,
        priority TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL,
        due_date TEXT,
        completed_at TEXT
      )
    `);

    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS cron_jobs (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        schedule TEXT NOT NULL,
        enabled INTEGER NOT NULL DEFAULT 1,
        last_run TEXT,
        next_run TEXT,
        action TEXT NOT NULL,
        metadata TEXT
      )
    `);

    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS agents (
        name TEXT PRIMARY KEY,
        current_mood TEXT NOT NULL,
        relationship_level INTEGER NOT NULL DEFAULT 0,
        last_interaction TEXT,
        backstory TEXT
      )
    `);

    // Initialize default agents
    const agents = ["hydrix", "serhant", "mio", "karen"];
    for (const agent of agents) {
      await this.client.execute({
        sql: `INSERT OR IGNORE INTO agents (name, current_mood, relationship_level) VALUES (?, ?, ?)`,
        args: [agent, "joyful", 0],
      });
    }

    console.log("Database initialized successfully");
  }

  // Water intake methods
  async logWater(amount_ml: number, notes?: string): Promise<WaterIntake> {
    const id = crypto.randomUUID();
    const timestamp = new Date().toISOString();

    await this.client.execute({
      sql: `INSERT INTO water_intake (id, timestamp, amount_ml, notes) VALUES (?, ?, ?, ?)`,
      args: [id, timestamp, amount_ml, notes || null],
    });

    return { id, timestamp, amount_ml, notes };
  }

  async getWaterStats(): Promise<WaterStats> {
    const today = new Date().toISOString().split("T")[0];

    const result = await this.client.execute({
      sql: `SELECT SUM(amount_ml) as total FROM water_intake WHERE DATE(timestamp) = ?`,
      args: [today],
    });

    const goalResult = await this.client.execute({
      sql: `SELECT goal_ml FROM water_goals WHERE date = ?`,
      args: [today],
    });

    const countResult = await this.client.execute({
      sql: `SELECT COUNT(*) as count FROM water_intake WHERE DATE(timestamp) = ?`,
      args: [today],
    });

    const lastLogResult = await this.client.execute({
      sql: `SELECT timestamp FROM water_intake ORDER BY timestamp DESC LIMIT 1`,
    });

    const today_ml = Number(result.rows[0]?.total || 0);
    const goal_ml = Number(goalResult.rows[0]?.goal_ml || 2000);
    const entries_today = Number(countResult.rows[0]?.count || 0);
    const last_log = lastLogResult.rows[0]?.timestamp as string | null;

    return {
      today_ml,
      goal_ml,
      percentage: (today_ml / goal_ml) * 100,
      entries_today,
      last_log,
    };
  }

  async getWaterHistory(days: number = 7): Promise<WaterIntake[]> {
    const result = await this.client.execute({
      sql: `SELECT * FROM water_intake WHERE timestamp >= datetime('now', '-${days} days') ORDER BY timestamp DESC`,
    });

    return result.rows.map((row) => ({
      id: row.id as string,
      timestamp: row.timestamp as string,
      amount_ml: Number(row.amount_ml),
      notes: row.notes as string | undefined,
    }));
  }

  async setWaterGoal(goal_ml: number): Promise<void> {
    const today = new Date().toISOString().split("T")[0];

    await this.client.execute({
      sql: `INSERT OR REPLACE INTO water_goals (date, goal_ml) VALUES (?, ?)`,
      args: [today, goal_ml],
    });
  }

  // Task methods
  async createTask(
    title: string,
    description?: string,
    priority: string = "medium",
    due_date?: string
  ): Promise<Task> {
    const id = crypto.randomUUID();
    const now = new Date().toISOString();

    await this.client.execute({
      sql: `INSERT INTO tasks (id, title, description, priority, status, created_at, updated_at, due_date) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
      args: [id, title, description || null, priority, "pending", now, now, due_date || null],
    });

    return {
      id,
      title,
      description,
      priority: priority as any,
      status: "pending",
      created_at: now,
      updated_at: now,
      due_date: due_date || null,
      completed_at: null,
    };
  }

  async listTasks(status?: string, priority?: string, limit: number = 50): Promise<Task[]> {
    let sql = `SELECT * FROM tasks WHERE 1=1`;
    const args: any[] = [];

    if (status) {
      sql += ` AND status = ?`;
      args.push(status);
    }

    if (priority) {
      sql += ` AND priority = ?`;
      args.push(priority);
    }

    sql += ` ORDER BY created_at DESC LIMIT ?`;
    args.push(limit);

    const result = await this.client.execute({ sql, args });

    return result.rows.map((row) => ({
      id: row.id as string,
      title: row.title as string,
      description: row.description as string | undefined,
      priority: row.priority as any,
      status: row.status as any,
      created_at: row.created_at as string,
      updated_at: row.updated_at as string,
      due_date: row.due_date as string | null,
      completed_at: row.completed_at as string | null,
    }));
  }

  async updateTask(
    id: string,
    updates: Partial<Omit<Task, "id" | "created_at">>
  ): Promise<void> {
    const fields: string[] = [];
    const args: any[] = [];

    for (const [key, value] of Object.entries(updates)) {
      if (value !== undefined) {
        fields.push(`${key} = ?`);
        args.push(value);
      }
    }

    if (fields.length === 0) return;

    fields.push("updated_at = ?");
    args.push(new Date().toISOString());
    args.push(id);

    await this.client.execute({
      sql: `UPDATE tasks SET ${fields.join(", ")} WHERE id = ?`,
      args,
    });
  }

  // Agent methods
  async getAgentState(name: string): Promise<AgentState | null> {
    const result = await this.client.execute({
      sql: `SELECT * FROM agents WHERE name = ?`,
      args: [name],
    });

    if (result.rows.length === 0) return null;

    const row = result.rows[0];
    return {
      name: row.name as any,
      current_mood: row.current_mood as any,
      relationship_level: Number(row.relationship_level),
      last_interaction: row.last_interaction as string | null,
      backstory: row.backstory as string | undefined,
    };
  }

  async updateAgentState(
    name: string,
    updates: Partial<Omit<AgentState, "name">>
  ): Promise<void> {
    const fields: string[] = [];
    const args: any[] = [];

    for (const [key, value] of Object.entries(updates)) {
      if (value !== undefined) {
        fields.push(`${key} = ?`);
        args.push(value);
      }
    }

    if (fields.length === 0) return;

    args.push(name);

    await this.client.execute({
      sql: `UPDATE agents SET ${fields.join(", ")} WHERE name = ?`,
      args,
    });
  }

  // Cron job methods
  async createCronJob(
    name: string,
    schedule: string,
    action: string,
    description?: string,
    metadata?: Record<string, unknown>
  ): Promise<CronJob> {
    const id = crypto.randomUUID();

    await this.client.execute({
      sql: `INSERT INTO cron_jobs (id, name, description, schedule, action, metadata, enabled) VALUES (?, ?, ?, ?, ?, ?, 1)`,
      args: [id, name, description || null, schedule, action, JSON.stringify(metadata || {})],
    });

    return {
      id,
      name,
      description,
      schedule,
      action: action as any,
      enabled: true,
      last_run: null,
      next_run: null,
      metadata,
    };
  }

  async listCronJobs(): Promise<CronJob[]> {
    const result = await this.client.execute(`SELECT * FROM cron_jobs ORDER BY name`);

    return result.rows.map((row) => ({
      id: row.id as string,
      name: row.name as string,
      description: row.description as string | undefined,
      schedule: row.schedule as string,
      action: row.action as any,
      enabled: Boolean(row.enabled),
      last_run: row.last_run as string | null,
      next_run: row.next_run as string | null,
      metadata: row.metadata ? JSON.parse(row.metadata as string) : undefined,
    }));
  }

  async updateCronJob(id: string, updates: Partial<Omit<CronJob, "id">>): Promise<void> {
    const fields: string[] = [];
    const args: any[] = [];

    for (const [key, value] of Object.entries(updates)) {
      if (value !== undefined) {
        if (key === "metadata") {
          fields.push(`${key} = ?`);
          args.push(JSON.stringify(value));
        } else {
          fields.push(`${key} = ?`);
          args.push(value);
        }
      }
    }

    if (fields.length === 0) return;

    args.push(id);

    await this.client.execute({
      sql: `UPDATE cron_jobs SET ${fields.join(", ")} WHERE id = ?`,
      args,
    });
  }
}

// Singleton instance
export const db = new Database();
