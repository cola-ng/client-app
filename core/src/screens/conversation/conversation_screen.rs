//! MoFA FM Screen - Main screen for AI-powered audio streaming
//!
//! This module is split into sub-modules for better organization:
//! - `audio_controls.rs` - Audio device selection, mic monitoring
//! - `chat_panel.rs` - Chat display, prompt input
//! - `log_panel.rs` - Log display, filtering
//! - `dora_handlers.rs` - Dora event handling, dataflow control

use std::path::PathBuf;

use makepad_widgets::*;
use makepad_component::*;
use widgets::StateChangeListener;
use widgets::participant_panel::ParticipantPanelWidgetExt;

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
    use makepad_component::*;

    use widgets::theme::*;
    use widgets::participant_panel::ParticipantPanel;
    use widgets::log_panel::LogPanel;
    use crate::screens::conversation::mofa_hero::MofaHero;

    // Local layout constants (colors imported from theme)
    SECTION_SPACING = 12.0
    PANEL_RADIUS = 4.0
    PANEL_PADDING = 12.0

    // Reusable panel header style with dark mode support
    PanelHeader = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (SLATE_800), self.dark_mode);
            }
        }
    }

    // Reusable vertical divider
    VerticalDivider = <View> {
        width: 1, height: Fill
        margin: {top: 4, bottom: 4}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
            }
        }
    }

    // Colang Screen - refreshed layout inspired by the sketch
    pub ConversationScreen = {{ConversationScreen}} {
        width: Fill, height: Fill
        flow: Overlay
        padding: { left: 16, right: 16, top: 16, bottom: 16 }
        align: {y: 0.0}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (DARK_BG_DARK), self.dark_mode);
            }
        }

        main_layout = <View> {
            width: Fill, height: Fill
            flow: Right
            spacing: 16

            // Left column - main content area (adaptive width)
            left_column = <View> {
                width: Fill, height: Fill
                flow: Down
                spacing: (SECTION_SPACING)
                align: {y: 0.0}

                // System status bar (self-contained widget)
                mofa_hero = <MofaHero> { width: Fill }

                // Participant status cards container
                participant_container = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    participant_bar = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: (SECTION_SPACING)

                        myself_panel = <ParticipantPanel> {
                            width: Fill, height: Fit
                            header = { name_label = { text: "Myself" } }
                        }
                        teacher_panel = <ParticipantPanel> {
                            width: Fill, height: Fit
                            header = { name_label = { text: "Teacher" } }
                        }
                    }
                }

                // Chat window container (fills remaining space)
                chat_container = <View> {
                    width: Fill, height: Fill
                    flow: Down
                    spacing: 12

                    chat_info = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 10
                        align: {y: 0.5}

                        chat_avatar = <RoundedView> {
                            width: 40, height: 40
                            draw_bg: {
                                color: (INDIGO_100)
                                border_radius: 20.0
                            }
                            <Label> {
                                text: "🤖"
                                draw_text: {
                                    text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                                }
                            }
                        }

                        chat_meta = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 2
                            chat_title = <Label> {
                                text: "AI 教练 · Luna"
                                draw_text: {
                                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                    color: (TEXT_PRIMARY)
                                }
                            }
                            chat_subtitle = <Label> {
                                text: "酒店入住 · 正常语速 · 美式口音"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                    color: (TEXT_SECONDARY)
                                }
                            }
                        }

                        chat_actions = <View> {
                            width: Fit, height: Fit
                            flow: Right
                            spacing: 8
                            align: {y: 0.5}

                            audio_chip = <RoundedView> {
                                width: Fit, height: Fit
                                padding: { left: 10, right: 10, top: 4, bottom: 4 }
                                draw_bg: {
                                    color: (INDIGO_100)
                                    border_radius: 12.0
                                }
                                <Label> {
                                    text: "🔊"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (ACCENT_INDIGO)
                                    }
                                }
                            }

                            settings_chip = <RoundedView> {
                                width: Fit, height: Fit
                                padding: { left: 10, right: 10, top: 4, bottom: 4 }
                                draw_bg: {
                                    color: (SLATE_100)
                                    border_radius: 12.0
                                }
                                <Label> {
                                    text: "⚙️"
                                    draw_text: {
                                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                        color: (TEXT_PRIMARY)
                                    }
                                }
                            }

                            end_chip = <Button> {
                                width: Fit, height: Fit
                                padding: { left: 12, right: 12, top: 8, bottom: 8 }
                                text: "结束对话"
                                draw_text: {
                                    color: (TEXT_PRIMARY)
                                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                }
                                draw_bg: {
                                    color: (HOVER_BG)
                                    border_radius: 12.0
                                }
                            }

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
                        }
                    }

                    chat_section = <RoundedView> {
                        width: Fill, height: Fill
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: (PANEL_RADIUS)
                            border_size: 1.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                let bg = mix((WHITE), (PANEL_BG_DARK), self.dark_mode);
                                let border = mix((SLATE_200), (SLATE_600), self.dark_mode);
                                sdf.fill(bg);
                                sdf.stroke(border, self.border_size);
                                return sdf.result;
                            }
                        }
                        flow: Down

                        // Chat header with copy button
                        chat_header = <PanelHeader> {
                            chat_title = <Label> {
                                text: "聊天记录"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                            <Filler> {}
                            // Copy to clipboard button
                            copy_chat_btn = <Button> {
                                width: 28, height: 24
                                text: ""
                                draw_bg: {
                                    instance hover: 0.0
                                    instance pressed: 0.0
                                    instance copied: 0.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        let c = self.rect_size * 0.5;

                                        // Background - flash green when copied
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                        let bg_color = mix((SLATE_200), (SLATE_500), self.hover);
                                        let bg_color = mix(bg_color, (SLATE_600), self.pressed);
                                        let bg_color = mix(bg_color, #22c55e, self.copied);
                                        sdf.fill(bg_color);

                                        // Icon color - white when copied for contrast
                                        let icon_color = mix((WHITE), (WHITE), self.copied);

                                        // Clipboard icon - back rectangle
                                        sdf.box(c.x - 4.0, c.y - 2.0, 8.0, 9.0, 1.0);
                                        sdf.stroke(icon_color, 1.2);

                                        // Clipboard icon - front rectangle (overlapping)
                                        sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                        sdf.fill(bg_color);
                                        sdf.box(c.x - 2.0, c.y - 5.0, 8.0, 9.0, 1.0);
                                        sdf.stroke(icon_color, 1.2);

                                        return sdf.result;
                                    }
                                }
                            }
                        }

                        // Chat messages area (scrollable, fills space)
                        chat_scroll = <ScrollYView> {
                            width: Fill, height: Fill
                            flow: Down
                            scroll_bars: <ScrollBars> {
                                show_scroll_x: false
                                show_scroll_y: true
                            }

                            chat_content_wrapper = <View> {
                                width: Fill, height: Fit
                                padding: (PANEL_PADDING)
                                flow: Down
                                spacing: 12

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
                }

                // Prompt input area container
                prompt_container = <View> {
                    width: Fill, height: Fit
                    flow: Down

                    prompt_section = <RoundedView> {
                        width: Fill, height: Fit
                        padding: { left: 16, right: 16, top: 12, bottom: 12 }
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: 18.0
                            border_size: 1.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                let bg = mix((WHITE), (PANEL_BG_DARK), self.dark_mode);
                                let border = mix((SLATE_200), (SLATE_600), self.dark_mode);
                                sdf.fill(bg);
                                sdf.stroke(border, self.border_size);
                                return sdf.result;
                            }
                        }
                        flow: Right
                        spacing: 12
                        align: {y: 0.5}

                        prompt_input = <TextInput> {
                            width: Fill, height: Fit
                            padding: {left: 14, right: 14, top: 12, bottom: 12}
                            empty_text: "说点什么... (按住空格键语音输入)"
                            draw_bg: {
                                instance dark_mode: 0.0
                                border_radius: 12.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                    let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                                    sdf.fill(bg);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
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

                            voice_input_btn = <Button> {
                                width: 40, height: 40
                                text: "🎤"
                                draw_bg: { color: (ACCENT_GREEN) border_radius: 20.0 }
                                draw_text: { color: (WHITE) text_style: <FONT_MEDIUM>{ font_size: 16.0 } }
                                icon_walk: { width: 0, height: 0 }
                            }

                            send_prompt_btn = <Button> {
                                width: 84, height: 40
                                padding: {left: 12, right: 12}
                                text: "发送"
                                draw_text: {
                                    color: (WHITE)
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                }
                                draw_bg: {
                                    instance color: (ACCENT_INDIGO)
                                    instance color_hover: (BLUE_700)
                                    border_radius: 14.0
                                    fn get_color(self) -> vec4 { return mix(self.color, self.color_hover, self.hover); }
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                        sdf.fill(self.get_color());
                                        return sdf.result;
                                    }
                                }
                            }

                            reset_btn = <Button> {
                                width: Fit, height: 40
                                padding: {left: 10, right: 10}
                                text: "清空"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_SECONDARY), (SLATE_200), self.dark_mode);
                                    }
                                }
                                draw_bg: {
                                    instance dark_mode: 0.0
                                    border_radius: 12.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                        let base = mix((HOVER_BG), (SLATE_700), self.dark_mode);
                                        let hover_color = mix((SLATE_200), (SLATE_500), self.dark_mode);
                                        sdf.fill(mix(base, hover_color, self.hover));
                                        return sdf.result;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Right column - contextual tools panel
            right_panel = <View> {
                width: 360, height: Fill
                flow: Down
                spacing: 12

                scene_card = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 16
                    draw_bg: { color: (WHITE) border_radius: 14.0 }
                    flow: Down
                    spacing: 8

                    scene_title = <Label> {
                        text: "🎭 当前场景"
                        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 13.0 } color: (TEXT_PRIMARY) }
                    }
                    scene_desc = <Label> {
                        text: "酒店前台 · 入住登记"
                        draw_text: { text_style: <FONT_REGULAR>{ font_size: 12.0 } color: (TEXT_PRIMARY) }
                    }
                    scene_hint = <Label> {
                        text: "目标：完成预订并询问房间设施"
                        draw_text: { text_style: <FONT_REGULAR>{ font_size: 11.0 } color: (TEXT_SECONDARY) }
                    }
                    progress_bg = <View> {
                        width: Fill, height: 6
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.4
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                // Background
                                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                                sdf.fill(#cbd5e1); // SLATE_200
                                // Progress fill
                                let progress_width = self.rect_size.x * self.progress;
                                sdf.rect(0., 0., progress_width, self.rect_size.y);
                                sdf.fill(#6366f1); // ACCENT_INDIGO
                                return sdf.result;
                            }
                        }
                    }
                }

                word_card = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 16
                    draw_bg: { color: (WHITE) border_radius: 14.0 }
                    flow: Down
                    spacing: 8

                    word_title = <Label> {
                        text: "📝 即时词卡"
                        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 13.0 } color: (ACCENT_INDIGO) }
                    }
                    word_heading = <Label> {
                        text: "book (v.)"
                        draw_text: { text_style: <FONT_BOLD>{ font_size: 20.0 } color: (TEXT_PRIMARY) }
                    }
                    word_pronounce = <Label> {
                        text: "/bʊk/ · 预订、预约"
                        draw_text: { text_style: <FONT_REGULAR>{ font_size: 12.0 } color: (TEXT_SECONDARY) }
                    }
                    word_example = <Label> {
                        text: "例句: I'd like to book a table for two."
                        draw_text: { text_style: <FONT_REGULAR>{ font_size: 11.0 } color: (TEXT_PRIMARY) }
                    }
                    word_actions = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8

                        favorite_btn = <Button> {
                            width: Fit, height: 28
                            padding: {left: 10, right: 10, top: 4, bottom: 4}
                            text: "+ 收藏"
                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 11.0 } color: (TEXT_PRIMARY) }
                            draw_bg: { color: (INDIGO_100) border_radius: 12.0 }
                        }
                        shadow_btn = <Button> {
                            width: Fit, height: 28
                            padding: {left: 10, right: 10, top: 4, bottom: 4}
                            text: "跟读"
                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 11.0 } color: (TEXT_PRIMARY) }
                            draw_bg: { color: (INDIGO_100) border_radius: 12.0 }
                        }
                        more_examples_btn = <Button> {
                            width: Fit, height: 28
                            padding: {left: 10, right: 10, top: 4, bottom: 4}
                            text: "更多例句"
                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 11.0 } color: (TEXT_PRIMARY) }
                            draw_bg: { color: (INDIGO_100) border_radius: 12.0 }
                        }
                    }
                }

                error_card = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 16
                    draw_bg: { color: (WHITE) border_radius: 14.0 }
                    flow: Down
                    spacing: 8

                    error_title = <Label> {
                        text: "⚠️ 易错点提醒"
                        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 13.0 } color: (TEXT_PRIMARY) }
                    }
                    error_item = <RoundedView> {
                        width: Fill, height: 30
                        draw_bg: { color: (INDIGO_100) border_radius: 8.0 }
                        padding: { left: 10, right: 10 }
                        align: {x: 0.0, y: 0.5}
                        <Label> { text: "want to + 动词原形" draw_text: { text_style: <FONT_MEDIUM>{ font_size: 11.0 } color: (TEXT_PRIMARY) } }
                        <View> { width: Fill, height: 1 }
                    }
                }

                voice_settings = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 16
                    draw_bg: { color: (WHITE) border_radius: 14.0 }
                    flow: Down
                    spacing: 8

                    voice_title = <Label> { text: "🔊 语音设置" draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 13.0 } color: (TEXT_PRIMARY) } }
                    accent_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8
                        <Label> { text: "口音：" draw_text: { text_style: <FONT_REGULAR>{ font_size: 11.0 } color: (TEXT_PRIMARY) } }
                        accent_us = <Button> { width: 50, height: 24 text: "美式" draw_text: { color: (WHITE) text_style: <FONT_MEDIUM>{ font_size: 11.0 } } draw_bg: { color: (ACCENT_INDIGO) border_radius: 10.0 } }
                        accent_uk = <Button> { width: 50, height: 24 text: "英式" draw_text: { color: (TEXT_PRIMARY) text_style: <FONT_MEDIUM>{ font_size: 11.0 } } draw_bg: { color: (INDIGO_100) border_radius: 10.0 } }
                        accent_au = <Button> { width: 50, height: 24 text: "澳式" draw_text: { color: (TEXT_PRIMARY) text_style: <FONT_MEDIUM>{ font_size: 11.0 } } draw_bg: { color: (INDIGO_100) border_radius: 10.0 } }
                    }
                    speed_row = <Label> { text: "语速：0.8x ━━━●━━━ 1.5x" draw_text: { text_style: <FONT_REGULAR>{ font_size: 11.0 } color: (TEXT_SECONDARY) } }
                }
            }
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
                            draw_bg: { color: (INDIGO_100) border_radius: 6.0 }
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
pub struct ConversationScreen {
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

impl Widget for ConversationScreen {
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
                        .chat_container
                        .chat_section
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
                    .chat_container
                    .chat_section
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
                        .chat_container
                        .chat_section
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
                    .prompt_container
                    .prompt_section
                    .button_group
                    .send_prompt_btn
            ))
            .clicked(actions)
        {
            self.send_prompt(cx);
        }

        // Handle Reset button click
        if self
            .view
            .button(ids!(
                main_layout
                    .left_column
                    .prompt_container
                    .prompt_section
                    .button_group
                    .reset_btn
            ))
            .clicked(actions)
        {
            self.reset_conversation(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ConversationScreenRef {
    /// Stop audio and dora timers - call this before hiding/removing the widget
    /// to prevent timer callbacks on inactive state
    /// Note: AEC blink animation is shader-driven and doesn't need stopping
    pub fn stop_timers(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow_mut() {
            cx.stop_timer(inner.audio_timer);
            cx.stop_timer(inner.dora_timer);
            ::log::debug!("ConversationScreen timers stopped");
        }
    }

    /// Restart audio and dora timers - call this when the widget becomes visible again
    /// Note: AEC blink animation is shader-driven and auto-resumes
    pub fn start_timers(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.audio_timer = cx.start_interval(0.05); // 50ms for mic level
            inner.dora_timer = cx.start_interval(0.1); // 100ms for dora events
            ::log::debug!("ConversationScreen timers started");
        }
    }
}

impl StateChangeListener for ConversationScreenRef {
    fn on_dark_mode_change(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Apply dark mode to screen background
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Apply dark mode to chat section
            inner
                .view
                .view(ids!(main_layout.left_column.chat_container.chat_section))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );

            // Apply dark mode to chat header and title
            inner
                .view
                .view(ids!(
                    main_layout
                        .left_column
                        .chat_container
                        .chat_section
                        .chat_header
                ))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .label(ids!(
                    main_layout
                        .left_column
                        .chat_container
                        .chat_section
                        .chat_header
                        .chat_title
                ))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );

            // Apply dark mode to MofaHero
            inner
                .view
                .mofa_hero(ids!(main_layout.left_column.mofa_hero))
                .update_dark_mode(cx, dark_mode);

            // Apply dark mode to participant panels
            inner
                .view
                .participant_panel(ids!(
                    main_layout
                        .left_column
                        .participant_container
                        .participant_bar
                        .myself_panel
                ))
                .update_dark_mode(cx, dark_mode);
            inner
                .view
                .participant_panel(ids!(
                    main_layout
                        .left_column
                        .participant_container
                        .participant_bar
                        .teacher_panel
                ))
                .update_dark_mode(cx, dark_mode);

            // Apply dark mode to prompt section
            inner
                .view
                .view(ids!(
                    main_layout.left_column.prompt_container.prompt_section
                ))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .button(ids!(
                    main_layout
                        .left_column
                        .prompt_container
                        .prompt_section
                        .button_group
                        .reset_btn
                ))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );

            inner.view.redraw(cx);
        }
    }
}
