-- SQLite Migration: Enhanced English Learning Schema
-- Version: 002
-- Date: 2026-01-14
-- Description: Adds tables for scenes, scenes, classic dialogues, reading practice, and learning sessions

-- ============================================================================
-- SCENARIOS & SCENES
-- ============================================================================

-- Table: scenes
-- Main scenario categories (e.g., Airport, Hotel, Restaurant, Interview)
CREATE TABLE IF NOT EXISTS scenes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_en TEXT NOT NULL,
    name_zh TEXT NOT NULL,
    description_en TEXT,
    description_zh TEXT,
    icon_emoji TEXT, -- Emoji representation
    difficulty_level TEXT CHECK(difficulty_level IN ('beginner', 'intermediate', 'advanced')) DEFAULT 'intermediate',
    category TEXT, -- 'daily', 'business', 'travel', etc.
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    display_order INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1,
    
    UNIQUE(name_en)
);

CREATE INDEX IF NOT EXISTS idx_scenes_active 
ON scenes(is_active, display_order);

-- Table: scene_dialogues
-- Specific dialogue scripts within scenes
CREATE TABLE IF NOT EXISTS scene_dialogues (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sceneid INTEGER NOT NULL,
    title_en TEXT NOT NULL,
    title_zh TEXT NOT NULL,
    description_en TEXT,
    description_zh TEXT,
    total_turns INTEGER DEFAULT 0, -- Number of dialogue turns
    estimated_duration_seconds INTEGER,
    difficulty_level TEXT CHECK(difficulty_level IN ('beginner', 'intermediate', 'advanced')),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    
    FOREIGN KEY (sceneid) REFERENCES scenes(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_scene_dialogues_scenario 
ON scene_dialogues(sceneid);

-- Table: dialogue_turns
-- Individual lines in a dialogue
CREATE TABLE IF NOT EXISTS dialogue_turns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scene_dialogue_id INTEGER NOT NULL,
    turn_number INTEGER NOT NULL,
    speaker_role TEXT NOT NULL, -- 'user', 'npc', 'ai_teacher'
    speaker_name TEXT, -- Character name (e.g., "Receptionist", "Customer")
    content_en TEXT NOT NULL,
    content_zh TEXT NOT NULL,
    audio_path TEXT,
    phonetic_transcription TEXT, -- IPA transcription
    key_phrases TEXT, -- JSON array of important phrases
    notes TEXT, -- Teaching notes or context
    
    FOREIGN KEY (scene_dialogue_id) REFERENCES scene_dialogues(id) ON DELETE CASCADE,
    UNIQUE(scene_dialogue_id, turn_number)
);

CREATE INDEX IF NOT EXISTS idx_dialogue_turns_scene 
ON dialogue_turns(scene_dialogue_id, turn_number);

-- ============================================================================
-- CLASSIC DIALOGUES (Movies, TV, TED)
-- ============================================================================

-- Table: classic_dialogue_sources
-- Sources like movies, TV shows, TED talks
CREATE TABLE IF NOT EXISTS classic_dialogue_sources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_type TEXT NOT NULL CHECK(source_type IN ('movie', 'tv_show', 'ted_talk', 'documentary', 'other')),
    title TEXT NOT NULL,
    year INTEGER,
    description_en TEXT,
    description_zh TEXT,
    thumbnail_url TEXT,
    imdb_id TEXT,
    difficulty_level TEXT CHECK(difficulty_level IN ('beginner', 'intermediate', 'advanced')),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    
    UNIQUE(source_type, title)
);

CREATE INDEX IF NOT EXISTS idx_classic_sources_type 
ON classic_dialogue_sources(source_type);

-- Table: classic_dialogue_clips
-- Specific clips/scenes from sources
CREATE TABLE IF NOT EXISTS classic_dialogue_clips (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id INTEGER NOT NULL,
    clip_title_en TEXT NOT NULL,
    clip_title_zh TEXT NOT NULL,
    start_time_seconds INTEGER, -- Start time in source material
    end_time_seconds INTEGER,
    video_url TEXT,
    transcript_en TEXT NOT NULL,
    transcript_zh TEXT NOT NULL,
    key_vocabulary TEXT, -- JSON array of vocabulary items
    cultural_notes TEXT,
    grammar_points TEXT, -- JSON array of grammar points
    difficulty_vocab INTEGER DEFAULT 3 CHECK(difficulty_vocab BETWEEN 1 AND 5),
    difficulty_speed INTEGER DEFAULT 3 CHECK(difficulty_speed BETWEEN 1 AND 5),
    difficulty_slang INTEGER DEFAULT 3 CHECK(difficulty_slang BETWEEN 1 AND 5),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    popularity_score INTEGER DEFAULT 0,
    
    FOREIGN KEY (source_id) REFERENCES classic_dialogue_sources(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_classic_clips_source 
ON classic_dialogue_clips(source_id);

CREATE INDEX IF NOT EXISTS idx_classic_clips_popularity 
ON classic_dialogue_clips(popularity_score DESC);

-- ============================================================================
-- READING PRACTICE
-- ============================================================================

-- Table: reading_exercises
-- Pronunciation/reading practice exercises
CREATE TABLE IF NOT EXISTS reading_exercises (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title_en TEXT NOT NULL,
    title_zh TEXT NOT NULL,
    description_en TEXT,
    description_zh TEXT,
    difficulty_level TEXT CHECK(difficulty_level IN ('beginner', 'intermediate', 'advanced')),
    exercise_type TEXT CHECK(exercise_type IN ('sentence', 'paragraph', 'dialogue', 'tongue_twister')) DEFAULT 'sentence',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Table: reading_sentences
-- Individual sentences for reading practice
CREATE TABLE IF NOT EXISTS reading_sentences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    exercise_id INTEGER NOT NULL,
    sentence_order INTEGER NOT NULL,
    content_en TEXT NOT NULL,
    content_zh TEXT NOT NULL,
    phonetic_transcription TEXT,
    native_audio_path TEXT,
    focus_sounds TEXT, -- JSON array of sounds to focus on (e.g., "th", "r", "l")
    common_mistakes TEXT, -- JSON array of common pronunciation mistakes
    
    FOREIGN KEY (exercise_id) REFERENCES reading_exercises(id) ON DELETE CASCADE,
    UNIQUE(exercise_id, sentence_order)
);

CREATE INDEX IF NOT EXISTS idx_reading_sentences_exercise 
ON reading_sentences(exercise_id, sentence_order);

-- Table: reading_practice_attempts
-- Log of user's reading practice attempts
CREATE TABLE IF NOT EXISTS reading_practice_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER DEFAULT 1, -- For multi-user support later
    sentence_id INTEGER NOT NULL,
    session_id TEXT NOT NULL,
    attempted_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    user_audio_path TEXT,
    
    -- AI Scoring
    pronunciation_score INTEGER CHECK(pronunciation_score BETWEEN 0 AND 100),
    fluency_score INTEGER CHECK(fluency_score BETWEEN 0 AND 100),
    intonation_score INTEGER CHECK(intonation_score BETWEEN 0 AND 100),
    overall_score INTEGER CHECK(overall_score BETWEEN 0 AND 100),
    
    -- Detailed feedback
    detected_errors TEXT, -- JSON array of detected pronunciation errors
    ai_feedback_en TEXT,
    ai_feedback_zh TEXT,
    waveform_data TEXT, -- JSON data for waveform visualization
    
    FOREIGN KEY (sentence_id) REFERENCES reading_sentences(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_reading_attempts_sentence 
ON reading_practice_attempts(sentence_id, attempted_at);

CREATE INDEX IF NOT EXISTS idx_reading_attempts_session 
ON reading_practice_attempts(session_id);

-- ============================================================================
-- LEARNING SESSIONS
-- ============================================================================

-- Table: learning_sessions
-- Track overall learning sessions
CREATE TABLE IF NOT EXISTS learning_sessions (
    session_id TEXT PRIMARY KEY, -- UUID
    user_id INTEGER DEFAULT 1,
    session_type TEXT CHECK(session_type IN ('free_talk', 'scenario', 'classic_dialogue', 'reading', 'review', 'assistant')),
    sceneid INTEGER,
    scene_dialogue_id INTEGER,
    classic_clip_id INTEGER,
    started_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    ended_at INTEGER,
    duration_seconds INTEGER,
    
    -- Performance metrics
    total_words_spoken INTEGER DEFAULT 0,
    average_wpm REAL,
    error_count INTEGER DEFAULT 0,
    correction_count INTEGER DEFAULT 0,
    
    -- Session notes
    notes TEXT,
    ai_summary_en TEXT,
    ai_summary_zh TEXT,
    
    FOREIGN KEY (sceneid) REFERENCES scenes(id) ON DELETE SET NULL,
    FOREIGN KEY (scene_dialogue_id) REFERENCES scene_dialogues(id) ON DELETE SET NULL,
    FOREIGN KEY (classic_clip_id) REFERENCES classic_dialogue_clips(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_started 
ON learning_sessions(started_at DESC);

CREATE INDEX IF NOT EXISTS idx_sessions_type 
ON learning_sessions(session_type, started_at);

-- ============================================================================
-- USER PROGRESS & STATS
-- ============================================================================

-- Table: user_achievements
-- Track user achievements and milestones
CREATE TABLE IF NOT EXISTS user_achievements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER DEFAULT 1,
    achievement_type TEXT NOT NULL,
    achievement_name TEXT NOT NULL,
    description_en TEXT,
    description_zh TEXT,
    earned_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    metadata TEXT, -- JSON for additional data
    
    UNIQUE(user_id, achievement_type, achievement_name)
);

CREATE INDEX IF NOT EXISTS idx_achievements_user 
ON user_achievements(user_id, earned_at DESC);

-- Table: daily_stats
-- Daily learning statistics
CREATE TABLE IF NOT EXISTS daily_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER DEFAULT 1,
    stat_date TEXT NOT NULL, -- YYYY-MM-DD format
    minutes_studied INTEGER DEFAULT 0,
    words_practiced INTEGER DEFAULT 0,
    sessions_completed INTEGER DEFAULT 0,
    errors_corrected INTEGER DEFAULT 0,
    new_words_learned INTEGER DEFAULT 0,
    review_words_count INTEGER DEFAULT 0,
    
    UNIQUE(user_id, stat_date)
);

CREATE INDEX IF NOT EXISTS idx_daily_stats_date 
ON daily_stats(stat_date DESC);

-- ============================================================================
-- VOCABULARY & PHRASES
-- ============================================================================

-- Table: key_phrases
-- Important phrases and expressions
CREATE TABLE IF NOT EXISTS key_phrases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    phrase_en TEXT NOT NULL,
    phrase_zh TEXT NOT NULL,
    phonetic_transcription TEXT,
    usage_context TEXT,
    example_sentence_en TEXT,
    example_sentence_zh TEXT,
    category TEXT, -- 'greeting', 'request', 'opinion', etc.
    formality_level TEXT CHECK(formality_level IN ('casual', 'neutral', 'formal')),
    frequency_score INTEGER DEFAULT 0, -- How often encountered
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    
    UNIQUE(phrase_en)
);

CREATE INDEX IF NOT EXISTS idx_key_phrases_category 
ON key_phrases(category);

-- Table: user_vocabulary
-- Track user's vocabulary mastery
CREATE TABLE IF NOT EXISTS user_vocabulary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER DEFAULT 1,
    word TEXT NOT NULL,
    word_zh TEXT,
    mastery_level INTEGER DEFAULT 1 CHECK(mastery_level BETWEEN 1 AND 5),
    first_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_practiced_at INTEGER,
    practice_count INTEGER DEFAULT 0,
    correct_count INTEGER DEFAULT 0,
    next_review_at INTEGER,
    
    UNIQUE(user_id, word)
);

CREATE INDEX IF NOT EXISTS idx_user_vocab_review 
ON user_vocabulary(next_review_at) WHERE next_review_at IS NOT NULL;

-- ============================================================================
-- REAL-TIME ASSISTANT DATA
-- ============================================================================

-- Table: assistant_conversations
-- Track real-time assistant usage
CREATE TABLE IF NOT EXISTS assistant_conversations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    external_app TEXT, -- 'whatsapp', 'wechat', etc.
    started_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    ended_at INTEGER,
    message_count INTEGER DEFAULT 0,
    ai_suggestions_count INTEGER DEFAULT 0,
    grammar_corrections_count INTEGER DEFAULT 0,
    translations_count INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_assistant_sessions 
ON assistant_conversations(started_at DESC);

-- Table: assistant_suggestions
-- Log AI suggestions made during real-time assistance
CREATE TABLE IF NOT EXISTS assistant_suggestions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id INTEGER NOT NULL,
    suggestion_type TEXT CHECK(suggestion_type IN ('response', 'correction', 'translation', 'vocabulary')),
    suggested_text TEXT NOT NULL,
    was_accepted INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    
    FOREIGN KEY (conversation_id) REFERENCES assistant_conversations(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_assistant_suggestions_conversation 
ON assistant_suggestions(conversation_id, created_at);
