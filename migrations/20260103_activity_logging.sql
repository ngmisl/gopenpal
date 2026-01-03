-- Activity logging for tracking all system events
CREATE TABLE IF NOT EXISTS activity_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    activity_type TEXT NOT NULL CHECK (activity_type IN (
        'reminder_fired',
        'reminder_acted',
        'water_logged',
        'cron_installed',
        'cron_removed',
        'cron_updated',
        'chat_interaction',
        'stats_requested',
        'settings_changed'
    )),
    details TEXT,
    metadata TEXT  -- JSON for additional data
);

-- Reminder effectiveness tracking
CREATE TABLE IF NOT EXISTS reminder_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reminder_type TEXT NOT NULL,  -- 'cron', 'manual', 'foreground'
    action_taken BOOLEAN NOT NULL DEFAULT 0,
    water_intake_id INTEGER,  -- Link to water_intake if acted upon
    response_time_seconds INTEGER,
    FOREIGN KEY (water_intake_id) REFERENCES water_intake(id)
);

-- Create indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_activity_log_timestamp ON activity_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_activity_log_type ON activity_log(activity_type);
CREATE INDEX IF NOT EXISTS idx_reminder_events_timestamp ON reminder_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_reminder_events_type ON reminder_events(reminder_type);

-- Create views for common analytics queries

-- Daily water intake statistics
CREATE VIEW IF NOT EXISTS daily_water_stats AS
SELECT
    DATE(timestamp) as date,
    COUNT(*) as intake_count,
    SUM(amount_ml) as total_ml,
    AVG(amount_ml) as avg_ml,
    MIN(amount_ml) as min_ml,
    MAX(amount_ml) as max_ml
FROM water_intake
GROUP BY DATE(timestamp)
ORDER BY date DESC;

-- Hourly patterns (which hours user drinks most)
CREATE VIEW IF NOT EXISTS hourly_patterns AS
SELECT
    CAST(strftime('%H', timestamp) AS INTEGER) as hour,
    COUNT(*) as intake_count,
    SUM(amount_ml) as total_ml,
    AVG(amount_ml) as avg_ml
FROM water_intake
GROUP BY hour
ORDER BY hour;

-- Weekly patterns (which days user drinks most)
CREATE VIEW IF NOT EXISTS weekly_patterns AS
SELECT
    CASE CAST(strftime('%w', timestamp) AS INTEGER)
        WHEN 0 THEN 'Sunday'
        WHEN 1 THEN 'Monday'
        WHEN 2 THEN 'Tuesday'
        WHEN 3 THEN 'Wednesday'
        WHEN 4 THEN 'Thursday'
        WHEN 5 THEN 'Friday'
        WHEN 6 THEN 'Saturday'
    END as day_name,
    CAST(strftime('%w', timestamp) AS INTEGER) as day_num,
    COUNT(*) as intake_count,
    SUM(amount_ml) as total_ml,
    AVG(amount_ml) as avg_ml
FROM water_intake
GROUP BY day_num
ORDER BY day_num;

-- Reminder effectiveness stats
CREATE VIEW IF NOT EXISTS reminder_effectiveness AS
SELECT
    reminder_type,
    COUNT(*) as total_reminders,
    SUM(CASE WHEN action_taken THEN 1 ELSE 0 END) as actions_taken,
    ROUND(100.0 * SUM(CASE WHEN action_taken THEN 1 ELSE 0 END) / COUNT(*), 2) as effectiveness_percentage,
    AVG(CASE WHEN action_taken THEN response_time_seconds ELSE NULL END) as avg_response_seconds
FROM reminder_events
GROUP BY reminder_type;
