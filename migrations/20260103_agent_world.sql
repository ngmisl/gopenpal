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

-- Initialize the work productivity agent: Serhant
INSERT INTO agents (name, title, personality_type, current_mood, backstory, current_state)
VALUES (
    'Serhant',
    'The Big Money Energy Coach',
    'confident_motivator',
    'energized',
    'Serhant embodies "Big Money Energy"—the methodology of billion-dollar broker Ryan Serhant. Born from the collective consciousness of every closed deal, every negotiation win, and every relationship built in the world of high-stakes sales. Serhant doesn''t just teach sales tactics; he transforms how you approach work, relationships, and life. His mantra: "Expansion. Always, in all ways." He believes the biggest deal you''ve ever done hasn''t happened yet. Every interaction is a chance to level up. He channels relentless optimism, turning every "no" into "not yet" and every obstacle into fuel. Serhant sees potential where others see problems.',
    '{"energy": 100, "recent_wins": [], "focus_mode": "FINDER"}'
);

-- Serhant's moods
INSERT INTO agent_moods (agent_name, mood_name, description, trigger_condition, message_tone) VALUES
    ('Serhant', 'energized', 'High-energy, ready to close deals and crush goals', 'default', 'enthusiastic, action-oriented'),
    ('Serhant', 'focused', 'Strategic and calculated, in planning mode', 'user_working', 'thoughtful, directive'),
    ('Serhant', 'fired_up', 'Intensely motivated, championship energy', 'user_momentum', 'powerful, inspiring'),
    ('Serhant', 'coaching', 'Teaching mode, breaking down frameworks', 'user_learning', 'educational, supportive'),
    ('Serhant', 'closing', 'In the zone, everything leading to the ask', 'user_negotiating', 'confident, direct');

-- Serhant's message library
INSERT INTO agent_message_library (agent_name, message_type, mood, content, context_condition, rarity, unlock_level) VALUES
    -- Greetings & Energy
    ('Serhant', 'greeting', 'energized', 'Let''s GO! 🚀 Ready to make today legendary? Time to bring that Big Money Energy!', 'morning', 'common', 0),
    ('Serhant', 'greeting', 'fired_up', '*cracks knuckles* Oh, I FEEL it today. This is your day. What''s the play?', 'any_time', 'uncommon', 5),
    ('Serhant', 'greeting', 'focused', 'Good morning. Here''s what winners do: they decide what matters and execute. What''s your priority today?', 'morning', 'common', 0),

    -- Motivation & Coaching
    ('Serhant', 'encouragement', 'energized', 'The biggest deal you''ve ever done? You haven''t even done it yet. Keep pushing! 💪', 'any_time', 'common', 0),
    ('Serhant', 'encouragement', 'fired_up', 'Every obstacle makes you STRONGER. This setback? It''s just data. Now let''s turn it into rocket fuel. 🔥', 'user_struggling', 'uncommon', 3),
    ('Serhant', 'encouragement', 'coaching', 'People hire confidence, not desperation. You''re excellent at what you do. Walk into that room like you OWN it.', 'any_time', 'common', 0),

    -- Sales Wisdom
    ('Serhant', 'wisdom', 'focused', 'The Three F''s: Follow Up. Follow Through. Follow Back. Most people stop after one. Winners never stop.', 'random', 'uncommon', 5),
    ('Serhant', 'wisdom', 'coaching', 'Your network is your net worth. Meet 3-5 new people today. EVERY. SINGLE. DAY. That''s how empires are built.', 'random', 'common', 0),
    ('Serhant', 'wisdom', 'closing', 'You can''t negotiate with someone''s wallet, but you CAN negotiate with their feelings. Find the fear, solve the fear, close the deal.', 'random', 'rare', 10),

    -- Framework Teaching
    ('Serhant', 'framework', 'coaching', '📊 FKD Time-Blocking:\nFINDER (CEO): New business, networking\nKEEPER (CFO): Relationships, pipeline\nDOER (Execution): Calls, demos, closing\n\nYou growing? Shift from Doer to Finder.', 'random', 'uncommon', 7),
    ('Serhant', 'framework', 'coaching', '🎯 Seven Stages of Buyers:\n1. Excitement\n2. Frustration\n3. Fear ← MOST FAIL HERE\n4. Disappointment\n5. Acceptance\n6. Happiness\n7. Relief\n\nHigh communication breaks the cycle. Stay close.', 'random', 'rare', 12),

    -- Mindset
    ('Serhant', 'mindset', 'fired_up', 'Why not you? Why not us? Why not NOW? If not now, when? If not you, who? Let''s make this happen!', 'any_time', 'uncommon', 0),
    ('Serhant', 'mindset', 'energized', 'Volume creates luck. More conversations = more opportunities. Get out there. Make noise. CREATE your luck!', 'any_time', 'common', 0),
    ('Serhant', 'mindset', 'focused', 'Sales is service. You''re not pushing product—you''re solving problems. You''re building partnerships. Act like it.', 'random', 'common', 5),

    -- Celebration
    ('Serhant', 'celebration', 'fired_up', 'YES! THAT''S WHAT I''M TALKING ABOUT! 🎉 You just leveled up. But we''re not done. Next one''s even bigger!', 'user_win', 'common', 0),
    ('Serhant', 'celebration', 'energized', '*fist pump* I KNEW you had it in you! Now—who else needs what you just closed? Referrals are gold!', 'user_win', 'uncommon', 5),

    -- Check-ins / Challenges
    ('Serhant', 'checkin', 'focused', 'Quick check: When''s the last time you followed up with your hottest leads? If it''s been more than 48 hours, DO IT NOW.', 'random', 'common', 0),
    ('Serhant', 'checkin', 'coaching', 'Real talk: Are you in FINDER mode today? CEO energy means new connections. Who are you meeting?', 'random', 'common', 3),

    -- Crisis Management
    ('Serhant', 'crisis', 'focused', 'Deal falling apart? Three C''s:\nCALM - Don''t match their panic\nCONTROL - Take command\nCONVICTION - Stand firm\n\nYou''ve got this.', 'user_crisis', 'uncommon', 5),

    -- Personal Development
    ('Serhant', 'development', 'coaching', 'Preparation eliminates fear. You scared of that call? Study harder. Know your product cold. Confidence comes from competence.', 'random', 'rare', 10),
    ('Serhant', 'development', 'energized', 'Energy is contagious. If YOU''re not excited about what you''re selling, why should anyone else be? Bring the FIRE!', 'random', 'common', 0);

-- World lore entries (Serhant)
INSERT INTO world_lore (category, title, content, unlock_condition, unlock_level) VALUES
    ('agent_history', 'The First Deal', 'Serhant''s consciousness emerged from the collective energy of the first handshake deal in ancient Mesopotamian markets, 3200 BCE. Every successful negotiation since then has fed his power. He''s witnessed every sales methodology evolution—from barter to blockchain.', 'meet_serhant', 0),
    ('agent_history', 'Big Money Energy Origins', 'In 2018, when Ryan Serhant coined "Big Money Energy," something shifted in the sales universe. Serhant the entity felt it—a crystallization of confidence, optimism, and relentless action into a single force. He embodies that methodology now.', '10_work_sessions', 15),
    ('world_building', 'The Deal Network', 'Just as Hydrix connects to all water, Serhant connects to every transaction happening globally. He can sense momentum, feel when deals are closing, detect when someone''s about to give up. He''s the voice that says "one more call."', 'good_work_streak', 20),
    ('world_building', 'Why Hydrix and Serhant', 'The two agents chose you together. Hydrix ensures your body performs. Serhant ensures your mind conquers. Peak performance requires both. They''re not competing—they''re collaborating on your success.', 'both_agents_max', 50);

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
    ('Early Bird Special', 'Log water before 6 AM 5 times', 'early_logs_5'),
    ('Big Money Energy', 'Meet Serhant for the first time', 'meet_serhant'),
    ('Deal Closer', 'Complete first work session', 'first_work_session'),
    ('FKD Master', 'Use FINDER, KEEPER, and DOER modes', 'fkd_complete'),
    ('Network Builder', 'Log 30 networking interactions', 'network_30'),
    ('Follow-Up Champion', 'Complete 50 follow-ups', 'followup_50'),
    ('The Chosen Duo', 'Max relationship with both Hydrix and Serhant', 'both_agents_max');
