//! # Meaning View
//!
//! Practice view for meaning selection exercises (4 choices).

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;

    // Choice option button
    ChoiceOption = <Button> {
        width: Fill, height: Fit
        padding: {left: 20, right: 20, top: 16, bottom: 16}
        align: {x: 0.0, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            instance selected: 0.0
            instance correct: 0.0
            instance wrong: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                // Background colors
                let light_bg = mix((WHITE), (SLATE_50), self.hover);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.hover);
                let bg = mix(light_bg, dark_bg, self.dark_mode);

                // Selected state
                let selected_light = (ORANGE_100);
                let selected_dark = #7c2d12;
                let selected_bg = mix(selected_light, selected_dark, self.dark_mode);

                // Correct/wrong states
                let correct_bg = mix(#dcfce7, #14532d, self.dark_mode);
                let wrong_bg = mix(#fee2e2, #7f1d1d, self.dark_mode);

                let base_bg = mix(bg, selected_bg, self.selected);
                let final_bg = mix(mix(base_bg, correct_bg, self.correct), wrong_bg, self.wrong);
                sdf.fill(final_bg);

                // Border
                let light_border = (SLATE_200);
                let dark_border = (SLATE_700);
                let selected_border = (ACCENT_PRIMARY);
                let correct_border = (ACCENT_GREEN);
                let wrong_border = (ACCENT_RED);

                let base_border = mix(
                    mix(light_border, dark_border, self.dark_mode),
                    selected_border,
                    self.selected
                );
                let final_border = mix(mix(base_border, correct_border, self.correct), wrong_border, self.wrong);
                sdf.stroke(final_border, mix(1.0, 2.0, max(self.selected, max(self.correct, self.wrong))));

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance correct: 0.0
            instance wrong: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 14.0 }

            fn get_color(self) -> vec4 {
                let normal = mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                let correct_color = mix(#166534, #4ade80, self.dark_mode);
                let wrong_color = mix(#991b1b, #f87171, self.dark_mode);
                return mix(mix(normal, correct_color, self.correct), wrong_color, self.wrong);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    pub MeaningView = {{MeaningView}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5}
        spacing: 24

        // Word display
        word_card = <CardBase> {
            width: Fill, height: Fit
            padding: 32
            flow: Down
            align: {x: 0.5}
            spacing: 12

            word_label = <Label> {
                text: "abandon"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 36.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            phonetic_label = <Label> {
                text: "/əˈbændən/"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            // Play audio button
            play_btn = <Button> {
                width: Fit, height: Fit
                margin: {top: 8}
                padding: {left: 16, right: 16, top: 10, bottom: 10}
                text: "🔊 播放发音"
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
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((SLATE_700), (SLATE_200), self.dark_mode);
                    }
                }
            }
        }

        // Instruction
        instruction = <Label> {
            text: "选择正确的中文释义"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }

        // Choices
        choices = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 12

            choice_a = <ChoiceOption> { text: "A. 放弃" }
            choice_b = <ChoiceOption> { text: "B. 接受" }
            choice_c = <ChoiceOption> { text: "C. 支持" }
            choice_d = <ChoiceOption> { text: "D. 保留" }
        }

        // Feedback area
        feedback = <View> {
            width: Fill, height: Fit
            visible: false
            padding: 16
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance correct: 1.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    let correct_bg = mix(#dcfce7, #14532d, self.dark_mode);
                    let wrong_bg = mix(#fee2e2, #7f1d1d, self.dark_mode);
                    sdf.fill(mix(wrong_bg, correct_bg, self.correct));
                    return sdf.result;
                }
            }

            feedback_text = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    instance correct: 1.0
                    text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        let correct_color = mix(#166534, #4ade80, self.dark_mode);
                        let wrong_color = mix(#991b1b, #f87171, self.dark_mode);
                        return mix(wrong_color, correct_color, self.correct);
                    }
                }
            }
        }

        // Next button
        next_btn = <PrimaryButton> {
            text: "下一个"
            visible: false
            margin: {top: 16}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MeaningViewAction {
    None,
    PlayAudio,
    AnswerSelected { index: usize, correct: bool },
    Next,
}

#[derive(Live, LiveHook, Widget)]
pub struct MeaningView {
    #[deref]
    view: View,
    #[rust]
    correct_index: usize,
    #[rust]
    selected_index: Option<usize>,
    #[rust]
    answered: bool,
}

impl Widget for MeaningView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle play audio
        if self.view.button(ids!(word_card.play_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, MeaningViewAction::PlayAudio);
        }

        // Handle choice selection
        if !self.answered {
            let choices = [
                ids!(choices.choice_a),
                ids!(choices.choice_b),
                ids!(choices.choice_c),
                ids!(choices.choice_d),
            ];

            for (i, choice_id) in choices.iter().enumerate() {
                if self.view.button(*choice_id).clicked(actions) {
                    self.select_answer(cx, i);
                    let correct = i == self.correct_index;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MeaningViewAction::AnswerSelected { index: i, correct },
                    );
                }
            }
        }

        // Handle next button
        if self.view.button(ids!(next_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, MeaningViewAction::Next);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MeaningView {
    fn select_answer(&mut self, cx: &mut Cx, index: usize) {
        self.selected_index = Some(index);
        self.answered = true;

        let is_correct = index == self.correct_index;

        let choices = [
            ids!(choices.choice_a),
            ids!(choices.choice_b),
            ids!(choices.choice_c),
            ids!(choices.choice_d),
        ];

        // Update choice states
        for (i, choice_id) in choices.iter().enumerate() {
            if i == self.correct_index {
                // Show correct answer
                self.view.button(*choice_id).apply_over(
                    cx,
                    live! {
                        draw_bg: { correct: 1.0, wrong: 0.0, selected: 0.0 }
                        draw_text: { correct: 1.0, wrong: 0.0 }
                    },
                );
            } else if i == index && !is_correct {
                // Show wrong selection
                self.view.button(*choice_id).apply_over(
                    cx,
                    live! {
                        draw_bg: { correct: 0.0, wrong: 1.0, selected: 0.0 }
                        draw_text: { correct: 0.0, wrong: 1.0 }
                    },
                );
            }
        }

        // Show feedback
        self.view.view(ids!(feedback)).set_visible(cx, true);
        let feedback_text = if is_correct { "正确！" } else { "错误，正确答案已高亮显示" };
        self.view.label(ids!(feedback.feedback_text)).set_text(cx, feedback_text);
        self.view.view(ids!(feedback)).apply_over(
            cx,
            live! { draw_bg: { correct: (if is_correct { 1.0 } else { 0.0 }) } },
        );
        self.view.label(ids!(feedback.feedback_text)).apply_over(
            cx,
            live! { draw_text: { correct: (if is_correct { 1.0 } else { 0.0 }) } },
        );

        // Show next button
        self.view.button(ids!(next_btn)).set_visible(cx, true);

        self.view.redraw(cx);
    }

    fn reset(&mut self, cx: &mut Cx) {
        self.selected_index = None;
        self.answered = false;

        let choices = [
            ids!(choices.choice_a),
            ids!(choices.choice_b),
            ids!(choices.choice_c),
            ids!(choices.choice_d),
        ];

        for choice_id in choices.iter() {
            self.view.button(*choice_id).apply_over(
                cx,
                live! {
                    draw_bg: { correct: 0.0, wrong: 0.0, selected: 0.0 }
                    draw_text: { correct: 0.0, wrong: 0.0 }
                },
            );
        }

        self.view.view(ids!(feedback)).set_visible(cx, false);
        self.view.button(ids!(next_btn)).set_visible(cx, false);

        self.view.redraw(cx);
    }
}

impl MeaningViewRef {
    pub fn set_word(&self, cx: &mut Cx, word: &str, phonetic: &str, correct_meaning: &str, wrong_meanings: &[String]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.label(ids!(word_card.word_label)).set_text(cx, word);
            inner.view.label(ids!(word_card.phonetic_label)).set_text(cx, phonetic);

            // Shuffle and set choices (for now, correct is always A)
            // TODO: Implement proper shuffling
            inner.correct_index = 0;

            let choices = [
                ids!(choices.choice_a),
                ids!(choices.choice_b),
                ids!(choices.choice_c),
                ids!(choices.choice_d),
            ];

            inner.view.button(choices[0]).apply_over(cx, live! { text: (format!("A. {}", correct_meaning)) });

            for (i, wrong) in wrong_meanings.iter().take(3).enumerate() {
                let prefix = match i {
                    0 => "B",
                    1 => "C",
                    _ => "D",
                };
                inner.view.button(choices[i + 1]).apply_over(cx, live! { text: (format!("{}. {}", prefix, wrong)) });
            }

            inner.reset(cx);
        }
    }
}
