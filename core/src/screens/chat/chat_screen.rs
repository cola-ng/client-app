//! MoFA FM Screen - Main screen for AI-powered audio streaming
//!
//! This module is split into sub-modules for better organization:
//! - `audio_controls.rs` - Audio device selection, mic monitoring
//! - `chat_panel.rs` - Chat display, prompt input
//! - `log_panel.rs` - Log display, filtering
//! - `dora_handlers.rs` - Dora event handling, dataflow control

use std::path::PathBuf;

use makepad_widgets::*;
use colang_widgets::StateChangeListener;
use colang_widgets::participant_panel::ParticipantPanelWidgetExt;
use makepad_component::*;

use super::ChatMessageEntry;
use super::mofa_hero::{MofaHeroAction, MofaHeroWidgetExt};
use crate::dora_integration::{DoraCommand, DoraIntegration};
use crate::log_bridge;

mod audio_controls;
mod chat_panel;
mod dora_handlers;
mod log_panel;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Orange accent colors
    ACCENT_ORANGE = #f97316
    ORANGE_100 = #ffedd5
    ORANGE_500 = #f97316
    use colang_widgets::participant_panel::ParticipantPanel;
    use colang_widgets::log_panel::LogPanel;
    use crate::screens::chat::mofa_hero::MofaHero;

    // Local layout constants (colors imported from theme)
    SECTION_SPACING = 12.0
    PANEL_RADIUS = 12.0
    PANEL_PADDING = 16.0

    // Conversation history item
    ConversationItem = <View> {
        width: Fill, height: Fit
        padding: 16
        flow: Right
        spacing: 12
        cursor: Hand

        conv_avatar = <RoundedView> {
            width: 40, height: 40
            show_bg: true
            draw_bg: {
                color: (ORANGE_100)
                border_radius: 20.0
            }
            align: {x: 0.5, y: 0.5}

            <Label> {
                text: "💬"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 18.0 }
                }
            }
        }

        conv_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            conv_title = <Label> {
                text: "新对话"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            conv_preview = <Label> {
                text: "开始对话..."
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                    }
                }
            }

            conv_time = <Label> {
                text: "刚刚"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                    }
                }
            }
        }
    }

    // Language toggle button
    LangToggleBtn = <RoundedView> {
        width: Fit, height: 24
        padding: {left: 8, right: 8}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            border_radius: 4.0
            fn pixel(self) -> vec4 {
                let orange = vec4(0.976, 0.451, 0.086, 1.0);
                let white = vec4(1.0, 1.0, 1.0, 1.0);
                return mix(white, orange, self.selected);
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand
    }

    // Reusable panel header style with dark mode support
    PanelHeader = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }
    }

    // 日常唠嗑 Screen - redesigned to match website
    pub ChatScreen = {{ChatScreen}} {
        width: Fill, height: Fill
        flow: Overlay
        padding: { left: 24, right: 24, top: 24, bottom: 24 }
        align: {y: 0.0}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Gradient: orange-50 (#fff7ed) → amber-50 (#fffbeb) → yellow-50 (#fefce8)
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0);
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);

                let t = self.pos.x;
                let light_color = vec4(0.0);
                if t < 0.5 {
                    light_color = mix(orange_50, amber_50, t * 2.0);
                } else {
                    light_color = mix(amber_50, yellow_50, (t - 0.5) * 2.0);
                }
                return mix(light_color, dark_bg, self.dark_mode);
            }
        }

        main_layout = <RoundedView> {
            width: Fill, height: Fill
            flow: Right
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 12.0
                fn get_color(self) -> vec4 {
                    return mix((WHITE), (SLATE_800), self.dark_mode);
                }
            }

            // Left sidebar - Conversation history
            left_sidebar = <View> {
                width: 280, height: Fill
                flow: Down
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix(vec4(0.976, 0.980, 0.984, 1.0), (SLATE_900), self.dark_mode);
                    }
                }

                // New conversation button
                new_conv_section = <View> {
                    width: Fill, height: Fit
                    padding: 16
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn get_color(self) -> vec4 {
                            return mix((WHITE), (SLATE_800), self.dark_mode);
                        }
                    }

                    new_conv_btn = <Button> {
                        width: Fill, height: 40
                        text: "+ 新对话"
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                            color: (WHITE)
                        }
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                sdf.fill(vec4(0.976, 0.451, 0.086, 1.0));
                                return sdf.result;
                            }
                        }
                    }
                }

                // Conversation list
                conv_list_scroll = <ScrollYView> {
                    width: Fill, height: Fill
                    flow: Down

                    conv_list = <View> {
                        width: Fill, height: Fit
                        flow: Down

                        conv_item_1 = <ConversationItem> {
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(1.0, 1.0, 1.0, 1.0);
                                }
                            }
                            // Active indicator - left border
                            margin: {left: 0}
                            padding: {left: 12}

                            conv_content = {
                                conv_title = { text: "旅行计划讨论" }
                                conv_preview = { text: "That sounds like a great trip!" }
                                conv_time = { text: "30分钟前" }
                            }
                        }

                        conv_item_2 = <ConversationItem> {
                            conv_content = {
                                conv_title = { text: "工作面试准备" }
                                conv_preview = { text: "Let's practice some common questions." }
                                conv_time = { text: "2小时前" }
                            }
                        }

                        conv_item_3 = <ConversationItem> {
                            conv_content = {
                                conv_title = { text: "餐厅点餐练习" }
                                conv_preview = { text: "Would you like to see the menu?" }
                                conv_time = { text: "昨天" }
                            }
                        }
                    }
                }
            }

            // Divider
            sidebar_divider = <View> {
                width: 1, height: Fill
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix((SLATE_200), (SLATE_600), self.dark_mode);
                    }
                }
            }

            // Right panel - Chat window
            left_column = <View> {
                width: Fill, height: Fill
                flow: Down

                // Chat header with language toggles
                chat_header = <PanelHeader> {
                    flow: Right
                    spacing: 12
                    align: {y: 0.5}

                    chat_title = <Label> {
                        text: "日常唠嗑"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> { width: Fill }

                    // Language toggle for AI
                    ai_lang_section = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 6
                        align: {y: 0.5}

                        <Label> {
                            text: "AI:"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (TEXT_MUTED)
                            }
                        }

                        ai_lang_group = <RoundedView> {
                            width: Fit, height: 26
                            flow: Right
                            show_bg: true
                            draw_bg: {
                                border_radius: 4.0
                                border_color: (SLATE_200)
                                border_width: 1.0
                                color: (WHITE)
                            }

                            ai_lang_both = <LangToggleBtn> {
                                draw_bg: { selected: 1.0 }
                                <Label> {
                                    text: "双语"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (WHITE)
                                    }
                                }
                            }
                            ai_lang_en = <LangToggleBtn> {
                                <Label> {
                                    text: "英"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (TEXT_SECONDARY)
                                    }
                                }
                            }
                            ai_lang_zh = <LangToggleBtn> {
                                <Label> {
                                    text: "中"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (TEXT_SECONDARY)
                                    }
                                }
                            }
                        }
                    }

                    // Language toggle for user
                    user_lang_section = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 6
                        align: {y: 0.5}
                        margin: {left: 12}

                        <Label> {
                            text: "我:"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (TEXT_MUTED)
                            }
                        }

                        user_lang_group = <RoundedView> {
                            width: Fit, height: 26
                            flow: Right
                            show_bg: true
                            draw_bg: {
                                border_radius: 4.0
                                border_color: (SLATE_200)
                                border_width: 1.0
                                color: (WHITE)
                            }

                            user_lang_both = <LangToggleBtn> {
                                draw_bg: { selected: 1.0 }
                                <Label> {
                                    text: "双语"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (WHITE)
                                    }
                                }
                            }
                            user_lang_en = <LangToggleBtn> {
                                <Label> {
                                    text: "英"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (TEXT_SECONDARY)
                                    }
                                }
                            }
                            user_lang_zh = <LangToggleBtn> {
                                <Label> {
                                    text: "中"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (TEXT_SECONDARY)
                                    }
                                }
                            }
                        }
                    }

                    // Copy button
                    copy_chat_btn = <Button> {
                        width: 28, height: 24
                        margin: {left: 12}
                        text: ""
                        draw_bg: {
                            instance hover: 0.0
                            instance pressed: 0.0
                            instance copied: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let c = self.rect_size * 0.5;
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                let bg_color = mix((SLATE_200), (SLATE_500), self.hover);
                                let bg_color = mix(bg_color, (SLATE_600), self.pressed);
                                let bg_color = mix(bg_color, #22c55e, self.copied);
                                sdf.fill(bg_color);
                                let icon_color = mix((WHITE), (WHITE), self.copied);
                                sdf.box(c.x - 4.0, c.y - 2.0, 8.0, 9.0, 1.0);
                                sdf.stroke(icon_color, 1.2);
                                sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                sdf.fill(bg_color);
                                sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                sdf.stroke(icon_color, 1.2);
                                return sdf.result;
                            }
                        }
                    }
                }

                // Header divider
                header_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((SLATE_200), (SLATE_600), self.dark_mode);
                        }
                    }
                }

                // Chat container - messages area and input
                chat_container = <View> {
                    width: Fill, height: Fill
                    flow: Down

                    // Chat messages area (scrollable, fills space)
                    chat_section = <View> {
                        width: Fill, height: Fill
                        flow: Down

                        chat_scroll = <ScrollYView> {
                            width: Fill, height: Fill
                            flow: Down
                            padding: (PANEL_PADDING)
                            scroll_bars: <ScrollBars> {
                                show_scroll_x: false
                                show_scroll_y: true
                            }

                            chat_content_wrapper = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 16

                                chat_content = <Markdown> {
                                    width: Fill, height: Fit
                                    font_size: 14.0
                                    font_color: (TEXT_PRIMARY)
                                    paragraph_spacing: 10

                                    draw_normal: {
                                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                    }
                                    draw_bold: {
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                    }
                                }
                            }
                        }
                    }

                    // Input divider
                    input_divider = <View> {
                        width: Fill, height: 1
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                return mix((SLATE_200), (SLATE_600), self.dark_mode);
                            }
                        }
                    }

                    // Prompt input area
                    prompt_container = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        padding: 16
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix((WHITE), (SLATE_800), self.dark_mode);
                            }
                        }

                        // Large mic button centered
                        mic_row = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            align: {x: 0.5}
                            spacing: 8
                            margin: {bottom: 16}

                            voice_input_btn = <Button> {
                                width: 64, height: 64
                                text: "🎤"
                                draw_bg: {
                                    instance recording: 0.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.circle(32., 32., 32.);
                                        let orange = vec4(0.976, 0.451, 0.086, 1.0);
                                        let red = vec4(0.937, 0.267, 0.267, 1.0);
                                        sdf.fill(mix(orange, red, self.recording));
                                        return sdf.result;
                                    }
                                }
                                draw_text: {
                                    color: (WHITE)
                                    text_style: <FONT_MEDIUM>{ font_size: 24.0 }
                                }
                                icon_walk: { width: 0, height: 0 }
                            }

                            recording_hint = <Label> {
                                visible: false
                                text: "正在录音... 点击话筒停止"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    color: #ef4444
                                }
                            }
                        }

                        // Text input row
                        prompt_section = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 8
                            align: {y: 0.5}

                            prompt_input = <TextInput> {
                                width: Fill, height: Fit
                                padding: {left: 12, right: 12, top: 10, bottom: 10}
                                empty_text: "输入文字消息... (Shift+Enter 换行)"
                                draw_bg: {
                                    instance dark_mode: 0.0
                                    border_radius: 8.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                        sdf.fill(mix((WHITE), (SLATE_700), self.dark_mode));
                                        sdf.stroke(mix((SLATE_300), (SLATE_600), self.dark_mode), 1.0);
                                        return sdf.result;
                                    }
                                }
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                                draw_selection: { color: (INDIGO_200) }
                            }

                            button_group = <View> {
                                width: Fit, height: Fit
                                flow: Right
                                spacing: 8

                                send_prompt_btn = <Button> {
                                    width: 36, height: 36
                                    text: "➤"
                                    draw_text: {
                                        color: (WHITE)
                                        text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                                    }
                                    draw_bg: {
                                        fn pixel(self) -> vec4 {
                                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                            sdf.fill(vec4(0.976, 0.451, 0.086, 1.0));
                                            return sdf.result;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Hidden elements to maintain compatibility (mofa_hero, participant panels, right_panel)
        hidden_compat = <View> {
            visible: false

            // System status bar (self-contained widget)
            mofa_hero = <MofaHero> { width: Fill }

            // Participant status cards container
            participant_container = <View> {
                participant_bar = <View> {
                    myself_panel = <ParticipantPanel> {
                        header = { name_label = { text: "Myself" } }
                    }
                    teacher_panel = <ParticipantPanel> {
                        header = { name_label = { text: "Teacher" } }
                    }
                }
            }

            // Reset button for compatibility
            reset_btn = <Button> { text: "清空" }
        }

        // System log overlay (global)
        log_overlay = <View> {
            width: Fill, height: Fill
            visible: false
            flow: Overlay

            log_scrim = <View> {
                width: Fill, height: Fill
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let light_color = vec4(0.0, 0.0, 0.0, 0.35);
                        let dark_color = vec4(0.0, 0.0, 0.0, 0.5);
                        return mix(light_color, dark_color, self.dark_mode);
                    }
                }
            }

            log_modal = <RoundedView> {
                width: 840, height: 620
                abs_pos: vec2(120.0, 80.0)
                draw_bg: { color: (WHITE) border_radius: 14.0 }
                flow: Down
                padding: 12

                // Keep legacy IDs for the log logic
                log_section = <View> {
                    width: Fill, height: Fill
                    flow: Down
                    spacing: 8

                    log_header = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        padding: { left: 4, right: 4, top: 4, bottom: 4 }

                        log_title_row = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            log_title_label = <Label> {
                                text: "System Log"
                                draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 14.0 } color: (TEXT_PRIMARY) }
                            }
                        }

                        close_log_btn = <Button> {
                            width: 28, height: 28
                            text: "✕"
                            draw_text: { color: (TEXT_PRIMARY) text_style: <FONT_BOLD>{ font_size: 12.0 } }
                            draw_bg: { color: (HOVER_BG) border_radius: 6.0 }
                        }
                    }

                    log_filter_row = <View> {
                        width: Fill, height: 32
                        flow: Right
                        align: {y: 0.5}
                        spacing: 8

                        level_filter = <DropDown> {
                            width: 80, height: 26
                            labels: ["ALL", "DEBUG", "INFO", "WARN", "ERROR"]
                            values: [ALL, DEBUG, INFO, WARN, ERROR]
                        }
                        node_filter = <DropDown> {
                            width: 120, height: 26
                            labels: ["All Nodes", "ASR", "TTS", "LLM", "Bridge", "Monitor", "App"]
                            values: [ALL, ASR, TTS, LLM, BRIDGE, MONITOR, APP]
                        }
                        log_search = <TextInput> { width: Fill, height: 26 empty_text: "Search..." }
                        copy_log_btn = <Button> {
                            width: 32, height: 26
                            text: "📋"
                            draw_bg: { color: (ORANGE_100) border_radius: 6.0 }
                            draw_text: { color: (TEXT_PRIMARY) text_style: <FONT_MEDIUM>{ font_size: 12.0 } }
                        }
                    }

                    log_content_column = <RoundedView> {
                        width: Fill, height: Fill
                        draw_bg: { color: (SLATE_50) border_radius: 10.0 }
                        flow: Down

                        log_scroll = <ScrollYView> {
                            width: Fill, height: Fill
                            flow: Down
                            scroll_bars: <ScrollBars> { show_scroll_x: false show_scroll_y: true }

                            log_content_wrapper = <View> {
                                width: Fill, height: Fit
                                padding: { left: 12, right: 12, top: 10, bottom: 10 }
                                flow: Down

                                log_content = <Markdown> {
                                    width: Fill, height: Fit
                                    font_size: 11.0
                                    font_color: (GRAY_600)
                                    paragraph_spacing: 6

                                    draw_normal: { text_style: <FONT_REGULAR>{ font_size: 11.0 } }
                                    draw_bold: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
                                    draw_fixed: { text_style: <FONT_REGULAR>{ font_size: 10.0 } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatScreen {
    #[deref]
    view: View,
    #[rust]
    log_overlay_visible: bool,
    #[rust]
    audio_manager: Option<crate::audio::AudioManager>,
    #[rust]
    audio_timer: Timer,
    #[rust]
    audio_initialized: bool,
    #[rust]
    log_level_filter: usize, // 0=ALL, 1=DEBUG, 2=INFO, 3=WARN, 4=ERROR
    #[rust]
    log_node_filter: usize, // 0=ALL, 1=ASR, 2=TTS, 3=LLM, 4=Bridge, 5=Monitor, 6=App
    #[rust]
    log_entries: Vec<String>, // Raw log entries for filtering

    // Dora integration
    #[rust]
    dora_integration: Option<DoraIntegration>,
    #[rust]
    dataflow_path: Option<PathBuf>,
    #[rust]
    dora_timer: Timer,
    #[rust]
    copy_chat_feedback_timer: Timer,
    #[rust]
    copy_log_feedback_timer: Timer,
    #[rust]
    chat_messages: Vec<ChatMessageEntry>,
    #[rust]
    last_chat_count: usize,
    // Pending streaming messages (updated in-place, removed when streaming ends)
    #[rust]
    pending_streaming_messages: Vec<ChatMessageEntry>,

    // Audio playback
    #[rust]
    audio_player: Option<std::sync::Arc<crate::audio_player::AudioPlayer>>,
    // Participant audio levels for decay animation (matches conference-dashboard)
    #[rust]
    participant_levels: [f64; 2], // 0=myself, 1=teacher
}

impl Widget for ChatScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Initialize audio and log bridge on first event
        if !self.audio_initialized {
            // Initialize log bridge to capture Rust logs
            log_bridge::init();
            self.init_audio(cx);
            self.audio_initialized = true;
        }

        // Handle audio timer for mic level updates, log polling, and buffer status
        if self.audio_timer.is_event(event).is_some() {
            self.update_mic_level(cx);
            // Poll Rust logs (50ms interval is fine for log updates)
            self.poll_rust_logs(cx);
            // Send actual buffer fill percentage to dora for backpressure control
            // This replaces the bridge's estimation with the real value from AudioPlayer
            if let Some(ref player) = self.audio_player {
                let fill_percentage = player.buffer_fill_percentage();
                if let Some(ref dora) = self.dora_integration {
                    dora.send_command(DoraCommand::UpdateBufferStatus { fill_percentage });
                }
            }

            // Poll for captured audio chunks and send to dora
            if let Some(ref audio_manager) = self.audio_manager {
                let chunks = audio_manager.poll_audio_chunks();
                if !chunks.is_empty() {
                    if let Some(ref dora) = self.dora_integration {
                        for chunk in chunks {
                            ::log::debug!(
                                "Sending mic audio chunk: {} samples at {}Hz",
                                chunk.samples.len(),
                                chunk.sample_rate
                            );
                            dora.send_audio(chunk.samples, chunk.sample_rate);
                        }
                    }
                }
            }
        }

        // Handle dora timer for polling dora events
        if self.dora_timer.is_event(event).is_some() {
            self.poll_dora_events(cx);
        }

        // Handle copy chat feedback timer - reset animation
        if self.copy_chat_feedback_timer.is_event(event).is_some() {
            self.view
                .button(ids!(
                    main_layout
                        .left_column
                        .chat_header
                        .copy_chat_btn
                ))
                .apply_over(cx, live! { draw_bg: { copied: 0.0 } });
            self.view.redraw(cx);
        }

        // Handle copy log feedback timer - reset animation
        if self.copy_log_feedback_timer.is_event(event).is_some() {
            self.view
                .button(ids!(
                    log_overlay
                        .log_modal
                        .log_section
                        .log_filter_row
                        .copy_log_btn
                ))
                .apply_over(cx, live! { draw_bg: { copied: 0.0 } });
            self.view.redraw(cx);
        }

        // Handle actions
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => &[],
        };

        // Handle MofaHero start/stop actions
        for action in actions {
            match action.as_widget_action().cast() {
                MofaHeroAction::StartClicked => {
                    ::log::info!("Screen received StartClicked action");
                    self.handle_mofa_start(cx);
                }
                MofaHeroAction::StopClicked => {
                    ::log::info!("Screen received StopClicked action");
                    self.handle_mofa_stop(cx);
                }
                MofaHeroAction::None => {}
            }
        }

        // Handle log filters
        if let Some(selected) = self
            .view
            .drop_down(ids!(
                log_overlay
                    .log_modal
                    .log_section
                    .log_filter_row
                    .level_filter
            ))
            .selected(actions)
        {
            self.log_level_filter = selected;
            self.update_log_display(cx);
        }

        if let Some(selected) = self
            .view
            .drop_down(ids!(
                log_overlay.log_modal.log_section.log_filter_row.node_filter
            ))
            .selected(actions)
        {
            self.log_node_filter = selected;
            self.update_log_display(cx);
        }

        // Handle copy log button
        if self
            .view
            .button(ids!(
                log_overlay
                    .log_modal
                    .log_section
                    .log_filter_row
                    .copy_log_btn
            ))
            .clicked(actions)
        {
            self.copy_logs_to_clipboard(cx);
            self.view
                .button(ids!(
                    log_overlay
                        .log_modal
                        .log_section
                        .log_filter_row
                        .copy_log_btn
                ))
                .apply_over(cx, live! { draw_bg: { copied: 1.0 } });
            self.view.redraw(cx);
            self.copy_log_feedback_timer = cx.start_timeout(1.0);
        }

        if self
            .view
            .button(ids!(log_overlay.log_modal.log_section.close_log_btn))
            .clicked(actions)
        {
            self.hide_log_overlay(cx);
        }

        // Handle copy chat button
        if self
            .view
            .button(ids!(
                main_layout
                    .left_column
                    .chat_header
                    .copy_chat_btn
            ))
            .clicked(actions)
        {
            self.copy_chat_to_clipboard(cx);
            // Trigger copied feedback animation
            self.view
                .button(ids!(
                    main_layout
                        .left_column
                        .chat_header
                        .copy_chat_btn
                ))
                .apply_over(cx, live! { draw_bg: { copied: 1.0 } });
            self.view.redraw(cx);
            // Start timer to reset animation after 1 second
            self.copy_chat_feedback_timer = cx.start_timeout(1.0);
        }

        // Handle log search text change
        if self
            .view
            .text_input(ids!(
                log_overlay.log_modal.log_section.log_filter_row.log_search
            ))
            .changed(actions)
            .is_some()
        {
            self.update_log_display(cx);
        }

        // Handle Send button click
        if self
            .view
            .button(ids!(
                main_layout
                    .left_column
                    .chat_container
                    .prompt_container
                    .prompt_section
                    .button_group
                    .send_prompt_btn
            ))
            .clicked(actions)
        {
            self.send_prompt(cx);
        }

        // Handle voice input button click
        if self
            .view
            .button(ids!(
                main_layout
                    .left_column
                    .chat_container
                    .prompt_container
                    .mic_row
                    .voice_input_btn
            ))
            .clicked(actions)
        {
            // Toggle recording state (placeholder)
            ::log::info!("Voice input button clicked");
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChatScreenRef {
    /// Stop audio and dora timers - call this before hiding/removing the widget
    /// to prevent timer callbacks on inactive state
    /// Note: AEC blink animation is shader-driven and doesn't need stopping
    pub fn stop_timers(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow_mut() {
            cx.stop_timer(inner.audio_timer);
            cx.stop_timer(inner.dora_timer);
            ::log::debug!("ChatScreen timers stopped");
        }
    }

    /// Restart audio and dora timers - call this when the widget becomes visible again
    /// Note: AEC blink animation is shader-driven and auto-resumes
    pub fn start_timers(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.audio_timer = cx.start_interval(0.05); // 50ms for mic level
            inner.dora_timer = cx.start_interval(0.1); // 100ms for dora events
            ::log::debug!("ChatScreen timers started");
        }
    }
}

impl StateChangeListener for ChatScreenRef {
    fn on_dark_mode_change(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply dark mode to screen background
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Apply dark mode to main layout
            inner
                .view
                .view(ids!(main_layout))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );

            // Apply dark mode to chat header
            inner
                .view
                .view(ids!(main_layout.left_column.chat_header))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .label(ids!(main_layout.left_column.chat_header.chat_title))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );

            // Apply dark mode to left sidebar
            inner
                .view
                .view(ids!(main_layout.left_sidebar))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );

            // Apply dark mode to prompt container
            inner
                .view
                .view(ids!(main_layout.left_column.chat_container.prompt_container))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );

            // Apply dark mode to MofaHero (hidden but still needed)
            inner
                .view
                .mofa_hero(ids!(hidden_compat.mofa_hero))
                .update_dark_mode(cx, dark_mode);

            // Apply dark mode to participant panels (hidden but still needed)
            inner
                .view
                .participant_panel(ids!(
                    hidden_compat
                        .participant_container
                        .participant_bar
                        .myself_panel
                ))
                .update_dark_mode(cx, dark_mode);
            inner
                .view
                .participant_panel(ids!(
                    hidden_compat
                        .participant_container
                        .participant_bar
                        .teacher_panel
                ))
                .update_dark_mode(cx, dark_mode);

            inner.view.redraw(cx);
        }
    }
}
