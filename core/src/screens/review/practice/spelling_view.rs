//! # Spelling View
//!
//! Practice view for spelling exercises (listen and type).

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;
    use crate::screens::review::components::SecondaryButton;

    pub SpellingView = {{SpellingView}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5}
        spacing: 24

        // Audio section
        audio_card = <CardBase> {
            width: Fill, height: Fit
            padding: 32
            flow: Down
            align: {x: 0.5}
            spacing: 16

            instruction = <Label> {
                text: "听音频，输入你听到的单词"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            // Audio play button
            play_btn = <View> {
                width: 80, height: 80
                align: {x: 0.5, y: 0.5}
                cursor: Hand
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    instance hover: 0.0
                    instance playing: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let c = self.rect_size * 0.5;

                        // Background circle
                        sdf.circle(c.x, c.y, 36.);
                        let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                        let hover_bg = mix((ORANGE_200), (SLATE_600), self.dark_mode);
                        sdf.fill(mix(bg, hover_bg, self.hover));

                        // Play/pause icon
                        if self.playing < 0.5 {
                            // Play triangle
                            sdf.move_to(c.x - 8., c.y - 12.);
                            sdf.line_to(c.x + 12., c.y);
                            sdf.line_to(c.x - 8., c.y + 12.);
                            sdf.close_path();
                            sdf.fill((ACCENT_PRIMARY));
                        } else {
                            // Pause bars
                            sdf.box(c.x - 10., c.y - 10., 8., 20., 2.);
                            sdf.fill((ACCENT_PRIMARY));
                            sdf.box(c.x + 2., c.y - 10., 8., 20., 2.);
                            sdf.fill((ACCENT_PRIMARY));
                        }

                        return sdf.result;
                    }
                }
            }

            play_label = <Label> {
                text: "点击播放"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Input section
        input_card = <CardBase> {
            width: Fill, height: Fit
            padding: 24
            flow: Down
            spacing: 16

            input_label = <Label> {
                text: "输入单词："
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((SLATE_600), (SLATE_400), self.dark_mode);
                    }
                }
            }

            word_input = <TextInput> {
                width: Fill, height: Fit
                padding: {left: 16, right: 16, top: 14, bottom: 14}
                empty_message: "输入你听到的单词..."

                draw_bg: {
                    instance dark_mode: 0.0
                    instance error: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                        sdf.fill(bg);
                        let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                        let error_border = (ACCENT_RED);
                        sdf.stroke(mix(border, error_border, self.error), 1.0);
                        return sdf.result;
                    }
                }

                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 18.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            submit_btn = <PrimaryButton> {
                text: "检查答案"
                margin: {top: 8}
            }
        }

        // Result section (shown after submission)
        result_section = <View> {
            width: Fill, height: Fit
            visible: false
            flow: Down
            spacing: 16

            result_card = <CardBase> {
                width: Fill, height: Fit
                padding: 24
                flow: Down
                spacing: 12

                result_icon = <View> {
                    width: Fill, height: Fit
                    align: {x: 0.5}

                    icon = <View> {
                        width: 64, height: 64
                        show_bg: true
                        draw_bg: {
                            instance correct: 1.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let c = self.rect_size * 0.5;
                                sdf.circle(c.x, c.y, 28.);
                                let correct_bg = (ACCENT_GREEN);
                                let wrong_bg = (ACCENT_RED);
                                sdf.fill(mix(wrong_bg, correct_bg, self.correct));

                                // Checkmark or X
                                if self.correct > 0.5 {
                                    sdf.move_to(c.x - 10., c.y);
                                    sdf.line_to(c.x - 3., c.y + 8.);
                                    sdf.line_to(c.x + 12., c.y - 8.);
                                    sdf.stroke((WHITE), 3.0);
                                } else {
                                    sdf.move_to(c.x - 8., c.y - 8.);
                                    sdf.line_to(c.x + 8., c.y + 8.);
                                    sdf.stroke((WHITE), 3.0);
                                    sdf.move_to(c.x + 8., c.y - 8.);
                                    sdf.line_to(c.x - 8., c.y + 8.);
                                    sdf.stroke((WHITE), 3.0);
                                }

                                return sdf.result;
                            }
                        }
                    }
                }

                result_text = <Label> {
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                // Show correct answer if wrong
                correct_answer = <View> {
                    width: Fill, height: Fit
                    visible: false
                    flow: Down
                    spacing: 4
                    align: {x: 0.5}

                    correct_label = <Label> {
                        text: "正确答案："
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    correct_word = <Label> {
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 24.0 }
                            fn get_color(self) -> vec4 {
                                return mix((ACCENT_GREEN), (ACCENT_GREEN_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Similarity score
                similarity_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {x: 0.5, y: 0.5}
                    spacing: 8

                    similarity_label = <Label> {
                        text: "相似度："
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    similarity_value = <Label> {
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
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

                retry_btn = <SecondaryButton> { text: "再试一次" }
                next_btn = <PrimaryButton> { text: "下一个" }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum SpellingViewAction {
    None,
    PlayAudio,
    Submit { answer: String, correct: bool, similarity: f64 },
    Retry,
    Next,
}

#[derive(Live, LiveHook, Widget)]
pub struct SpellingView {
    #[deref]
    view: View,
    #[rust]
    correct_word: String,
    #[rust]
    answered: bool,
    #[rust]
    playing: bool,
}

impl Widget for SpellingView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Handle play button hover/click
        self.handle_play_button(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle submit button
        if self.view.button(ids!(input_card.submit_btn)).clicked(actions) && !self.answered {
            self.check_answer(cx, scope);
        }

        // Handle retry button
        if self.view.button(ids!(result_section.action_row.retry_btn)).clicked(actions) {
            self.reset(cx);
            cx.widget_action(self.widget_uid(), &scope.path, SpellingViewAction::Retry);
        }

        // Handle next button
        if self.view.button(ids!(result_section.action_row.next_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, SpellingViewAction::Next);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl SpellingView {
    fn handle_play_button(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let play_btn = self.view.view(ids!(audio_card.play_btn));

        match event.hits(cx, play_btn.area()) {
            Hit::FingerHoverIn(_) => {
                play_btn.apply_over(cx, live! { draw_bg: { hover: 1.0 } });
                self.view.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                play_btn.apply_over(cx, live! { draw_bg: { hover: 0.0 } });
                self.view.redraw(cx);
            }
            Hit::FingerUp(_) => {
                self.playing = !self.playing;
                play_btn.apply_over(cx, live! { draw_bg: { playing: (if self.playing { 1.0 } else { 0.0 }) } });
                self.view.label(ids!(audio_card.play_label))
                    .set_text(cx, if self.playing { "播放中..." } else { "点击播放" });
                cx.widget_action(self.widget_uid(), &scope.path, SpellingViewAction::PlayAudio);
                self.view.redraw(cx);
            }
            _ => {}
        }
    }

    fn check_answer(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let user_input = self.view.text_input(ids!(input_card.word_input)).text();
        let user_answer = user_input.trim().to_lowercase();
        let correct = user_answer == self.correct_word.to_lowercase();

        // Calculate similarity using Levenshtein distance
        let similarity = Self::calculate_similarity(&user_answer, &self.correct_word.to_lowercase());

        self.answered = true;

        // Update UI
        self.view.view(ids!(result_section)).set_visible(cx, true);
        self.view.view(ids!(input_card)).set_visible(cx, false);

        // Set result icon
        self.view.view(ids!(result_section.result_card.result_icon.icon)).apply_over(
            cx,
            live! { draw_bg: { correct: (if correct { 1.0 } else { 0.0 }) } },
        );

        // Set result text
        let result_text = if correct {
            "正确！拼写完全正确"
        } else if similarity >= 0.7 {
            "接近正确！"
        } else {
            "不正确"
        };
        self.view.label(ids!(result_section.result_card.result_text)).set_text(cx, result_text);

        // Show correct answer if wrong
        if !correct {
            self.view.view(ids!(result_section.result_card.correct_answer)).set_visible(cx, true);
            self.view.label(ids!(result_section.result_card.correct_answer.correct_word))
                .set_text(cx, &self.correct_word);
        }

        // Show similarity score
        self.view.label(ids!(result_section.result_card.similarity_row.similarity_value))
            .set_text(cx, &format!("{:.0}%", similarity * 100.0));

        cx.widget_action(
            self.widget_uid(),
            &scope.path,
            SpellingViewAction::Submit {
                answer: user_answer,
                correct,
                similarity,
            },
        );

        self.view.redraw(cx);
    }

    fn calculate_similarity(a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        let len_a = a.len();
        let len_b = b.len();
        let max_len = len_a.max(len_b);

        // Simple Levenshtein distance
        let mut prev: Vec<usize> = (0..=len_b).collect();
        let mut curr = vec![0; len_b + 1];

        for (i, ca) in a.chars().enumerate() {
            curr[0] = i + 1;
            for (j, cb) in b.chars().enumerate() {
                let cost = if ca == cb { 0 } else { 1 };
                curr[j + 1] = (prev[j + 1] + 1)
                    .min(curr[j] + 1)
                    .min(prev[j] + cost);
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        let distance = prev[len_b];
        1.0 - (distance as f64 / max_len as f64)
    }

    fn reset(&mut self, cx: &mut Cx) {
        self.answered = false;
        self.playing = false;

        self.view.text_input(ids!(input_card.word_input)).set_text(cx, "");
        self.view.view(ids!(input_card)).set_visible(cx, true);
        self.view.view(ids!(result_section)).set_visible(cx, false);
        self.view.view(ids!(result_section.result_card.correct_answer)).set_visible(cx, false);
        self.view.view(ids!(audio_card.play_btn)).apply_over(cx, live! { draw_bg: { playing: 0.0 } });
        self.view.label(ids!(audio_card.play_label)).set_text(cx, "点击播放");

        self.view.redraw(cx);
    }
}

impl SpellingViewRef {
    pub fn set_word(&self, cx: &mut Cx, word: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.correct_word = word.to_string();
            inner.reset(cx);
        }
    }
}
