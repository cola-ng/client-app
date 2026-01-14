-- Sample Data for English Learning Application
-- Version: 001
-- Date: 2026-01-14
-- Description: Realistic sample data to populate the database

BEGIN TRANSACTION;

-- ============================================================================
-- SCENARIOS DATA
-- ============================================================================

INSERT INTO scenarios (name_en, name_zh, description_en, description_zh, icon_emoji, difficulty_level, category, display_order) VALUES
('Airport Check-in', '机场办理登机', 'Practice conversations at airport check-in counters', '练习机场值机柜台对话', '✈️', 'intermediate', 'travel', 1),
('Hotel Reservation', '酒店预订', 'Learn how to book and manage hotel reservations', '学习如何预订和管理酒店', '🏨', 'beginner', 'travel', 2),
('Restaurant Ordering', '餐厅点餐', 'Practice ordering food and drinks in restaurants', '练习在餐厅点餐', '🍽️', 'beginner', 'daily', 3),
('Job Interview', '工作面试', 'Prepare for professional job interviews', '准备专业的工作面试', '💼', 'advanced', 'business', 4),
('Doctor Appointment', '看病就医', 'Learn medical vocabulary and describe symptoms', '学习医疗词汇和描述症状', '🏥', 'intermediate', 'daily', 5),
('Shopping', '购物', 'Practice shopping conversations and negotiations', '练习购物对话和讨价还价', '🛍️', 'beginner', 'daily', 6),
('Meeting Introduction', '会议介绍', 'Professional self-introduction in business meetings', '商务会议中的专业自我介绍', '🤝', 'intermediate', 'business', 7),
('Phone Call', '电话沟通', 'Handle phone conversations professionally', '专业地处理电话对话', '📞', 'intermediate', 'business', 8);

-- ============================================================================
-- SCENE DIALOGUES
-- ============================================================================

INSERT INTO scene_dialogues (scenario_id, title_en, title_zh, description_en, description_zh, total_turns, estimated_duration_seconds, difficulty_level) VALUES
(1, 'Checking in at the Counter', '在柜台办理登机', 'Basic airport check-in procedure', '基本的机场值机流程', 10, 120, 'intermediate'),
(2, 'Making a Reservation', '预订房间', 'Call hotel to make a reservation', '致电酒店预订房间', 8, 90, 'beginner'),
(3, 'Ordering Dinner', '晚餐点餐', 'Order a meal at a restaurant', '在餐厅点餐', 12, 150, 'beginner'),
(4, 'Initial Interview Questions', '初步面试问题', 'Common job interview questions', '常见的工作面试问题', 15, 300, 'advanced'),
(5, 'Describing Symptoms', '描述症状', 'Explain health problems to a doctor', '向医生解释健康问题', 10, 180, 'intermediate');

-- ============================================================================
-- DIALOGUE TURNS
-- ============================================================================

-- Airport Check-in Dialogue
INSERT INTO dialogue_turns (scene_dialogue_id, turn_number, speaker_role, speaker_name, content_en, content_zh, phonetic_transcription) VALUES
(1, 1, 'npc', 'Agent', 'Good morning! May I see your passport and ticket, please?', '早上好！请出示您的护照和机票，好吗？', 'ɡʊd ˈmɔrnɪŋ meɪ aɪ si jʊr ˈpæspɔrt ənd ˈtɪkɪt pliz'),
(1, 2, 'user', 'Traveler', 'Here you go.', '给你。', 'hɪr ju ɡoʊ'),
(1, 3, 'npc', 'Agent', 'Thank you. Are you checking any bags today?', '谢谢。您今天有行李要托运吗？', 'θæŋk ju ɑr ju ˈtʃɛkɪŋ ˈɛni bæɡz təˈdeɪ'),
(1, 4, 'user', 'Traveler', 'Yes, I have one suitcase to check.', '是的，我有一个行李箱要托运。', 'jɛs aɪ həv wʌn ˈsutˌkeɪs tu tʃɛk'),
(1, 5, 'npc', 'Agent', 'Please place it on the scale. ... It''s 22 kilos. That''s within the limit.', '请把它放在秤上。...22公斤。在限制范围内。', 'pliz pleɪs ɪt ɑn ðə skeɪl ɪts twɛnti tu ˈkiloʊz ðæts wɪˈðɪn ðə ˈlɪmɪt');

-- Hotel Reservation Dialogue
INSERT INTO dialogue_turns (scene_dialogue_id, turn_number, speaker_role, speaker_name, content_en, content_zh, phonetic_transcription) VALUES
(2, 1, 'npc', 'Receptionist', 'Good afternoon, Grand Hotel. How may I help you?', '下午好，格兰德酒店。有什么可以帮您的吗？', 'ɡʊd ˌæftərˈnun ɡrænd hoʊˈtɛl haʊ meɪ aɪ hɛlp ju'),
(2, 2, 'user', 'Guest', 'Hi, I''d like to make a reservation for next weekend.', '您好，我想预订下周末的房间。', 'haɪ aɪd laɪk tu meɪk ə ˌrɛzərˈveɪʃən fɔr nɛkst ˈwikˌɛnd'),
(2, 3, 'npc', 'Receptionist', 'Certainly! For what dates would you like to book?', '当然！您想预订哪几天？', 'ˈsɜrtənli fɔr wʌt deɪts wʊd ju laɪk tu bʊk'),
(2, 4, 'user', 'Guest', 'I need a room for two nights, checking in on Friday the 20th.', '我需要一个房间住两晚，20号星期五入住。', 'aɪ nid ə rum fɔr tu naɪts ˈtʃɛkɪŋ ɪn ɑn ˈfraɪdeɪ ðə ˈtwɛntiəθ');

-- Restaurant Ordering Dialogue
INSERT INTO dialogue_turns (scene_dialogue_id, turn_number, speaker_role, speaker_name, content_en, content_zh, phonetic_transcription) VALUES
(3, 1, 'npc', 'Waiter', 'Good evening! Are you ready to order?', '晚上好！您准备好点餐了吗？', 'ɡʊd ˈivnɪŋ ɑr ju ˈrɛdi tu ˈɔrdər'),
(3, 2, 'user', 'Customer', 'Yes, I''ll have the grilled salmon, please.', '是的，我要烤三文鱼，谢谢。', 'jɛs aɪl həv ðə ɡrɪld ˈsæmən pliz'),
(3, 3, 'npc', 'Waiter', 'Excellent choice! How would you like it cooked?', '很好的选择！您想要几成熟？', 'ˈɛksələnt tʃɔɪs haʊ wʊd ju laɪk ɪt kʊkt'),
(3, 4, 'user', 'Customer', 'Medium, please. And could I get a side salad?', '中等，谢谢。我可以要一份配菜沙拉吗？', 'ˈmidiəm pliz ənd kʊd aɪ ɡɛt ə saɪd ˈsæləd');

-- ============================================================================
-- CLASSIC DIALOGUE SOURCES
-- ============================================================================

INSERT INTO classic_dialogue_sources (source_type, title, year, description_en, description_zh, difficulty_level) VALUES
('movie', 'The Pursuit of Happyness', 2006, 'Inspirational story about perseverance and fatherhood', '关于毅力和父爱的励志故事', 'intermediate'),
('movie', 'The Social Network', 2010, 'Story of Facebook''s founding', 'Facebook创立的故事', 'advanced'),
('tv_show', 'Friends', 1994, 'Classic sitcom about six friends in New York', '关于纽约六个朋友的经典情景喜剧', 'beginner'),
('tv_show', 'The Office', 2005, 'Mockumentary about office workers', '关于办公室员工的伪纪录片', 'intermediate'),
('ted_talk', 'How Great Leaders Inspire Action', 2009, 'Simon Sinek''s famous TED talk', 'Simon Sinek的著名TED演讲', 'advanced'),
('ted_talk', 'The Power of Vulnerability', 2010, 'Brené Brown on human connection', 'Brené Brown关于人际联系的演讲', 'intermediate');

-- ============================================================================
-- CLASSIC DIALOGUE CLIPS
-- ============================================================================

INSERT INTO classic_dialogue_clips (source_id, clip_title_en, clip_title_zh, start_time_seconds, end_time_seconds, transcript_en, transcript_zh, difficulty_vocab, difficulty_speed, difficulty_slang, popularity_score) VALUES
(1, 'Job Interview Scene', '工作面试场景', 3420, 3540, 'What would you say if a man walked in here with no shirt, and I hired him? What would you say?', '如果一个没穿衬衫的人走进来，我雇用了他，你会说什么？', 3, 3, 2, 95),
(3, 'Coffee Shop Introduction', '咖啡馆介绍', 120, 180, 'Hi, I''m Chandler. I make jokes when I''m uncomfortable.', '嗨，我是钱德勒。我不舒服的时候会开玩笑。', 2, 2, 1, 100),
(4, 'Office Meeting', '办公室会议', 890, 950, 'That''s what she said!', '她就是这么说的！', 1, 2, 3, 88),
(5, 'Golden Circle', '黄金圈理论', 180, 280, 'People don''t buy what you do; they buy why you do it.', '人们不是买你做什么；他们买的是你为什么做。', 4, 3, 1, 92);

-- ============================================================================
-- READING EXERCISES
-- ============================================================================

INSERT INTO reading_exercises (title_en, title_zh, description_en, description_zh, difficulty_level, exercise_type) VALUES
('Common Phrases Practice', '常用短语练习', 'Practice everyday common phrases', '练习日常常用短语', 'beginner', 'sentence'),
('Business English', '商务英语', 'Professional business expressions', '专业商务表达', 'advanced', 'sentence'),
('Tongue Twisters', '绕口令', 'Improve pronunciation with tongue twisters', '通过绕口令改善发音', 'intermediate', 'tongue_twister'),
('Travel Conversations', '旅行对话', 'Useful phrases for travelers', '旅行者实用短语', 'beginner', 'dialogue');

-- ============================================================================
-- READING SENTENCES
-- ============================================================================

INSERT INTO reading_sentences (exercise_id, sentence_order, content_en, content_zh, phonetic_transcription, focus_sounds) VALUES
(1, 1, 'Could you please help me with this?', '你能帮我一下吗？', 'kʊd ju pliz hɛlp mi wɪð ðɪs', '["th", "h", "w"]'),
(1, 2, 'I would like to make a reservation.', '我想预订。', 'aɪ wʊd laɪk tu meɪk ə ˌrɛzərˈveɪʃən', '["v", "r", "ʃ"]'),
(1, 3, 'How much does this cost?', '这个多少钱？', 'haʊ mʌtʃ dʌz ðɪs kɔst', '["h", "ch", "st"]'),
(2, 1, 'We need to schedule a meeting to discuss the quarterly results.', '我们需要安排一次会议讨论季度业绩。', 'wi nid tu ˈskɛdʒul ə ˈmitɪŋ tu dɪˈskʌs ðə ˈkwɔrtərli rɪˈzʌlts', '["sk", "j", "kw"]'),
(2, 2, 'I will send you the proposal by end of business today.', '我今天下班前会把提案发给你。', 'aɪ wɪl sɛnd ju ðə prəˈpoʊzəl baɪ ɛnd əv ˈbɪznəs təˈdeɪ', '["pr", "z", "nd"]'),
(3, 1, 'She sells seashells by the seashore.', '她在海边卖贝壳。', 'ʃi sɛlz ˈsiʃɛlz baɪ ðə ˈsiʃɔr', '["s", "sh", "z"]'),
(3, 2, 'Peter Piper picked a peck of pickled peppers.', '彼得·派珀摘了一配克腌辣椒。', 'ˈpitər ˈpaɪpər pɪkt ə pɛk əv ˈpɪkəld ˈpɛpərz', '["p", "k", "d"]');

-- ============================================================================
-- ISSUE WORDS (Sample problematic words)
-- ============================================================================

INSERT INTO issue_words (word, issue_type, description_en, description_zh, difficulty_level, pick_count, context) VALUES
('through', 'pronunciation', 'Difficulty with "th" sound', '"th"音发音困难', 3, 5, 'I walked through the park.'),
('receipt', 'pronunciation', 'Silent "p" often mispronounced', '经常错误发音不发音的"p"', 2, 3, 'May I have a receipt?'),
('schedule', 'pronunciation', 'Different pronunciation in US vs UK', '美式和英式发音不同', 2, 4, 'What''s your schedule?'),
('although', 'usage', 'Confused with "though" and "even though"', '与"though"和"even though"混淆', 3, 2, 'Although it was raining, we went out.'),
('affect', 'usage', 'Often confused with "effect"', '经常与"effect"混淆', 4, 6, 'How will this affect our plans?'),
('their', 'grammar', 'Confused with "there" and "they''re"', '与"there"和"they''re"混淆', 2, 8, 'Where is their house?'),
('comfortable', 'pronunciation', 'Many syllables often mispronounced', '多音节经常发错音', 3, 4, 'This chair is very comfortable.'),
('literally', 'usage', 'Often misused for emphasis', '经常误用以示强调', 3, 3, 'It literally took five minutes.');

-- ============================================================================
-- SAMPLE CONVERSATIONS (Recent history)
-- ============================================================================

INSERT INTO conversations (session_id, speaker, use_lang, content_en, content_zh, words_per_minute) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'teacher', 'en', 'Good morning! How are you today?', '早上好！你今天怎么样？', 145),
('550e8400-e29b-41d4-a716-446655440001', 'user', 'en', 'I''m good, thank you. How about you?', '我很好，谢谢。你呢？', 120),
('550e8400-e29b-41d4-a716-446655440001', 'teacher', 'en', 'I''m great! Let''s practice talking about your weekend plans.', '我很好！让我们练习谈论你的周末计划。', 150),
('550e8400-e29b-41d4-a716-446655440001', 'user', 'en', 'This weekend, I plan to visit my friend.', '这个周末，我计划去看望我的朋友。', 115);

-- ============================================================================
-- LEARNING SESSIONS (Recent sessions)
-- ============================================================================

INSERT INTO learning_sessions (session_id, session_type, scenario_id, started_at, ended_at, duration_seconds, total_words_spoken, average_wpm, error_count, correction_count) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'free_talk', NULL, strftime('%s', 'now', '-2 days'), strftime('%s', 'now', '-2 days', '+25 minutes'), 1500, 450, 120, 8, 6),
('550e8400-e29b-41d4-a716-446655440002', 'scenario', 3, strftime('%s', 'now', '-1 day'), strftime('%s', 'now', '-1 day', '+15 minutes'), 900, 280, 125, 5, 4),
('550e8400-e29b-41d4-a716-446655440003', 'reading', NULL, strftime('%s', 'now', '-3 hours'), strftime('%s', 'now', '-3 hours', '+10 minutes'), 600, 150, 90, 3, 3);

-- ============================================================================
-- DAILY STATS (Last 7 days)
-- ============================================================================

INSERT INTO daily_stats (stat_date, minutes_studied, words_practiced, sessions_completed, errors_corrected, new_words_learned, review_words_count) VALUES
(date('now', '-6 days'), 35, 45, 2, 8, 6, 3),
(date('now', '-5 days'), 28, 38, 2, 6, 4, 5),
(date('now', '-4 days'), 42, 52, 3, 10, 8, 4),
(date('now', '-3 days'), 30, 40, 2, 7, 5, 6),
(date('now', '-2 days'), 25, 35, 2, 5, 3, 8),
(date('now', '-1 day'), 38, 48, 3, 9, 7, 5),
(date('now'), 15, 20, 1, 3, 2, 2);

-- ============================================================================
-- USER ACHIEVEMENTS
-- ============================================================================

INSERT INTO user_achievements (achievement_type, achievement_name, description_en, description_zh) VALUES
('milestone', 'First Conversation', 'Completed your first AI conversation', '完成第一次AI对话'),
('milestone', '7 Day Streak', 'Practiced for 7 consecutive days', '连续练习7天'),
('milestone', '100 Words Mastered', 'Successfully mastered 100 vocabulary words', '成功掌握100个词汇'),
('skill', 'Restaurant Expert', 'Completed all restaurant scenarios', '完成所有餐厅场景'),
('challenge', 'Tongue Twister Master', 'Successfully completed 10 tongue twisters', '成功完成10个绕口令');

-- ============================================================================
-- KEY PHRASES (Common expressions)
-- ============================================================================

INSERT INTO key_phrases (phrase_en, phrase_zh, usage_context, example_sentence_en, example_sentence_zh, category, formality_level, frequency_score) VALUES
('How''s it going?', '最近怎么样？', 'Casual greeting', 'Hey John, how''s it going?', '嘿约翰，最近怎么样？', 'greeting', 'casual', 95),
('I''d like to...', '我想要...', 'Polite request', 'I''d like to make a reservation.', '我想要预订。', 'request', 'formal', 90),
('Could you please...?', '你能请...吗？', 'Polite request', 'Could you please pass the salt?', '你能请把盐递给我吗？', 'request', 'formal', 88),
('To be honest', '说实话', 'Expressing opinion', 'To be honest, I don''t agree.', '说实话，我不同意。', 'opinion', 'neutral', 75),
('Let me know', '让我知道', 'Requesting feedback', 'Let me know when you''re ready.', '准备好了告诉我。', 'request', 'neutral', 92),
('Thanks a bunch', '非常感谢', 'Expressing gratitude', 'Thanks a bunch for your help!', '非常感谢你的帮助！', 'gratitude', 'casual', 70);

-- ============================================================================
-- USER VOCABULARY (Sample mastered words)
-- ============================================================================

INSERT INTO user_vocabulary (word, word_zh, mastery_level, practice_count, correct_count, last_practiced_at, next_review_at) VALUES
('hello', '你好', 5, 20, 20, strftime('%s', 'now', '-1 day'), strftime('%s', 'now', '+7 days')),
('restaurant', '餐厅', 4, 15, 14, strftime('%s', 'now', '-2 days'), strftime('%s', 'now', '+3 days')),
('reservation', '预订', 3, 10, 8, strftime('%s', 'now', '-3 days'), strftime('%s', 'now', '+1 day')),
('comfortable', '舒适的', 2, 5, 3, strftime('%s', 'now', '-1 hour'), strftime('%s', 'now', '+2 hours')),
('although', '虽然', 2, 6, 4, strftime('%s', 'now', '-2 hours'), strftime('%s', 'now', '+4 hours')),
('definitely', '肯定地', 3, 8, 7, strftime('%s', 'now', '-4 days'), strftime('%s', 'now', '+2 days'));

COMMIT;
