//! # Phrase View
//!
//! Practice view for phrase fill-in-the-blank exercises.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;
    use crate::screens::review::components::SecondaryButton;

    // Inline blank input for sentences
    BlankInput = <TextInput> {
        width: 120, height: Fit
        padding: {left: 8, right: 8, top: 4, bottom: 4}

        draw_bg: {
            instance dark_mode: 0.0
            instance error: 0.0
            instance correct: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Underline style input
                sdf.rect(0., self.rect_size.y - 2., self.rect_size.x, 2.);
                let normal = mix((SLATE_400), (SLATE_500), self.dark_mode);
                let error_color = (ACCENT_RED);
                let correct_color = (ACCENT_GREEN);
                let color = mix(mix(normal, error_color, self.error), correct_color, self.correct);
                sdf.fill(color);

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    pub PhraseView = {{PhraseView}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5}
        spacing: 24

        // Word hint at top
        word_hint = <CardBase> {
            width: Fill, height: Fit
            padding: 20
            flow: Down
            align: {x: 0.5}
            spacing: 8

            hint_label = <Label> {
                text: "用以下单词填空"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            word_label = <Label> {
                text: "abandon"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                    fn get_color(self) -> vec4 {
                        return mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                    }
                }
            }

            meaning_label = <Label> {
                text: "放弃"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Sentence card
        sentence_card = <CardBase> {
            width: Fill, height: Fit
            padding: 32
            flow: Down
            spacing: 20

            instruction = <Label> {
                text: "在空白处填入单词的正确形式："
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            // Sentence with inline input
            // Note: For complex sentence rendering with inline blanks,
            // we'll use a simplified approach with before/after parts
            sentence_container = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                spacing: 4

                sentence_before = <Label> {
                    text: "Don't "
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 18.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                blank_input = <BlankInput> {
                    empty_message: "___"
                }

                sentence_after = <Label> {
                    text: " your dreams."
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 18.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }
            }

            // Chinese translation
            translation = <Label> {
                text: "不要放弃你的梦想。"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            submit_btn = <PrimaryButton> {
                text: "检查答案"
                margin: {top: 8}
            }
        }

        // Result section
        result_section = <View> {
            width: Fill, height: Fit
            visible: false
            flow: Down
            spacing: 16

            result_card = <CardBase> {
                width: Fill, height: Fit
                padding: 24
                flow: Down
                align: {x: 0.5}
                spacing: 12

                result_icon = <View> {
                    width: 56, height: 56
                    show_bg: true
                    draw_bg: {
                        instance correct: 1.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let c = self.rect_size * 0.5;
                            sdf.circle(c.x, c.y, 24.);
                            sdf.fill(mix((ACCENT_RED), (ACCENT_GREEN), self.correct));

                            if self.correct > 0.5 {
                                sdf.move_to(c.x - 8., c.y);
                                sdf.line_to(c.x - 2., c.y + 6.);
                                sdf.line_to(c.x + 10., c.y - 6.);
                                sdf.stroke((WHITE), 2.5);
                            } else {
                                sdf.move_to(c.x - 6., c.y - 6.);
                                sdf.line_to(c.x + 6., c.y + 6.);
                                sdf.stroke((WHITE), 2.5);
                                sdf.move_to(c.x + 6., c.y - 6.);
                                sdf.line_to(c.x - 6., c.y + 6.);
                                sdf.stroke((WHITE), 2.5);
                            }

                            return sdf.result;
                        }
                    }
                }

                result_text = <Label> {
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                // Show user's answer vs correct
                answer_comparison = <View> {
                    width: Fill, height: Fit
                    visible: false
                    flow: Down
                    spacing: 8
                    align: {x: 0.5}

                    user_answer_row = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 8

                        user_label = <Label> {
                            text: "你的答案："
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        user_answer = <Label> {
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                                color: (ACCENT_RED)
                            }
                        }
                    }

                    correct_answer_row = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        spacing: 8

                        correct_label = <Label> {
                            text: "正确答案："
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        correct_answer = <Label> {
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                                color: (ACCENT_GREEN)
                            }
                        }
                    }
                }
            }

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
pub enum PhraseViewAction {
    None,
    Submit { answer: String, correct: bool },
    Retry,
    Next,
}

#[derive(Live, LiveHook, Widget)]
pub struct PhraseView {
    #[deref]
    view: View,
    #[rust]
    correct_answer: String,
    #[rust]
    answered: bool,
}

impl Widget for PhraseView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle submit button
        if self.view.button(ids!(sentence_card.submit_btn)).clicked(actions) && !self.answered {
            self.check_answer(cx, scope);
        }

        // Handle retry button
        if self.view.button(ids!(result_section.action_row.retry_btn)).clicked(actions) {
            self.reset(cx);
            cx.widget_action(self.widget_uid(), &scope.path, PhraseViewAction::Retry);
        }

        // Handle next button
        if self.view.button(ids!(result_section.action_row.next_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, PhraseViewAction::Next);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl PhraseView {
    fn check_answer(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let user_input = self.view.text_input(ids!(sentence_card.sentence_container.blank_input)).text();
        let user_answer = user_input.trim().to_lowercase();
        let correct = user_answer == self.correct_answer.to_lowercase();

        self.answered = true;

        // Update blank input style
        self.view.text_input(ids!(sentence_card.sentence_container.blank_input)).apply_over(
            cx,
            live! {
                draw_bg: {
                    correct: (if correct { 1.0 } else { 0.0 }),
                    error: (if correct { 0.0 } else { 1.0 })
                }
            },
        );

        // Show result section
        self.view.view(ids!(result_section)).set_visible(cx, true);
        self.view.button(ids!(sentence_card.submit_btn)).set_visible(cx, false);

        // Update result icon
        self.view.view(ids!(result_section.result_card.result_icon)).apply_over(
            cx,
            live! { draw_bg: { correct: (if correct { 1.0 } else { 0.0 }) } },
        );

        // Update result text
        let result_text = if correct { "正确！" } else { "不正确" };
        self.view.label(ids!(result_section.result_card.result_text)).set_text(cx, result_text);

        // Show comparison if wrong
        if !correct {
            self.view.view(ids!(result_section.result_card.answer_comparison)).set_visible(cx, true);
            self.view.label(ids!(result_section.result_card.answer_comparison.user_answer_row.user_answer))
                .set_text(cx, &user_answer);
            self.view.label(ids!(result_section.result_card.answer_comparison.correct_answer_row.correct_answer))
                .set_text(cx, &self.correct_answer);
        }

        cx.widget_action(
            self.widget_uid(),
            &scope.path,
            PhraseViewAction::Submit {
                answer: user_answer,
                correct,
            },
        );

        self.view.redraw(cx);
    }

    fn reset(&mut self, cx: &mut Cx) {
        self.answered = false;

        self.view.text_input(ids!(sentence_card.sentence_container.blank_input)).set_text(cx, "");
        self.view.text_input(ids!(sentence_card.sentence_container.blank_input)).apply_over(
            cx,
            live! { draw_bg: { correct: 0.0, error: 0.0 } },
        );
        self.view.view(ids!(result_section)).set_visible(cx, false);
        self.view.view(ids!(result_section.result_card.answer_comparison)).set_visible(cx, false);
        self.view.button(ids!(sentence_card.submit_btn)).set_visible(cx, true);

        self.view.redraw(cx);
    }
}

impl PhraseViewRef {
    pub fn set_phrase(&self, cx: &mut Cx, word: &str, meaning: &str, sentence_before: &str, sentence_after: &str, translation: &str, answer: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.correct_answer = answer.to_string();

            // Set word hint
            inner.view.label(ids!(word_hint.word_label)).set_text(cx, word);
            inner.view.label(ids!(word_hint.meaning_label)).set_text(cx, meaning);

            // Set sentence parts
            inner.view.label(ids!(sentence_card.sentence_container.sentence_before)).set_text(cx, sentence_before);
            inner.view.label(ids!(sentence_card.sentence_container.sentence_after)).set_text(cx, sentence_after);
            inner.view.label(ids!(sentence_card.translation)).set_text(cx, translation);

            inner.reset(cx);
        }
    }
}
