//! Real-time Assistant Screen - AI assistance during real conversations
//!
//! Features:
//! - Real-time conversation monitoring
//! - AI-powered response suggestions
//! - Grammar checking
//! - Vocabulary assistance
//! - Translation support

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use widgets::theme::*;

    // ========================================================================
    // Assistant Screen Components
    // ========================================================================

    ConnectionStatus = <View> {
        width: Fill, height: Fit
        padding: 16
        flow: Right
        spacing: 12
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }

        status_indicator = <View> {
            width: 16, height: 16
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(8., 8., 8.);
                    sdf.fill((ACCENT_GREEN));
                    return sdf.result;
                }
            }
        }

        <Label> {
            text: "已连接 WhatsApp"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        settings_btn = <Button> {
            width: Fit, height: Fit
            padding: {left: 12, right: 12, top: 8, bottom: 8}
            text: "设置"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                    sdf.fill((ACCENT_INDIGO));
                    return sdf.result;
                }
            }
        }
    }

    ConversationPanel = <RoundedView> {
        width: Fill, height: Fill
        padding: 16
        flow: Down
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        <Label> {
            text: "💬 实时对话"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        conversation_scroll = <ScrollYView> {
            width: Fill, height: Fill

            conversation_content = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12
                padding: 8

                <Label> {
                    text: "对话内容将在这里显示..."
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }
            }
        }
    }

    AssistancePanel = <RoundedView> {
        width: Fill, height: Fill
        padding: 16
        flow: Down
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        <Label> {
            text: "🧠 AI 辅助面板"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        // Quick Translation
        translation_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            <Label> {
                text: "⚡ 快速翻译"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            translation_input = <TextInput> {
                width: Fill, height: 36
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                }
            }
        }

        // Grammar Check
        grammar_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            <Label> {
                text: "✅ 语法检查"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            grammar_result = <Label> {
                text: "等待输入..."
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Vocabulary Help
        vocabulary_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            <Label> {
                text: "📚 词汇助手"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <Label> {
                text: "点击查词或获取同义词建议"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }

    FeatureToggles = <View> {
        width: Fill, height: Fit
        padding: 12
        flow: Right
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }

        auto_suggest_toggle = <Button> {
            text: "✓ 自动建议"
            padding: {left: 12, right: 12, top: 8, bottom: 8}
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                    sdf.fill((ACCENT_GREEN));
                    return sdf.result;
                }
            }
        }

        grammar_toggle = <Button> {
            text: "✓ 语法检查"
            padding: {left: 12, right: 12, top: 8, bottom: 8}
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                    sdf.fill((ACCENT_GREEN));
                    return sdf.result;
                }
            }
        }
    }

    // ========================================================================
    // Main Assistant Screen
    // ========================================================================

    pub AssistantScreen = {{AssistantScreen}} {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        content = <View> {
            width: Fill, height: Fill
            flow: Down

            connection_status = <ConnectionStatus> {}

            main_content = <View> {
                width: Fill, height: Fill
                flow: Right
                padding: 16
                spacing: 16

                conversation_panel = <ConversationPanel> {}
                assistance_panel = <AssistancePanel> {}
            }

            feature_toggles = <FeatureToggles> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct AssistantScreen {
    #[deref]
    view: View,
}

impl Widget for AssistantScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl AssistantScreen {
    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }
}
