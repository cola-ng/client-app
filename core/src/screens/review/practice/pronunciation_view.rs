//! # Pronunciation View
//!
//! Practice view for pronunciation exercises with recording and AI scoring.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;
    use crate::screens::review::components::SecondaryButton;

    // Waveform display component
    WaveformDisplay = <View> {
        width: Fill, height: 80
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }
    }

    // Recording button
    RecordButton = <View> {
        width: 80, height: 80
        align: {x: 0.5, y: 0.5}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance recording: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Outer circle (pulsing when recording)
                let pulse = mix(36., 40., self.recording * sin(self.recording * 10.));
                sdf.circle(c.x, c.y, pulse);
                let outer_color = mix(
                    mix((SLATE_200), (SLATE_600), self.dark_mode),
                    (ACCENT_RED),
                    self.recording
                );
                sdf.fill(outer_color);

                // Inner circle
                sdf.circle(c.x, c.y, 30.);
                let inner_idle = mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                let inner_recording = (ACCENT_RED);
                let inner_hover = mix(inner_idle, (ACCENT_PRIMARY_HOVER), self.hover);
                let inner_color = mix(inner_hover, inner_recording, self.recording);
                sdf.fill(inner_color);

                // Mic icon (when not recording) or stop icon (when recording)
                if self.recording < 0.5 {
                    // Microphone icon
                    sdf.box(c.x - 5., c.y - 12., 10., 16., 5.);
                    sdf.fill((WHITE));
                    sdf.move_to(c.x - 8., c.y + 6.);
                    sdf.line_to(c.x - 8., c.y + 2.);
                    sdf.stroke((WHITE), 2.);
                    sdf.move_to(c.x + 8., c.y + 6.);
                    sdf.line_to(c.x + 8., c.y + 2.);
                    sdf.stroke((WHITE), 2.);
                    sdf.move_to(c.x, c.y + 8.);
                    sdf.line_to(c.x, c.y + 14.);
                    sdf.stroke((WHITE), 2.);
                } else {
                    // Stop icon (square)
                    sdf.box(c.x - 8., c.y - 8., 16., 16., 2.);
                    sdf.fill((WHITE));
                }

                return sdf.result;
            }
        }
    }

    // Score display
    ScoreCircle = <View> {
        width: 120, height: 120
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance score: 0.0
            instance visible: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                if self.visible < 0.5 {
                    return vec4(0.);
                }

                // Background circle
                sdf.circle(c.x, c.y, 56.);
                let track = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(track, 8.0);

                // Score arc
                let angle = self.score * 6.28318530718;
                let start = -1.5707963267;
                sdf.move_to(c.x, c.y - 56.);
                for i in 0..64 {
                    let t = (i as f32) / 64.;
                    if t <= self.score {
                        let a = start + t * 6.28318530718;
                        sdf.line_to(c.x + cos(a) * 56., c.y + sin(a) * 56.);
                    }
                }

                // Color based on score
                let low_color = (ACCENT_RED);
                let mid_color = (ACCENT_YELLOW);
                let high_color = (ACCENT_GREEN);
                let color = mix(low_color, mix(mid_color, high_color, step(0.6, self.score)), step(0.4, self.score));
                sdf.stroke(color, 8.0);

                return sdf.result;
            }
        }

        score_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 28.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    pub PronunciationView = {{PronunciationView}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5}
        spacing: 24

        // Word display
        word_card = <CardBase> {
            width: Fill, height: Fit
            padding: 24
            flow: Down
            align: {x: 0.5}
            spacing: 8

            word_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            phonetic_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            // Play native audio button
            play_native_btn = <Button> {
                width: Fit, height: Fit
                margin: {top: 12}
                padding: {left: 16, right: 16, top: 10, bottom: 10}
                text: "🔊 听原声"
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                        sdf.fill(bg);
                        let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                        sdf.stroke(border, 1.0);
                        return sdf.result;
                    }
                }
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((SLATE_700), (SLATE_200), self.dark_mode);
                    }
                }
            }
        }

        // Recording section
        recording_section = <CardBase> {
            width: Fill, height: Fit
            padding: 24
            flow: Down
            align: {x: 0.5}
            spacing: 16

            instruction = <Label> {
                text: "点击麦克风按钮开始录音"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            waveform = <WaveformDisplay> {}

            record_btn = <RecordButton> {}

            // Score display (shown after evaluation)
            score_display = <View> {
                width: Fill, height: Fit
                visible: false
                flow: Down
                align: {x: 0.5}
                spacing: 12

                score_circle = <ScoreCircle> {}

                score_feedback = <Label> {
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }
        }

        // Action buttons
        action_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {x: 1.0}
            spacing: 12

            retry_btn = <SecondaryButton> {
                text: "重新录音"
                visible: false
            }

            next_btn = <PrimaryButton> {
                text: "下一个"
                visible: false
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum PronunciationViewAction {
    None,
    PlayNative,
    StartRecording,
    StopRecording,
    Retry,
    Next { score: u8 },
}

#[derive(Live, LiveHook, Widget)]
pub struct PronunciationView {
    #[deref]
    view: View,
    #[rust]
    recording: bool,
    #[rust]
    has_score: bool,
    #[rust]
    score: u8,
}

impl Widget for PronunciationView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle play native audio
        if self.view.button(ids!(word_card.play_native_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, PronunciationViewAction::PlayNative);
        }

        // Handle record button
        self.handle_record_button(cx, event, scope);

        // Handle retry button
        if self.view.button(ids!(action_row.retry_btn)).clicked(actions) {
            self.reset_recording(cx);
            cx.widget_action(self.widget_uid(), &scope.path, PronunciationViewAction::Retry);
        }

        // Handle next button
        if self.view.button(ids!(action_row.next_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                PronunciationViewAction::Next { score: self.score },
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl PronunciationView {
    fn handle_record_button(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let record_btn = self.view.view(ids!(recording_section.record_btn));

        match event.hits(cx, record_btn.area()) {
            Hit::FingerHoverIn(_) => {
                record_btn.apply_over(cx, live! { draw_bg: { hover: 1.0 } });
                self.view.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                record_btn.apply_over(cx, live! { draw_bg: { hover: 0.0 } });
                self.view.redraw(cx);
            }
            Hit::FingerUp(_) => {
                if self.recording {
                    self.stop_recording(cx);
                    cx.widget_action(self.widget_uid(), &scope.path, PronunciationViewAction::StopRecording);
                    // Simulate score for now
                    self.show_score(cx, 85);
                } else {
                    self.start_recording(cx);
                    cx.widget_action(self.widget_uid(), &scope.path, PronunciationViewAction::StartRecording);
                }
            }
            _ => {}
        }
    }

    fn start_recording(&mut self, cx: &mut Cx) {
        self.recording = true;
        self.view.view(ids!(recording_section.record_btn)).apply_over(
            cx,
            live! { draw_bg: { recording: 1.0 } },
        );
        self.view.label(ids!(recording_section.instruction))
            .set_text(cx, "正在录音，点击停止...");
        self.view.redraw(cx);
    }

    fn stop_recording(&mut self, cx: &mut Cx) {
        self.recording = false;
        self.view.view(ids!(recording_section.record_btn)).apply_over(
            cx,
            live! { draw_bg: { recording: 0.0 } },
        );
        self.view.label(ids!(recording_section.instruction))
            .set_text(cx, "评估中...");
        self.view.redraw(cx);
    }

    fn show_score(&mut self, cx: &mut Cx, score: u8) {
        self.has_score = true;
        self.score = score;

        // Update score display
        self.view.view(ids!(recording_section.score_display)).set_visible(cx, true);
        self.view.view(ids!(recording_section.score_display.score_circle)).apply_over(
            cx,
            live! { draw_bg: { score: (score as f64 / 100.0), visible: 1.0 } },
        );
        self.view.label(ids!(recording_section.score_display.score_circle.score_text))
            .set_text(cx, &format!("{}分", score));

        // Set feedback text
        let feedback = if score >= 90 {
            "太棒了！发音非常标准"
        } else if score >= 70 {
            "不错！继续保持"
        } else if score >= 50 {
            "还需要多练习"
        } else {
            "再试一次吧"
        };
        self.view.label(ids!(recording_section.score_display.score_feedback))
            .set_text(cx, feedback);

        // Update instruction
        self.view.label(ids!(recording_section.instruction))
            .set_text(cx, "评估完成");

        // Show action buttons
        self.view.button(ids!(action_row.retry_btn)).set_visible(cx, true);
        self.view.button(ids!(action_row.next_btn)).set_visible(cx, true);

        self.view.redraw(cx);
    }

    fn reset_recording(&mut self, cx: &mut Cx) {
        self.recording = false;
        self.has_score = false;
        self.score = 0;

        self.view.view(ids!(recording_section.record_btn)).apply_over(
            cx,
            live! { draw_bg: { recording: 0.0 } },
        );
        self.view.view(ids!(recording_section.score_display)).set_visible(cx, false);
        self.view.button(ids!(action_row.retry_btn)).set_visible(cx, false);
        self.view.button(ids!(action_row.next_btn)).set_visible(cx, false);
        self.view.label(ids!(recording_section.instruction))
            .set_text(cx, "点击麦克风按钮开始录音");

        self.view.redraw(cx);
    }
}

impl PronunciationViewRef {
    pub fn set_word(&self, cx: &mut Cx, word: &str, phonetic: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.label(ids!(word_card.word_label)).set_text(cx, word);
            inner.view.label(ids!(word_card.phonetic_label)).set_text(cx, phonetic);
            inner.reset_recording(cx);
        }
    }
}
