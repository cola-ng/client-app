//! # Recognition View
//!
//! Practice view for flip card recognition with self-evaluation.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;

    // Flip card component
    FlipCard = <View> {
        width: Fill, height: 300
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance flipped: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Card with subtle 3D effect
                sdf.box(4., 6., self.rect_size.x - 4., self.rect_size.y - 4., 16.0);
                let shadow = mix(vec4(0., 0., 0., 0.08), vec4(0., 0., 0., 0.2), self.dark_mode);
                sdf.fill(shadow);

                sdf.box(0., 0., self.rect_size.x - 4., self.rect_size.y - 4., 16.0);
                let light_bg = mix((WHITE), (SLATE_50), self.hover);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.hover);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));

                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }

        // Front side (word only)
        front = <View> {
            width: Fill, height: Fill
            flow: Down
            align: {x: 0.5, y: 0.5}
            spacing: 16

            word_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 36.0 }
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

            hint_label = <Label> {
                text: "点击翻转查看释义"
                margin: {top: 24}
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Back side (meaning revealed)
        back = <View> {
            width: Fill, height: Fill
            visible: false
            flow: Down
            align: {x: 0.5, y: 0.5}
            spacing: 12

            word_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 24.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            meaning_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 28.0 }
                    fn get_color(self) -> vec4 {
                        return mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                    }
                }
            }

            phonetic_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }

    // Self evaluation button
    RatingButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 16, bottom: 16}
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            instance tint_r: 0.0
            instance tint_g: 0.0
            instance tint_b: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                let tint = vec4(self.tint_r, self.tint_g, self.tint_b, 1.0);
                let light_bg = mix((WHITE), tint, 0.1);
                let dark_bg = mix((SLATE_800), tint, 0.2);
                let hover_light = mix(light_bg, tint, 0.15);
                let hover_dark = mix(dark_bg, tint, 0.3);

                let base = mix(light_bg, dark_bg, self.dark_mode);
                let hovered = mix(hover_light, hover_dark, self.dark_mode);
                sdf.fill(mix(base, hovered, self.hover));

                let border = mix(tint, tint, 0.5);
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
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
        }
    }

    pub RecognitionView = {{RecognitionView}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5}
        spacing: 24

        // Flip card
        flip_card = <FlipCard> {
            front = {
                word_label = { text: "abandon" }
                phonetic_label = { text: "/əˈbændən/" }
            }
            back = {
                word_label = { text: "abandon" }
                meaning_label = { text: "放弃" }
                phonetic_label = { text: "/əˈbændən/" }
            }
        }

        // Self evaluation section
        evaluation_section = <View> {
            width: Fill, height: Fit
            visible: false
            flow: Down
            spacing: 16

            eval_label = <Label> {
                text: "你记住了吗？"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            ratings = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12

                rating_1 = <RatingButton> {
                    text: "😕 完全忘了"
                    draw_bg: { tint_r: 0.937, tint_g: 0.267, tint_b: 0.267 }  // red
                }

                rating_2 = <RatingButton> {
                    text: "🤔 有点印象"
                    draw_bg: { tint_r: 0.961, tint_g: 0.620, tint_b: 0.043 }  // amber
                }

                rating_3 = <RatingButton> {
                    text: "😊 记得住"
                    draw_bg: { tint_r: 0.133, tint_g: 0.773, tint_b: 0.506 }  // green
                }

                rating_4 = <RatingButton> {
                    text: "😎 很熟悉"
                    draw_bg: { tint_r: 0.231, tint_g: 0.510, tint_b: 0.965 }  // blue
                }
            }

            keyboard_hint = <Label> {
                text: "快捷键: 1-4 评估，空格翻卡"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum RecognitionViewAction {
    None,
    Flip,
    Rate { rating: u8 },
}

#[derive(Live, LiveHook, Widget)]
pub struct RecognitionView {
    #[deref]
    view: View,
    #[rust]
    flipped: bool,
}

impl Widget for RecognitionView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Handle flip card click
        self.handle_flip_card(cx, event, scope);

        // Handle keyboard shortcuts
        self.handle_keyboard(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle rating buttons
        let rating_buttons = [
            (ids!(evaluation_section.ratings.rating_1), 1u8),
            (ids!(evaluation_section.ratings.rating_2), 2u8),
            (ids!(evaluation_section.ratings.rating_3), 3u8),
            (ids!(evaluation_section.ratings.rating_4), 4u8),
        ];

        for (btn_id, rating) in rating_buttons {
            if self.view.button(btn_id).clicked(actions) {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    RecognitionViewAction::Rate { rating },
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl RecognitionView {
    fn handle_flip_card(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let card = self.view.view(ids!(flip_card));

        match event.hits(cx, card.area()) {
            Hit::FingerHoverIn(_) => {
                card.apply_over(cx, live! { draw_bg: { hover: 1.0 } });
                self.view.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                card.apply_over(cx, live! { draw_bg: { hover: 0.0 } });
                self.view.redraw(cx);
            }
            Hit::FingerUp(_) => {
                self.flip(cx);
                cx.widget_action(self.widget_uid(), &scope.path, RecognitionViewAction::Flip);
            }
            _ => {}
        }
    }

    fn handle_keyboard(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::KeyDown(ke) = event {
            match ke.key_code {
                KeyCode::Space => {
                    self.flip(cx);
                    cx.widget_action(self.widget_uid(), &scope.path, RecognitionViewAction::Flip);
                }
                KeyCode::Key1 if self.flipped => {
                    cx.widget_action(self.widget_uid(), &scope.path, RecognitionViewAction::Rate { rating: 1 });
                }
                KeyCode::Key2 if self.flipped => {
                    cx.widget_action(self.widget_uid(), &scope.path, RecognitionViewAction::Rate { rating: 2 });
                }
                KeyCode::Key3 if self.flipped => {
                    cx.widget_action(self.widget_uid(), &scope.path, RecognitionViewAction::Rate { rating: 3 });
                }
                KeyCode::Key4 if self.flipped => {
                    cx.widget_action(self.widget_uid(), &scope.path, RecognitionViewAction::Rate { rating: 4 });
                }
                _ => {}
            }
        }
    }

    fn flip(&mut self, cx: &mut Cx) {
        self.flipped = !self.flipped;

        let flipped_val = if self.flipped { 1.0 } else { 0.0 };
        self.view.view(ids!(flip_card)).apply_over(
            cx,
            live! { draw_bg: { flipped: (flipped_val) } },
        );

        // Toggle front/back visibility
        self.view.view(ids!(flip_card.front)).set_visible(cx, !self.flipped);
        self.view.view(ids!(flip_card.back)).set_visible(cx, self.flipped);

        // Show evaluation section when flipped
        self.view.view(ids!(evaluation_section)).set_visible(cx, self.flipped);

        self.view.redraw(cx);
    }

    fn reset(&mut self, cx: &mut Cx) {
        self.flipped = false;
        self.view.view(ids!(flip_card)).apply_over(cx, live! { draw_bg: { flipped: 0.0 } });
        self.view.view(ids!(flip_card.front)).set_visible(cx, true);
        self.view.view(ids!(flip_card.back)).set_visible(cx, false);
        self.view.view(ids!(evaluation_section)).set_visible(cx, false);
        self.view.redraw(cx);
    }
}

impl RecognitionViewRef {
    pub fn set_word(&self, cx: &mut Cx, word: &str, meaning: &str, phonetic: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Front side
            inner.view.label(ids!(flip_card.front.word_label)).set_text(cx, word);
            inner.view.label(ids!(flip_card.front.phonetic_label)).set_text(cx, phonetic);

            // Back side
            inner.view.label(ids!(flip_card.back.word_label)).set_text(cx, word);
            inner.view.label(ids!(flip_card.back.meaning_label)).set_text(cx, meaning);
            inner.view.label(ids!(flip_card.back.phonetic_label)).set_text(cx, phonetic);

            inner.reset(cx);
        }
    }
}
