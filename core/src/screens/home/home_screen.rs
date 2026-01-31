//! Home Screen - Matching website LandingPage.tsx
//!
//! Layout (no login required):
//! - Welcome card with greeting, "开始对话" button, AI suggestion
//! - Stats card with learning data and chart
//! - Role-play stages grid (fetched from API)
//! - Today's tasks card
//! - AI insights card

use std::sync::mpsc;

use makepad_widgets::*;

use crate::asset_api::{Stage, get_asset_api};
use crate::learn_api::{get_learn_api, DailyStat};
use crate::models::Preferences;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // ========================================================================
    // Design Tokens matching website LandingPage
    // ========================================================================

    // Card styling - white with shadow
    CardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Shadow
                let shadow = vec4(0.0, 0.0, 0.0, 0.06);
                sdf.box(2.0, 4.0, self.rect_size.x - 4.0, self.rect_size.y - 2.0, self.border_radius);
                sdf.fill(shadow);
                // Card bg
                let light = vec4(1.0, 1.0, 1.0, 1.0);
                let dark = (SLATE_800);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(mix(light, dark, self.dark_mode));
                return sdf.result;
            }
        }
    }

    // Panel styling - gray-50 background
    PanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light = vec4(0.976, 0.980, 0.984, 1.0); // gray-50
                let dark = (SLATE_700);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(mix(light, dark, self.dark_mode));
                return sdf.result;
            }
        }
    }

    SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    BodyText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    MutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
            }
        }
    }

    // Primary button (orange)
    PrimaryButton = <Button> {
        width: Fit, height: 36
        padding: {left: 16, right: 16}
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance hover: 0.0
            border_radius: 6.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let base = vec4(0.976, 0.451, 0.086, 1.0);  // orange-500
                let hover_color = vec4(0.918, 0.345, 0.047, 1.0); // orange-600
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(mix(base, hover_color, self.hover));
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            color: (WHITE)
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
            down = {
                default: off
                off = { from: {all: Snap} apply: {} }
                on = { from: {all: Snap} apply: {} }
            }
        }
    }

    // Secondary button (outline)
    SecondaryButton = <Button> {
        width: Fit, height: 36
        padding: {left: 16, right: 16}
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            border_radius: 6.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light = vec4(1.0, 1.0, 1.0, 1.0);
                let dark = (SLATE_700);
                let hover_light = vec4(0.976, 0.980, 0.984, 1.0);
                let hover_dark = (SLATE_600);
                let bg = mix(
                    mix(light, dark, self.dark_mode),
                    mix(hover_light, hover_dark, self.dark_mode),
                    self.hover
                );
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                let border = mix(vec4(0.82, 0.835, 0.863, 1.0), (SLATE_600), self.dark_mode);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
            down = {
                default: off
                off = { from: {all: Snap} apply: {} }
                on = { from: {all: Snap} apply: {} }
            }
        }
    }

    // Stat card component
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
                    return mix((ACCENT_PRIMARY), (ORANGE_400), self.dark_mode);
                }
            }
        }

        stat_label = <MutedText> {}
    }

    StatCardGreen = <StatCard> {
        stat_value = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return (ACCENT_GREEN);
                }
            }
        }
    }

    StatCardAmber = <StatCard> {
        stat_value = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return (AMBER_500);
                }
            }
        }
    }

    StatCardBlue = <StatCard> {
        stat_value = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return #3b82f6;
                }
            }
        }
    }

    // Task item component
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

    // Stage card component - matching website StageCard
    StageCard = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 4
        align: {x: 0.5}
        cursor: Hand

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light = vec4(0.976, 0.980, 0.984, 1.0); // gray-50
                let light_hover = vec4(1.0, 0.969, 0.929, 1.0); // orange-50
                let dark = (SLATE_700);
                let dark_hover = (SLATE_600);
                let bg = mix(
                    mix(light, dark, self.dark_mode),
                    mix(light_hover, dark_hover, self.dark_mode),
                    self.hover
                );
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        stage_icon = <Label> {
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 28.0 }
            }
        }

        stage_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        stage_subtitle = <MutedText> {}
    }

    // Insight card with colored background
    InsightCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 4
        show_bg: true
        draw_bg: {
            instance bg_color: 0.0  // 0=green, 1=amber, 2=blue, 3=purple
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let green = vec4(0.941, 0.992, 0.957, 1.0);   // green-50
                let amber = vec4(1.0, 0.984, 0.922, 1.0);     // amber-50
                let blue = vec4(0.937, 0.965, 1.0, 1.0);      // blue-50
                let purple = vec4(0.969, 0.945, 1.0, 1.0);    // purple-50
                let bg = vec4(0.0);
                if self.bg_color < 0.5 { bg = green; }
                else if self.bg_color < 1.5 { bg = amber; }
                else if self.bg_color < 2.5 { bg = blue; }
                else { bg = purple; }
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        insight_text = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        insight_action = <Label> {
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                color: (ACCENT_GREEN)
            }
        }
    }

    // AI suggestion bubble
    AISuggestion = <RoundedView> {
        width: 220, height: Fit
        padding: 12
        flow: Down
        spacing: 4
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light = vec4(1.0, 0.969, 0.929, 1.0); // orange-50
                let dark = (SLATE_700);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(mix(light, dark, self.dark_mode));
                return sdf.result;
            }
        }

        header = <Label> {
            text: "AI 建议"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                color: (ACCENT_PRIMARY)
            }
        }

        content = <Label> {
            width: Fill
            text: "\"昨天我们聊了旅行，今天继续练习酒店预订怎么样？\""
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Progress bar
    ProgressBar = <View> {
        width: Fill, height: 8
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.6
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Background
                let track_light = vec4(0.898, 0.906, 0.922, 1.0); // gray-200
                let track_dark = (SLATE_600);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(mix(track_light, track_dark, self.dark_mode));
                // Progress
                let orange = vec4(0.976, 0.451, 0.086, 1.0);
                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                sdf.fill(orange);
                return sdf.result;
            }
        }
    }

    // ========================================================================
    // Welcome Card - matching website layout
    // ========================================================================

    WelcomeCard = <CardBase> {
        width: Fill, height: Fit
        padding: 24
        flow: Right
        spacing: 16

        // Left content
        welcome_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            greeting = <Label> {
                text: "早上好，欢迎来到开朗英语！"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            subtitle = <BodyText> {
                text: "开始你的英语学习之旅"
            }

            buttons_row = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8
                margin: {top: 8}

                start_chat_btn = <PrimaryButton> {
                    text: "开始对话 >"
                }

                select_scene_btn = <SecondaryButton> {
                    text: "选择场景"
                }
            }
        }

        // AI suggestion
        ai_suggestion = <AISuggestion> {}
    }

    // ========================================================================
    // Stats Card - Learning Data
    // ========================================================================

    StatsCard = <CardBase> {
        width: Fill, height: Fit
        padding: 24
        flow: Down
        spacing: 16

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            title = <SectionTitle> { text: "学习数据" }
            <View> { width: Fill }
            review_btn = <SecondaryButton> {
                text: "开始复习"
                padding: {left: 12, right: 12}
                height: 32
            }
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 16

            // Stats grid - 2x2
            stats_grid = <View> {
                width: 200, height: Fit
                flow: Down
                spacing: 8

                row1 = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8

                    stat_minutes = <StatCard> {
                        stat_value = { text: "0" }
                        stat_label = { text: "本周对话(分钟)" }
                    }
                    stat_vocab = <StatCardGreen> {
                        stat_value = { text: "0" }
                        stat_label = { text: "已掌握词汇" }
                    }
                }

                row2 = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8

                    stat_review = <StatCardAmber> {
                        stat_value = { text: "0" }
                        stat_label = { text: "待复习" }
                    }
                    stat_accuracy = <StatCardBlue> {
                        stat_value = { text: "85%" }
                        stat_label = { text: "正确率" }
                    }
                }
            }

            // Chart placeholder
            chart_area = <PanelBase> {
                width: Fill, height: 120
                padding: 12
                flow: Down
                spacing: 8

                chart_header = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    <MutedText> { text: "学习时长分布" }
                    <View> { width: Fill }

                    period_day = <Button> {
                        width: 28, height: 20
                        text: "日"
                        draw_bg: {
                            border_radius: 4.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                sdf.fill(vec4(0.976, 0.451, 0.086, 1.0));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 10.0 }
                            color: (WHITE)
                        }
                        animator: {
                            hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                            down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        }
                    }
                    period_week = <Button> {
                        width: 28, height: 20
                        text: "周"
                        draw_bg: {
                            border_radius: 4.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                sdf.fill(vec4(1.0, 1.0, 1.0, 1.0));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 10.0 }
                            color: (TEXT_MUTED)
                        }
                        animator: {
                            hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                            down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        }
                    }
                    period_month = <Button> {
                        width: 28, height: 20
                        text: "月"
                        draw_bg: {
                            border_radius: 4.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                sdf.fill(vec4(1.0, 1.0, 1.0, 1.0));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 10.0 }
                            color: (TEXT_MUTED)
                        }
                        animator: {
                            hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                            down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        }
                    }
                }

                chart_bars = <View> {
                    width: Fill, height: Fill
                    flow: Right
                    spacing: 4
                    align: {y: 1.0}
                    padding: {top: 8}

                    bar1 = <View> { width: Fill, height: 20, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                    bar2 = <View> { width: Fill, height: 35, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                    bar3 = <View> { width: Fill, height: 15, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                    bar4 = <View> { width: Fill, height: 45, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                    bar5 = <View> { width: Fill, height: 30, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                    bar6 = <View> { width: Fill, height: 50, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                    bar7 = <View> { width: Fill, height: 25, show_bg: true, draw_bg: { color: (ORANGE_400) } }
                }

                chart_labels = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 4

                    <MutedText> { width: Fill, text: "一", draw_text: { text_style: { font_size: 9.0 } } }
                    <MutedText> { width: Fill, text: "二", draw_text: { text_style: { font_size: 9.0 } } }
                    <MutedText> { width: Fill, text: "三", draw_text: { text_style: { font_size: 9.0 } } }
                    <MutedText> { width: Fill, text: "四", draw_text: { text_style: { font_size: 9.0 } } }
                    <MutedText> { width: Fill, text: "五", draw_text: { text_style: { font_size: 9.0 } } }
                    <MutedText> { width: Fill, text: "六", draw_text: { text_style: { font_size: 9.0 } } }
                    <MutedText> { width: Fill, text: "日", draw_text: { text_style: { font_size: 9.0 } } }
                }
            }
        }
    }

    // ========================================================================
    // Role-play Stages Card
    // ========================================================================

    StagesCard = <CardBase> {
        width: Fill, height: Fit
        padding: 24
        flow: Down
        spacing: 16

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            title = <SectionTitle> { text: "角色扮演" }
            <View> { width: Fill }
            view_all = <Label> {
                text: "查看全部 >"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    color: (ACCENT_PRIMARY)
                }
            }
        }

        // Grid of 6 stages (2 rows x 3 columns)
        stages_grid = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            row1 = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8

                stage0 = <StageCard> {
                    stage_icon = { text: "📚" }
                    stage_title = { text: "加载中..." }
                    stage_subtitle = { text: "" }
                }
                stage1 = <StageCard> {
                    stage_icon = { text: "📚" }
                    stage_title = { text: "加载中..." }
                    stage_subtitle = { text: "" }
                }
                stage2 = <StageCard> {
                    stage_icon = { text: "📚" }
                    stage_title = { text: "加载中..." }
                    stage_subtitle = { text: "" }
                }
            }

            row2 = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8

                stage3 = <StageCard> {
                    stage_icon = { text: "📚" }
                    stage_title = { text: "加载中..." }
                    stage_subtitle = { text: "" }
                }
                stage4 = <StageCard> {
                    stage_icon = { text: "📚" }
                    stage_title = { text: "加载中..." }
                    stage_subtitle = { text: "" }
                }
                stage5 = <StageCard> {
                    stage_icon = { text: "📚" }
                    stage_title = { text: "加载中..." }
                    stage_subtitle = { text: "" }
                }
            }
        }
    }

    // ========================================================================
    // Today's Tasks Card
    // ========================================================================

    TasksCard = <CardBase> {
        width: Fill, height: Fit
        padding: 24
        flow: Down
        spacing: 12

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            title = <SectionTitle> { text: "今日任务" }
            <View> { width: Fill }
            progress_label = <MutedText> { text: "3/5 已完成" }
        }

        progress_bar = <ProgressBar> {
            draw_bg: { progress: 0.6 }
        }

        tasks_list = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            task1 = <TaskItem> {
                task_title = { text: "✅ 3分钟自由对话" }
                task_status = { text: "已完成" }
            }
            task2 = <TaskItem> {
                task_title = { text: "⏳ 场景练习：点餐" }
                task_status = { text: "进行中..." }
            }
            task3 = <TaskItem> {
                task_title = { text: "⭕ 复习 8 个易错点" }
                task_status = { text: "待完成" }
            }
            task4 = <TaskItem> {
                task_title = { text: "⭕ 跟读训练 5 句" }
                task_status = { text: "待完成" }
            }
        }
    }

    // ========================================================================
    // AI Insights Card
    // ========================================================================

    InsightsCard = <CardBase> {
        width: Fill, height: Fit
        padding: 24
        flow: Down
        spacing: 12

        title = <SectionTitle> { text: "AI 洞察" }

        insights_list = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            insight1 = <InsightCard> {
                draw_bg: { bg_color: 0.0 }
                insight_text = { text: "你的冠词使用进步明显！a/an 错误率下降 40%" }
                insight_action = { text: "持续保持", draw_text: { color: (ACCENT_GREEN) } }
            }

            insight2 = <InsightCard> {
                draw_bg: { bg_color: 1.0 }
                insight_text = { text: "建议多练习过去时态，这是你目前的薄弱点" }
                insight_action = { text: "点击开始专项练习 >", draw_text: { color: (AMBER_500) } }
            }

            insight3 = <InsightCard> {
                draw_bg: { bg_color: 2.0 }
                insight_text = { text: "本周学习时长比上周提升 25%，继续加油！" }
                insight_action = { text: "稳步提升中", draw_text: { color: (ACCENT_PRIMARY) } }
            }

            insight4 = <InsightCard> {
                draw_bg: { bg_color: 3.0 }
                insight_text = { text: "尝试\"餐厅点餐\"场景，巩固已学的日常用语" }
                insight_action = { text: "立即体验 >", draw_text: { color: (ACCENT_PRIMARY) } }
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
                // Gradient matching website: from-orange-50 via-amber-50 to-yellow-50
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0);
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);

                let t = self.pos.x + self.pos.y * 0.3;
                let light_color = vec4(0.0);
                if t < 0.5 {
                    light_color = mix(orange_50, amber_50, t * 2.0);
                } else {
                    light_color = mix(amber_50, yellow_50, (t - 0.5) * 2.0);
                }
                return mix(light_color, dark_bg, self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Right
                padding: 16
                spacing: 16

                // Left column - main content
                left_column = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 16

                    welcome_card = <WelcomeCard> {}
                    stats_card = <StatsCard> {}
                    stages_card = <StagesCard> {}
                }

                // Right column - tasks and insights
                right_column = <View> {
                    width: 320, height: Fit
                    flow: Down
                    spacing: 16

                    tasks_card = <TasksCard> {}
                    insights_card = <InsightsCard> {}
                }
            }
        }
    }
}

/// Data fetch result types
enum FetchResult {
    Stages(Result<Vec<Stage>, String>),
    Stats(Result<Vec<DailyStat>, String>),
}

/// Actions emitted by HomeScreen
#[derive(Clone, Debug, DefaultNone)]
pub enum HomeScreenAction {
    None,
    NavigateToChat,
    NavigateToScenes,
    NavigateToReview,
    NavigateToStage { stage_id: i64 },
}

/// HomeScreen widget
#[derive(Live, LiveHook, Widget)]
pub struct HomeScreen {
    #[deref]
    view: View,

    #[rust]
    stages: Vec<Stage>,

    #[rust]
    data_loaded: bool,

    #[rust]
    fetch_rx: Option<mpsc::Receiver<FetchResult>>,
}

impl Widget for HomeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Load data on first draw
        if let Event::Draw(_) = event {
            if !self.data_loaded {
                self.data_loaded = true;
                self.load_data(cx);
                self.update_greeting(cx);
            }
        }

        // Check for fetch results - collect first to avoid borrow conflicts
        let results: Vec<_> = self.fetch_rx.as_ref()
            .map(|rx| rx.try_iter().collect())
            .unwrap_or_default();

        for result in results {
            match result {
                FetchResult::Stages(Ok(stages)) => {
                    self.stages = stages;
                    self.update_stage_cards(cx);
                    self.view.redraw(cx);
                }
                FetchResult::Stages(Err(e)) => {
                    eprintln!("Failed to fetch stages: {}", e);
                }
                FetchResult::Stats(Ok(stats)) => {
                    self.update_stats(cx, &stats);
                    self.view.redraw(cx);
                }
                FetchResult::Stats(Err(e)) => {
                    eprintln!("Failed to fetch stats: {}", e);
                }
            }
        }

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle button clicks
        if self.view.button(ids!(content_scroll.content.left_column.welcome_card.welcome_content.buttons_row.start_chat_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, HomeScreenAction::NavigateToChat);
        }

        if self.view.button(ids!(content_scroll.content.left_column.welcome_card.welcome_content.buttons_row.select_scene_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, HomeScreenAction::NavigateToScenes);
        }

        if self.view.button(ids!(content_scroll.content.left_column.stats_card.header.review_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, HomeScreenAction::NavigateToReview);
        }

        // Handle stage card clicks
        let stage_ids = [
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row1.stage0),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row1.stage1),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row1.stage2),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row2.stage3),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row2.stage4),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row2.stage5),
        ];

        for (i, stage_id) in stage_ids.iter().enumerate() {
            if self.view.view(*stage_id).finger_up(actions).is_some() {
                if i < self.stages.len() {
                    let stage = &self.stages[i];
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        HomeScreenAction::NavigateToStage { stage_id: stage.id },
                    );
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl HomeScreen {
    /// Update greeting based on time of day
    fn update_greeting(&mut self, cx: &mut Cx) {
        // Get current hour using std::time (UTC then approximate local)
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Approximate local hour (UTC+8 for China timezone)
        let hour = ((secs / 3600) + 8) % 24;
        let greeting = if hour < 12 {
            "早上好"
        } else if hour < 18 {
            "下午好"
        } else {
            "晚上好"
        };

        // Check if logged in
        let prefs = Preferences::load();
        let user_name = if prefs.auth_token.is_some() {
            "用户"
        } else {
            "欢迎来到开朗英语"
        };

        let subtitle = if prefs.auth_token.is_some() {
            "今天想聊点什么？AI 已经准备好陪你练习了"
        } else {
            "开始你的英语学习之旅"
        };

        self.view.label(ids!(content_scroll.content.left_column.welcome_card.welcome_content.greeting))
            .set_text(cx, &format!("{}，{}！", greeting, user_name));
        self.view.label(ids!(content_scroll.content.left_column.welcome_card.welcome_content.subtitle))
            .set_text(cx, subtitle);
    }

    /// Load data from APIs
    fn load_data(&mut self, cx: &mut Cx) {
        let (tx, rx) = mpsc::channel();
        self.fetch_rx = Some(rx);

        let tx1 = tx.clone();
        let tx2 = tx;

        // Fetch stages
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(api) = get_asset_api() {
                    if let Ok(client) = api.read() {
                        let result = client.list_stages().await;
                        let _ = tx1.send(FetchResult::Stages(result));
                    }
                }
            });
        });

        // Fetch stats if logged in
        let prefs = Preferences::load();
        if prefs.auth_token.is_some() {
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Some(api) = get_learn_api() {
                        if let Ok(client) = api.read() {
                            let result = client.list_daily_stats(Some(7)).await;
                            let _ = tx2.send(FetchResult::Stats(result));
                        }
                    }
                });
            });
        }

        self.view.redraw(cx);
    }

    /// Update stage cards with fetched data
    fn update_stage_cards(&mut self, cx: &mut Cx) {
        let stage_ids = [
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row1.stage0),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row1.stage1),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row1.stage2),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row2.stage3),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row2.stage4),
            ids!(content_scroll.content.left_column.stages_card.stages_grid.row2.stage5),
        ];

        for (i, stage_id) in stage_ids.iter().enumerate() {
            let card = self.view.view(*stage_id);

            if i < self.stages.len() {
                let stage = &self.stages[i];
                card.set_visible(cx, true);
                card.label(ids!(stage_icon)).set_text(cx, stage.icon_emoji.as_deref().unwrap_or("📚"));
                card.label(ids!(stage_title)).set_text(cx, &stage.name_zh);
                card.label(ids!(stage_subtitle)).set_text(cx, &stage.name_en);
            } else {
                card.set_visible(cx, false);
            }
        }
    }

    /// Update stats with fetched data
    fn update_stats(&mut self, cx: &mut Cx, stats: &[DailyStat]) {
        // Calculate weekly minutes
        let weekly_minutes: i32 = stats.iter()
            .filter_map(|s| s.minutes_studied)
            .sum();

        self.view.label(ids!(content_scroll.content.left_column.stats_card.content.stats_grid.row1.stat_minutes.stat_value))
            .set_text(cx, &weekly_minutes.to_string());

        // Calculate mastered vocabulary (sum of new words learned)
        let vocab_count: i32 = stats.iter()
            .filter_map(|s| s.new_words_learned)
            .sum();

        self.view.label(ids!(content_scroll.content.left_column.stats_card.content.stats_grid.row1.stat_vocab.stat_value))
            .set_text(cx, &vocab_count.to_string());

        // Calculate pending review
        let review_count: i32 = stats.iter()
            .filter_map(|s| s.review_words_count)
            .sum();

        self.view.label(ids!(content_scroll.content.left_column.stats_card.content.stats_grid.row2.stat_review.stat_value))
            .set_text(cx, &review_count.to_string());
    }
}

impl HomeScreenRef {
    pub fn refresh_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.data_loaded = false;
            inner.load_data(cx);
        }
    }
}
