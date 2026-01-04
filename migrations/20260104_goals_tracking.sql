-- Goals tracking system
-- Supports daily water goals and future extensibility for other goal types

CREATE TABLE IF NOT EXISTS goals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_type TEXT NOT NULL,              -- 'daily_water', 'weekly_water', 'task_completion', etc.
    target_value REAL NOT NULL,           -- Numeric target (e.g., 2000 for 2000ml)
    unit TEXT NOT NULL,                   -- 'ml', 'tasks', 'hours', etc.
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    active INTEGER NOT NULL DEFAULT 1,    -- 1 for active, 0 for inactive
    notes TEXT                            -- Optional context or reasoning
);

-- Goal progress history (tracks daily/weekly achievement)
CREATE TABLE IF NOT EXISTS goal_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_id INTEGER NOT NULL,
    date TEXT NOT NULL,                   -- Date for this progress entry (YYYY-MM-DD)
    actual_value REAL NOT NULL,           -- Actual achievement (e.g., 1800ml)
    target_value REAL NOT NULL,           -- Target for reference
    achieved INTEGER NOT NULL DEFAULT 0,  -- 1 if goal met, 0 otherwise
    percentage REAL NOT NULL,             -- Progress as percentage (0-100+)
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (goal_id) REFERENCES goals(id) ON DELETE CASCADE,
    UNIQUE(goal_id, date)                 -- One progress record per goal per day
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_goals_type_active ON goals(goal_type, active);
CREATE INDEX IF NOT EXISTS idx_goal_progress_date ON goal_progress(date DESC);
CREATE INDEX IF NOT EXISTS idx_goal_progress_goal_date ON goal_progress(goal_id, date DESC);

-- Insert default daily water goal (2000ml - standard recommendation)
INSERT INTO goals (goal_type, target_value, unit, notes)
VALUES ('daily_water', 2000.0, 'ml', 'Default daily hydration goal (8 glasses)')
ON CONFLICT DO NOTHING;
