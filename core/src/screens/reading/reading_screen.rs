//! Reading Practice Screen - Pronunciation training with AI feedback
//!
//! Features:
//! - Audio waveform comparison
//! - Real-time pronunciation scoring
//! - Detailed feedback on pronunciation, fluency, and intonation
//! - Progress tracking

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use colang_widgets::theme::*;

    // ========================================================================
    // Reading Practice Components
    // ========================================================================

    SentenceDisplay = <RoundedView> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 8
        align: {x: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 10.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_100), (SLATE_700), self.dark_mode);
            }
        }

        sentence_label = <Label> {
            text: "原文"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                }
            }
        }

        sentence_en = <Label> {
            text: "Could you please help me with this?"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        sentence_zh = <Label> {
            text: "你能帮我一下吗？"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    WaveformPanel = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 8

        panel_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        waveform_view = <RoundedView> {
            width: Fill, height: 80
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 8.0
                fn get_color(self) -> vec4 {
                    return mix(vec4(0.15, 0.16, 0.19, 1.0), (SLATE_900), self.dark_mode);
                }
            }
        }

        play_btn = <Button> {
            width: 40, height: 40
            abs_pos: vec2(20, 20)
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.circle(20., 20., 20.);
                    sdf.fill((ACCENT_GREEN));
                    return sdf.result;
                }
            }
        }
    }

    ScoreCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 10.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_100), (SLATE_700), self.dark_mode);
            }
        }

        <Label> {
            text: "🧠 AI 评分与建议"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        score_display = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 16

            total_score = <View> {
                width: Fit, height: Fit
                flow: Down
                spacing: 4
                align: {x: 0.5}

                <View> {
                    width: 70, height: 70
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.circle(35., 35., 35.);
                            sdf.fill(vec4(0.067, 0.725, 0.506, 0.2));
                            sdf.circle(35., 35., 35.);
                            sdf.stroke((ACCENT_GREEN), 3.0);
                            return sdf.result;
                        }
                    }
                }

                <Label> {
                    text: "85"
                    draw_text: {
                        text_style: <FONT_BOLD>{ font_size: 24.0 }
                        color: (ACCENT_GREEN)
                    }
                }

                <Label> {
                    text: "总分"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 10.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }
            }

            detailed_scores = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                pronunciation_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 8

                    <Label> {
                        width: 80
                        text: "发音准确度"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 6
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.9
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 3.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        width: 40
                        text: "90%"
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (ACCENT_GREEN)
                        }
                    }
                }

                fluency_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 8

                    <Label> {
                        width: 80
                        text: "流畅度"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 6
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.8
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 3.0);
                                sdf.fill(vec4(0.984, 0.749, 0.141, 1.0));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        width: 40
                        text: "80%"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                let light_color = vec4(0.984, 0.749, 0.141, 1.0);
                                let dark_color = (YELLOW_500);
                                return mix(light_color, dark_color, self.dark_mode);
                            }
                        }
                    }
                }

                intonation_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 8

                    <Label> {
                        width: 80
                        text: "语调"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 6
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.85
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 3.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        width: 40
                        text: "85%"
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (ACCENT_GREEN)
                        }
                    }
                }
            }
        }

        feedback_text = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            <Label> {
                text: "⚠️ 需要注意: \"help\" 的发音稍重，注意轻读"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <Label> {
                text: "✓ 做得好: \"Could you\" 的连读非常自然！"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    color: (ACCENT_GREEN)
                }
            }
        }
    }

    ActionButtons = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 12
        align: {x: 0.5}

        prev_btn = <Button> {
            width: 140, height: 44
            text: "⏮️ 上一句"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    let color = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
        }

        record_btn = <Button> {
            width: 140, height: 44
            text: "🎙️ 重录"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill(vec4(0.937, 0.267, 0.267, 1.0));
                    return sdf.result;
                }
            }
        }

        next_btn = <Button> {
            width: 140, height: 44
            text: "下一句 ⏭️"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill((ACCENT_GREEN));
                    return sdf.result;
                }
            }
        }
    }

    // ========================================================================
    // Main Reading Practice Screen
    // ========================================================================

    pub ReadingScreen = {{ReadingScreen}} {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: 20
                spacing: 20

                // Header
                header = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8
                    visible: false

                    <Label> {
                        text: "🎤 跟读练习"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 24.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <Label> {
                        text: "发音纠正 · 音波对比 · AI 智能评分"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Progress
                progress_bar = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 12

                    <Label> {
                        text: "练习进度"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.5
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        text: "3/6 句"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Sentence display
                sentence_display = <SentenceDisplay> {}

                // Waveforms
                native_waveform = <WaveformPanel> {
                    panel_label = { text: "🔊 标准发音" }
                }

                user_waveform = <WaveformPanel> {
                    panel_label = { text: "🎙️ 你的发音" }
                }

                // Score and feedback
                score_card = <ScoreCard> {}

                // Action buttons
                action_buttons = <ActionButtons> {}
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ReadingScreen {
    #[deref]
    view: View,
}

impl Widget for ReadingScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReadingScreen {
    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }
}
