//! Home Screen - MainBody and learning hub
//!
//! Layout based on design/kiro/01-首页-sketch.svg:
//! - Welcome section with greeting and "开始对话" button
//! - AI suggestion bubble
//! - Today's tasks with progress bar
//! - Quick actions grid
//! - Stats panel (学习数据)
//! - AI insights section
//! - Recommended scenes

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_component::*;

    use colang_widgets::theme::*;

    // ========================================================================
    // Design Tokens from kiro/01-首页-sketch.svg
    // ========================================================================

    // Card styling with shadow effect
    CardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }
    }

    // Panel styling (secondary background)
    PanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }
    }

    // Section title styling
    SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Body text styling
    BodyText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    // Muted text styling
    MutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
            }
        }
    }

    // ========================================================================
    // Stat Card Component
    // ========================================================================

    StatCard = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 4
        align: {x: 0.5}

        stat_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_INDIGO), (INDIGO_400), self.dark_mode);
                }
            }
        }

        stat_label = <MutedText> {}
    }

    // Success colored stat
    StatCardSuccess = <StatCard> {
        stat_value = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_GREEN), (ACCENT_GREEN), self.dark_mode);
                }
            }
        }
    }

    // Warning colored stat
    StatCardWarning = <StatCard> {
        stat_value = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return mix((AMBER_500), (AMBER_500), self.dark_mode);
                }
            }
        }
    }

    // ========================================================================
    // Task Item Component
    // ========================================================================

    TaskItem = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 4

        task_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        task_status = <MutedText> {}
    }

    // ========================================================================
    // Quick Action Button
    // ========================================================================

    QuickActionButton = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 6
        align: {x: 0.5}
        cursor: Hand

        action_panel = <PanelBase> {
            width: Fill, height: 60
            align: {x: 0.5, y: 0.5}

            action_icon = <Label> {
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 20.0 }
                    color: (TEXT_PRIMARY)
                }
            }
        }

        action_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ========================================================================
    // Insight Card Component
    // ========================================================================

    InsightCard = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 6

        insight_text = <BodyText> {}
        insight_action = <MutedText> {}
    }

    // ========================================================================
    // Scene Card Component
    // ========================================================================

    SceneCard = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 6
        align: {x: 0.5}
        cursor: Hand

        sceneicon = <Label> {
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 24.0 }
                color: (TEXT_PRIMARY)
            }
        }

        scenetitle = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        scenesubtitle = <MutedText> {}
    }

    // ========================================================================
    // Progress Bar Component
    // ========================================================================

    ProgressBar = <View> {
        width: Fill, height: 6
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.6
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background track
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                let track_color = mix((SLATE_200), (SLATE_600), self.dark_mode);
                sdf.fill(track_color);

                // Progress fill
                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 3.0);
                sdf.fill((ACCENT_INDIGO));

                return sdf.result;
            }
        }
    }

    // ========================================================================
    // Welcome Card with Gradient
    // ========================================================================

    WelcomeCard = <CardBase> {
        width: Fill, height: Fit
        padding: 20
        flow: Right
        spacing: 16

        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                // Light gradient tint
                return mix(vec4(0.937, 0.941, 0.996, 1.0), (SLATE_800), self.dark_mode);
            }
        }

        // Left content
        welcome_content = <View> {
            width: Fit, height: Fit
            flow: Down
            spacing: 8

            greeting = <Label> {
                text: "👋 早上好，用户！"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            subtitle = <BodyText> {
                text: "今天想聊点什么？AI 已经准备好陪你练习了"
            }

            start_button = <Button> {
                text: "开始对话 →"
                margin: {top: 8}
                padding: {left: 16, right: 16, top: 10, bottom: 10}

                draw_bg: {
                    border_radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill((ACCENT_INDIGO));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    color: (WHITE)
                }
            }
        }

        <View> { width: Fill }

        select_scene_btn = <Button> {
            width: Fit, height: Fit
            padding: { left: 12, right: 12, top: 8, bottom: 8 }
            text: "选择场景"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_INDIGO), (INDIGO_300), self.dark_mode);
                }
            }
            draw_bg: {
                instance dark_mode: 0.0
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let r = 12.0;
                    let light = mix((INDIGO_100), (SLATE_200), self.hover);
                    let dark = mix((INDIGO_900), (SLATE_700), self.hover);
                    let color = mix(light, dark, self.dark_mode);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, r);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
        }

        // AI Suggestion bubble
        ai_suggestion = <PanelBase> {
            width: 240, height: Fit
            padding: 12
            flow: Down
            spacing: 6

            suggestion_header = <Label> {
                text: "💡 AI 建议"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                    color: (ACCENT_INDIGO)
                }
            }

            suggestion_text = <BodyText> {
                width: Fill
                text: "\"昨天我们聊了旅行，今天继续练习酒店预订怎么样？\""
            }
        }
    }

    // ========================================================================
    // Today's Tasks Section
    // ========================================================================

    TodayTasksCard = <CardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        // Header row
        tasks_header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            <SectionTitle> { text: "📋 今日任务" }
            <View> { width: Fill }
            tasks_progress_label = <MutedText> { text: "3/5 已完成" }
        }

        // Progress bar
        tasks_progress_bar = <ProgressBar> {
            draw_bg: { progress: 0.6 }
        }

        // Task items grid (2 columns)
        tasks_grid = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12

            tasks_col1 = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                task1 = <TaskItem> {
                    task_title = { text: "✅ 3分钟自由对话" }
                    task_status = { text: "已完成" }
                }

                task3 = <TaskItem> {
                    task_title = { text: "⭕ 复习 8 个易错点" }
                    task_status = { text: "待完成" }
                }
            }

            tasks_col2 = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                task2 = <TaskItem> {
                    task_title = { text: "⏳ 场景练习：点餐" }
                    task_status = { text: "进行中..." }
                }

                task4 = <TaskItem> {
                    task_title = { text: "⭕ 跟读训练 5 句" }
                    task_status = { text: "待完成" }
                }
            }
        }
    }

    // ========================================================================
    // Quick Actions Section
    // ========================================================================

    QuickActionsCard = <CardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        <SectionTitle> { text: "⚡ 快捷入口" }

        actions_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12

            action_scenario = <QuickActionButton> {
                action_panel = {
                    action_icon = { text: "🎭" }
                }
                action_label = { text: "场景模拟" }
            }

            action_dialogue = <QuickActionButton> {
                action_panel = {
                    action_icon = { text: "🎬" }
                }
                action_label = { text: "经典对白" }
            }

            action_reading = <QuickActionButton> {
                action_panel = {
                    action_icon = { text: "🎤" }
                }
                action_label = { text: "跟读练习" }
            }

            action_assistant = <QuickActionButton> {
                action_panel = {
                    action_icon = { text: "🤝" }
                }
                action_label = { text: "实时助手" }
            }
        }
    }

    // ========================================================================
    // Stats Section
    // ========================================================================

    StatsCard = <CardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        <SectionTitle> { text: "📊 学习数据" }

        // Stats grid
        stats_grid = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8

            stat_minutes = <StatCard> {
                stat_value = { text: "47" }
                stat_label = { text: "本周对话(分钟)" }
            }

            stat_vocab = <StatCardSuccess> {
                stat_value = { text: "156" }
                stat_label = { text: "已掌握词汇" }
            }

            stat_review = <StatCardWarning> {
                stat_value = { text: "23" }
                stat_label = { text: "待复习" }
            }

            stat_level = <StatCard> {
                stat_value = { text: "B1" }
                stat_label = { text: "当前水平" }
            }
        }

        // Weekly chart placeholder
        chart_placeholder = <PanelBase> {
            width: Fill, height: 40
            padding: {left: 12}
            align: {y: 0.5}

            <MutedText> { text: "本周学习时长分布 ▓▓▓░▓▓▓" }
        }
    }

    // ========================================================================
    // AI Insights Section
    // ========================================================================

    InsightsCard = <CardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        <SectionTitle> { text: "🧠 AI 洞察" }

        insight_positive = <InsightCard> {
            insight_text = { text: "💡 你的冠词使用进步明显！a/an 错误率下降 40%" }
            insight_action = {
                text: "持续保持"
                draw_text: { color: (ACCENT_GREEN) }
            }
        }

        insight_suggestion = <InsightCard> {
            insight_text = { text: "⚠️ 建议多练习过去时态，这是你目前的薄弱点" }
            insight_action = {
                text: "点击开始专项练习 →"
                draw_text: { color: (AMBER_500) }
            }
        }
    }

    // ========================================================================
    // Recommended Scenes Section
    // ========================================================================

    ScenesCard = <CardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        <SectionTitle> { text: "🎯 推荐场景" }

        scenes_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12

            scenehotel = <SceneCard> {
                sceneicon = { text: "🏨" }
                scenetitle = { text: "酒店入住" }
                scenesubtitle = { text: "继续上次" }
            }

            scenerestaurant = <SceneCard> {
                sceneicon = { text: "🍽️" }
                scenetitle = { text: "餐厅点餐" }
                scenesubtitle = { text: "新场景" }
            }

            sceneinterview = <SceneCard> {
                sceneicon = { text: "💼" }
                scenetitle = { text: "工作面试" }
                scenesubtitle = { text: "挑战" }
            }
        }
    }

    // ========================================================================
    // Main Home Screen
    // ========================================================================

    pub HomeScreen = {{HomeScreen}} {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Scrollable content
        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Right
                padding: 16
                spacing: 16

                // Left column (main content)
                left_column = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 16

                    welcome_card = <WelcomeCard> {}
                    tasks_card = <TodayTasksCard> {}
                    actions_card = <QuickActionsCard> {}
                }

                // Right column (stats and insights)
                right_column = <View> {
                    width: 400, height: Fit
                    flow: Down
                    spacing: 16

                    stats_card = <StatsCard> {}
                    insights_card = <InsightsCard> {}
                    scenes_card = <ScenesCard> {}
                }
            }
        }
    }
}

/// HomeScreen widget
#[derive(Live, LiveHook, Widget)]
pub struct HomeScreen {
    #[deref]
    view: View,
}

impl Widget for HomeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for HomeScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Handle button click - navigate to conversation
        if self
            .button(ids!(welcome_card.welcome_content.start_button))
            .clicked(actions)
        {
            // TODO: Emit action to navigate to AI conversation screen
        }

        // Handle quick action clicks - finger_up returns an Option
        if self
            .view(ids!(actions_card.actions_row.action_scenario.action_panel))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to scenario simulation
        }
        if self
            .view(ids!(actions_card.actions_row.action_dialogue.action_panel))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to classic dialogues
        }
        if self
            .view(ids!(actions_card.actions_row.action_reading.action_panel))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to reading practice
        }
        if self
            .view(ids!(actions_card.actions_row.action_assistant.action_panel))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to real-time assistant
        }

        // Handle scenario card clicks
        if self
            .view(ids!(scenes_card.scenes_row.scenehotel))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to hotel scenario
        }
        if self
            .view(ids!(scenes_card.scenes_row.scenerestaurant))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to restaurant scenario
        }
        if self
            .view(ids!(scenes_card.scenes_row.sceneinterview))
            .finger_up(actions)
            .is_some()
        {
            // TODO: Navigate to interview scenario
        }
    }
}

impl HomeScreen {
    /// Apply dark mode to all components
    pub fn apply_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        // Apply to main background
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );

        // Apply to all cards recursively
        self.apply_dark_mode_recursive(cx, dark_mode);
    }

    fn apply_dark_mode_recursive(&mut self, cx: &mut Cx, dark_mode: f64) {
        // Apply dark mode to draw_bg instances
        let dark_mode_update = live! {
            draw_bg: { dark_mode: (dark_mode) }
        };

        // Apply to major sections
        self.view(ids!(content_scroll.content.left_column.welcome_card))
            .apply_over(cx, dark_mode_update);
        self.view(ids!(content_scroll.content.left_column.tasks_card))
            .apply_over(cx, dark_mode_update);
        self.view(ids!(content_scroll.content.left_column.actions_card))
            .apply_over(cx, dark_mode_update);
        self.view(ids!(content_scroll.content.right_column.stats_card))
            .apply_over(cx, dark_mode_update);
        self.view(ids!(content_scroll.content.right_column.insights_card))
            .apply_over(cx, dark_mode_update);
        self.view(ids!(content_scroll.content.right_column.scenes_card))
            .apply_over(cx, dark_mode_update);
    }
}
