-- Work session tracking for Serhant's effectiveness analysis

CREATE TABLE IF NOT EXISTS work_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time DATETIME,
    session_type TEXT NOT NULL CHECK (session_type IN ('FINDER', 'KEEPER', 'DOER', 'general')),
    title TEXT,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    productivity_rating INTEGER CHECK (productivity_rating BETWEEN 1 AND 5),
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS work_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT CHECK (priority IN ('low', 'medium', 'high', 'urgent')) DEFAULT 'medium',
    status TEXT CHECK (status IN ('pending', 'in_progress', 'completed', 'blocked')) DEFAULT 'pending',
    due_date DATE,
    completed_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES work_sessions(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS follow_ups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_name TEXT NOT NULL,
    contact_type TEXT CHECK (contact_type IN ('client', 'lead', 'partner', 'network')) DEFAULT 'lead',
    last_contact_date DATE NOT NULL,
    next_follow_up_date DATE,
    notes TEXT,
    status TEXT CHECK (status IN ('active', 'completed', 'lost')) DEFAULT 'active',
    priority TEXT CHECK (priority IN ('low', 'medium', 'high', 'urgent')) DEFAULT 'medium',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS networking_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_name TEXT NOT NULL,
    contact_method TEXT CHECK (contact_method IN ('in_person', 'phone', 'email', 'social_media', 'event')) DEFAULT 'in_person',
    date DATE NOT NULL DEFAULT (DATE('now')),
    notes TEXT,
    follow_up_needed BOOLEAN DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Views for analytics

CREATE VIEW IF NOT EXISTS work_session_stats AS
SELECT
    session_type,
    COUNT(*) as session_count,
    AVG(productivity_rating) as avg_productivity,
    SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END) as completed_count,
    AVG(CASE
        WHEN end_time IS NOT NULL
        THEN (julianday(end_time) - julianday(start_time)) * 24
        ELSE 0
    END) as avg_duration_hours
FROM work_sessions
GROUP BY session_type;

CREATE VIEW IF NOT EXISTS task_completion_rate AS
SELECT
    DATE(created_at) as date,
    COUNT(*) as total_tasks,
    SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) as completed_tasks,
    ROUND(100.0 * SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) / COUNT(*), 2) as completion_rate
FROM work_tasks
GROUP BY DATE(created_at)
ORDER BY date DESC;

CREATE VIEW IF NOT EXISTS follow_up_effectiveness AS
SELECT
    contact_type,
    COUNT(*) as total_contacts,
    SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) as successful_count,
    ROUND(100.0 * SUM(CASE WHEN status = 'completed' THEN 1 ELSE 0 END) / COUNT(*), 2) as success_rate,
    AVG(julianday(updated_at) - julianday(created_at)) as avg_days_to_close
FROM follow_ups
GROUP BY contact_type;

CREATE VIEW IF NOT EXISTS networking_activity AS
SELECT
    strftime('%Y-%m', date) as month,
    COUNT(*) as contacts_made,
    COUNT(DISTINCT contact_name) as unique_contacts,
    SUM(CASE WHEN follow_up_needed = 1 THEN 1 ELSE 0 END) as pending_follow_ups
FROM networking_log
GROUP BY strftime('%Y-%m', date)
ORDER BY month DESC;

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_work_sessions_type ON work_sessions(session_type);
CREATE INDEX IF NOT EXISTS idx_work_sessions_dates ON work_sessions(start_time, end_time);
CREATE INDEX IF NOT EXISTS idx_work_tasks_status ON work_tasks(status);
CREATE INDEX IF NOT EXISTS idx_work_tasks_session ON work_tasks(session_id);
CREATE INDEX IF NOT EXISTS idx_follow_ups_dates ON follow_ups(next_follow_up_date);
CREATE INDEX IF NOT EXISTS idx_networking_date ON networking_log(date);
