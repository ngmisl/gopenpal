-- Water intake tracking
CREATE TABLE IF NOT EXISTS water_intake (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    amount_ml INTEGER NOT NULL,
    notes TEXT
);

-- Water reminder settings
CREATE TABLE IF NOT EXISTS reminder_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    enabled BOOLEAN NOT NULL DEFAULT 1,
    interval_minutes INTEGER NOT NULL DEFAULT 60,
    work_hours_start TIME NOT NULL DEFAULT '09:00:00',
    work_hours_end TIME NOT NULL DEFAULT '18:00:00',
    work_days TEXT NOT NULL DEFAULT 'Mon,Tue,Wed,Thu,Fri'
);

-- Chat history with AI
CREATE TABLE IF NOT EXISTS chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content TEXT NOT NULL,
    model TEXT,
    tokens_used INTEGER
);

-- Health metrics (for future expansion)
CREATE TABLE IF NOT EXISTS health_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    unit TEXT,
    notes TEXT
);

-- Work sessions (for future expansion)
CREATE TABLE IF NOT EXISTS work_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    activity TEXT NOT NULL,
    notes TEXT
);

-- Insert default reminder settings
INSERT OR IGNORE INTO reminder_settings (id, enabled, interval_minutes, work_hours_start, work_hours_end, work_days)
VALUES (1, 1, 60, '09:00:00', '18:00:00', 'Mon,Tue,Wed,Thu,Fri');

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_water_intake_timestamp ON water_intake(timestamp);
CREATE INDEX IF NOT EXISTS idx_chat_history_timestamp ON chat_history(timestamp);
CREATE INDEX IF NOT EXISTS idx_health_metrics_timestamp ON health_metrics(timestamp);
CREATE INDEX IF NOT EXISTS idx_work_sessions_start ON work_sessions(start_time);
