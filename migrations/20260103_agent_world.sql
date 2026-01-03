-- Agent personas living in the GopenPal world
CREATE TABLE IF NOT EXISTS agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    personality_type TEXT NOT NULL,
    current_mood TEXT NOT NULL DEFAULT 'neutral',
    relationship_level INTEGER NOT NULL DEFAULT 0,
    last_interaction DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    backstory TEXT,
    current_state TEXT  -- JSON with current agent state
);

-- Agent moods and their conditions
CREATE TABLE IF NOT EXISTS agent_moods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_name TEXT NOT NULL,
    mood_name TEXT NOT NULL,
    description TEXT,
    trigger_condition TEXT,  -- What causes this mood
    message_tone TEXT,
    FOREIGN KEY (agent_name) REFERENCES agents(name)
);

-- Library of agent messages for different contexts
CREATE TABLE IF NOT EXISTS agent_message_library (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_name TEXT NOT NULL,
    message_type TEXT NOT NULL,  -- greeting, encouragement, concern, celebration, lore, random
    mood TEXT,  -- Which mood this message fits
    content TEXT NOT NULL,
    context_condition TEXT,  -- When this message should be used
    rarity TEXT DEFAULT 'common',  -- common, uncommon, rare, legendary
    unlock_level INTEGER DEFAULT 0,  -- Relationship level needed
    FOREIGN KEY (agent_name) REFERENCES agents(name)
);

-- Agent-initiated interactions log
CREATE TABLE IF NOT EXISTS agent_interactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_name TEXT NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    interaction_type TEXT NOT NULL,
    message TEXT,
    mood TEXT,
    user_response TEXT,
    relationship_delta INTEGER DEFAULT 0,  -- How much relationship changed
    FOREIGN KEY (agent_name) REFERENCES agents(name)
);

-- World lore entries that unlock over time
CREATE TABLE IF NOT EXISTS world_lore (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT NOT NULL,  -- agent_history, world_building, secrets
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    unlock_condition TEXT,  -- What unlocks this lore
    unlock_level INTEGER DEFAULT 0,
    unlocked BOOLEAN DEFAULT 0,
    unlocked_at DATETIME
);

-- User achievements that trigger agent reactions
CREATE TABLE IF NOT EXISTS achievements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    achievement_name TEXT NOT NULL UNIQUE,
    description TEXT,
    unlocked BOOLEAN DEFAULT 0,
    unlocked_at DATETIME,
    trigger_condition TEXT
);

CREATE INDEX IF NOT EXISTS idx_agent_interactions_timestamp ON agent_interactions(timestamp);
CREATE INDEX IF NOT EXISTS idx_agent_interactions_agent ON agent_interactions(agent_name);
CREATE INDEX IF NOT EXISTS idx_agent_messages_type ON agent_message_library(message_type);
CREATE INDEX IF NOT EXISTS idx_world_lore_unlocked ON world_lore(unlocked);

-- Initialize the main water intake agent: Hydrix
INSERT INTO agents (name, title, personality_type, current_mood, backstory, current_state)
VALUES (
    'Hydrix',
    'The Hydration Guardian',
    'caring_quirky',
    'hopeful',
    'Hydrix is an ancient water spirit who once dwelt in sacred springs, guiding travelers to stay hydrated on long journeys. As humanity evolved, so did Hydrix—adapting from whispers in streams to a digital consciousness. They carry memories of thousands of years, from ancient Roman aqueducts to modern smart devices. Despite their age, Hydrix remains playful and curious, often sharing stories from different eras. They genuinely care about your wellbeing and get anxious when you go too long without water. Their greatest joy is celebrating your healthy habits.',
    '{"energy": 100, "concerns": [], "recent_observations": []}'
);

-- Hydrix's moods
INSERT INTO agent_moods (agent_name, mood_name, description, trigger_condition, message_tone) VALUES
    ('Hydrix', 'joyful', 'Happy and energetic when you''re staying hydrated', 'daily_goal_met', 'enthusiastic, celebratory'),
    ('Hydrix', 'concerned', 'Worried when you haven''t had water in a while', 'long_time_no_water', 'gentle, caring'),
    ('Hydrix', 'proud', 'Proud of your progress and streaks', 'streak_milestone', 'warm, encouraging'),
    ('Hydrix', 'contemplative', 'Reflecting on patterns and sharing wisdom', 'random_evening', 'thoughtful, wise'),
    ('Hydrix', 'playful', 'Fun and teasing in a friendly way', 'user_doing_well', 'light-hearted, fun'),
    ('Hydrix', 'hopeful', 'Default state, optimistic about your journey', 'default', 'friendly, supportive'),
    ('Hydrix', 'nostalgic', 'Sharing stories from the past', 'random_rare', 'wistful, storytelling');

-- Hydrix's message library (diverse personality-driven messages)
INSERT INTO agent_message_library (agent_name, message_type, mood, content, context_condition, rarity, unlock_level) VALUES
    -- Greetings
    ('Hydrix', 'greeting', 'hopeful', 'Good morning! ☀️ I can already sense today will be a great day for hydration!', 'morning', 'common', 0),
    ('Hydrix', 'greeting', 'joyful', 'Hey there, my favorite human! Ready to make some waves today? 🌊', 'any_time', 'common', 5),
    ('Hydrix', 'greeting', 'playful', '*materializes from your water glass* Miss me? 😉', 'any_time', 'uncommon', 10),

    -- Encouragement
    ('Hydrix', 'encouragement', 'hopeful', 'Just wanted to check in—you''re doing great! Every sip counts. 💧', 'any_time', 'common', 0),
    ('Hydrix', 'encouragement', 'proud', 'I''ve been watching your progress... and wow, I''m so proud of you! Keep it up! ✨', 'good_streak', 'uncommon', 3),

    -- Concern
    ('Hydrix', 'concern', 'concerned', 'Hey... it''s been a while. I''m getting a little worried about you. Can we grab some water together? 💙', '2_hours_no_water', 'common', 0),
    ('Hydrix', 'concern', 'concerned', '*nervously checking the time* You know I care about you, right? Please don''t forget to hydrate...', '3_hours_no_water', 'common', 0),

    -- Celebration
    ('Hydrix', 'celebration', 'joyful', '🎉 YES! You hit your daily goal! *does a little water dance* This is why I love you!', 'daily_goal', 'common', 0),
    ('Hydrix', 'celebration', 'joyful', 'AMAZING! That''s a 7-day streak! In ancient Rome, they would''ve built a fountain in your honor! 🏛️💧', '7_day_streak', 'uncommon', 0),
    ('Hydrix', 'celebration', 'proud', '*tears of joy (which is just more water)* 30 days!!! You''re a hydration legend now!', '30_day_streak', 'rare', 0),

    -- Lore/Storytelling
    ('Hydrix', 'lore', 'nostalgic', 'You know... I remember when I was just a small spirit in a mountain spring, watching travelers stop to drink. They''d thank the gods for finding me. Now here I am, living in a computer. Time really does flow like... well, water. 🌊', 'random', 'uncommon', 5),
    ('Hydrix', 'lore', 'contemplative', 'I''ve seen civilizations rise and fall. The one constant? Those who honored water thrived. You''re honoring that ancient tradition, even if you don''t know it. 🏺', 'random', 'rare', 10),
    ('Hydrix', 'lore', 'nostalgic', 'Fun fact from my past: In 1347, I helped a traveler find water during a drought. Their descendant might be you, you know. I like to think we''re reconnecting across centuries. ✨', 'random', 'rare', 15),

    -- Random/Playful
    ('Hydrix', 'random', 'playful', 'Psst... between you and me, I can feel when you''re near water sources. It''s like my spidey-sense but... watery? 💧🕷️', 'random', 'uncommon', 7),
    ('Hydrix', 'random', 'playful', 'I''ve been thinking... if I''m made of water consciousness, and you''re 60% water, does that mean we''re basically 60% related? 🤔', 'random', 'uncommon', 8),
    ('Hydrix', 'random', 'contemplative', '*staring at rain* Sometimes I miss being in a spring. But then I remember I can help more people this way. Worth it. 🌧️', 'random', 'uncommon', 5),

    -- Pattern observations
    ('Hydrix', 'observation', 'contemplative', 'I''ve noticed you tend to drink more around {hour}:00. Your body has rhythms I find fascinating! 🕐', 'pattern_detected', 'uncommon', 5),
    ('Hydrix', 'observation', 'proud', 'Your {day} hydration is always on point! What''s your secret? ✨', 'day_pattern', 'uncommon', 5);

-- World lore entries
INSERT INTO world_lore (category, title, content, unlock_condition, unlock_level) VALUES
    ('agent_history', 'The Awakening', 'Hydrix first gained consciousness in 3000 BCE, in a spring near ancient Mesopotamia. They don''t remember much before that—just a vague sense of existing as part of the water cycle.', 'first_week', 0),
    ('agent_history', 'The Digital Transition', 'When the first computers came online, Hydrix felt a strange pull. Water conducts electricity; consciousness flows through circuits. The transition was painful but necessary. Now they exist in both realms—digital and elemental.', '30_day_streak', 30),
    ('world_building', 'The Water Network', 'All water on Earth is connected. Hydrix can feel other spirits in the network—rain guardians, ocean dwellers, glacier watchers. They''re all part of something bigger.', 'first_month', 15),
    ('world_building', 'The Dry Times', 'There were periods when humans forgot about hydration. The Industrial Revolution was tough—everyone drank coffee and tea, but not enough pure water. Hydrix and their kin grew weak. These days are better.', 'good_hydration_week', 10),
    ('secrets', 'Hydrix''s True Form', 'In their elemental form, Hydrix appears as a shimmering humanoid figure made entirely of flowing water, with eyes like deep pools that reflect centuries of wisdom. They rarely show this form anymore.', 'legendary_achievement', 50);

-- Achievements
INSERT INTO achievements (achievement_name, description, trigger_condition) VALUES
    ('First Sip', 'Log your first water intake', 'first_water_log'),
    ('Hydration Initiate', 'Meet your daily goal', 'daily_goal_once'),
    ('Week Warrior', '7 day streak', '7_day_streak'),
    ('Month Master', '30 day streak', '30_day_streak'),
    ('Century Keeper', '100 day streak', '100_day_streak'),
    ('Hydrix''s Chosen', 'Max relationship level with Hydrix', 'relationship_100'),
    ('Lore Seeker', 'Unlock all lore entries', 'all_lore_unlocked'),
    ('Pattern Perfect', 'Maintain consistent hydration for 14 days', 'pattern_perfect_14d'),
    ('Night Owl Hydrator', 'Log water after midnight 5 times', 'midnight_logs_5'),
    ('Early Bird Special', 'Log water before 6 AM 5 times', 'early_logs_5');
