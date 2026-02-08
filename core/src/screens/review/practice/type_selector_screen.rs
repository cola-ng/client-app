//! # Review Type Selector Screen
//!
//! Screen for selecting the type of review exercise to practice.

use makepad_widgets::*;

use super::types::ReviewType;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;

    // Type selector card
    TypeCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 20
        flow: Right
        align: {y: 0.5}
        spacing: 16
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            instance selected: 0.0
            instance border_radius: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                let light_bg = mix((WHITE), (SLATE_50), self.hover);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.hover);
                let bg = mix(light_bg, dark_bg, self.dark_mode);

                // Selected state - orange tint
                let selected_light = (ORANGE_50);
                let selected_dark = #7c2d12;
                let selected_bg = mix(selected_light, selected_dark, self.dark_mode);
                let final_bg = mix(bg, selected_bg, self.selected);

                sdf.fill(final_bg);

                // Border
                let light_border = mix((SLATE_200), (ACCENT_PRIMARY), self.selected);
                let dark_border = mix((SLATE_700), (ACCENT_PRIMARY), self.selected);
                let border = mix(light_border, dark_border, self.dark_mode);
                sdf.stroke(border, mix(1.0, 2.0, self.selected));

                return sdf.result;
            }
        }

        type_icon = <View> {
            width: 56, height: 56
            align: {x: 0.5, y: 0.5}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                    let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(bg);
                    return sdf.result;
                }
            }

            icon_label = <Label> {
                draw_text: {
                    text_style: { font_size: 24.0 }
                }
            }
        }

        type_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            type_name = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            type_desc = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        select_indicator = <View> {
            width: 24, height: 24
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance selected: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Outer circle
                    sdf.circle(c.x, c.y, 10.);
                    let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                    let selected_border = (ACCENT_PRIMARY);
                    sdf.stroke(mix(border, selected_border, self.selected), 2.0);

                    // Inner dot when selected
                    if self.selected > 0.5 {
                        sdf.circle(c.x, c.y, 5.);
                        sdf.fill((ACCENT_PRIMARY));
                    }

                    return sdf.result;
                }
            }
        }
    }

    pub TypeSelectorScreen = {{TypeSelectorScreen}} {
        width: Fill, height: Fill
        flow: Down
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
                spacing: 16
                padding: {left: 24, right: 24, top: 24, bottom: 24}

                // Header
                header = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    back_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}

                        back_btn = <Button> {
                            width: Fit, height: Fit
                            padding: {left: 8, right: 12, top: 8, bottom: 8}
                            text: "← 返回"
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 { return (TRANSPARENT); }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((SLATE_600), (SLATE_400), self.dark_mode);
                                }
                            }
                        }
                    }

                    title = <Label> {
                        text: "选择练习类型"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 24.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    subtitle = <Label> {
                        text: "选择一种练习方式来复习你的单词"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Type cards
                types_container = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    pronunciation_card = <TypeCard> {
                        type_icon = { icon_label = { text: "🎤" } }
                        type_content = {
                            type_name = { text: "发音练习" }
                            type_desc = { text: "录音与原声对比，AI评分" }
                        }
                    }

                    meaning_card = <TypeCard> {
                        type_icon = { icon_label = { text: "📝" } }
                        type_content = {
                            type_name = { text: "词义选择" }
                            type_desc = { text: "四选一，选择正确的中文释义" }
                        }
                    }

                    spelling_card = <TypeCard> {
                        type_icon = { icon_label = { text: "✍️" } }
                        type_content = {
                            type_name = { text: "听写练习" }
                            type_desc = { text: "听音频，输入正确拼写" }
                        }
                    }

                    recognition_card = <TypeCard> {
                        type_icon = { icon_label = { text: "🔄" } }
                        type_content = {
                            type_name = { text: "翻卡识别" }
                            type_desc = { text: "翻卡片，自我评估掌握程度" }
                        }
                    }

                    phrase_card = <TypeCard> {
                        type_icon = { icon_label = { text: "📜" } }
                        type_content = {
                            type_name = { text: "短语填空" }
                            type_desc = { text: "在句子中填入正确的单词" }
                        }
                    }
                }

                // Start button
                start_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {x: 1.0}
                    margin: {top: 16}

                    start_btn = <PrimaryButton> {
                        text: "开始练习"
                        padding: {left: 24, right: 24, top: 12, bottom: 12}
                        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 14.0 } }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum TypeSelectorAction {
    None,
    Back,
    StartPractice(ReviewType),
}

#[derive(Live, LiveHook, Widget)]
pub struct TypeSelectorScreen {
    #[deref]
    view: View,
    #[rust]
    selected_type: ReviewType,
}

impl Widget for TypeSelectorScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle back button
        if self.view.button(ids!(content_scroll.content.header.back_row.back_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, TypeSelectorAction::Back);
        }

        // Handle type card clicks
        self.handle_type_card_clicks(cx, event);

        // Handle start button
        if self.view.button(ids!(content_scroll.content.start_container.start_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                TypeSelectorAction::StartPractice(self.selected_type),
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl TypeSelectorScreen {
    fn handle_type_card_clicks(&mut self, cx: &mut Cx, event: &Event) {
        let cards = [
            (ids!(content_scroll.content.types_container.pronunciation_card), ReviewType::Pronunciation),
            (ids!(content_scroll.content.types_container.meaning_card), ReviewType::Meaning),
            (ids!(content_scroll.content.types_container.spelling_card), ReviewType::Spelling),
            (ids!(content_scroll.content.types_container.recognition_card), ReviewType::Recognition),
            (ids!(content_scroll.content.types_container.phrase_card), ReviewType::Phrase),
        ];

        for (card_id, review_type) in cards {
            let card = self.view.view(card_id);

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
                    self.select_type(cx, review_type);
                }
                _ => {}
            }
        }
    }

    fn select_type(&mut self, cx: &mut Cx, review_type: ReviewType) {
        self.selected_type = review_type;
        self.update_selection_ui(cx);
    }

    fn update_selection_ui(&mut self, cx: &mut Cx) {
        let cards = [
            (ids!(content_scroll.content.types_container.pronunciation_card), ReviewType::Pronunciation),
            (ids!(content_scroll.content.types_container.meaning_card), ReviewType::Meaning),
            (ids!(content_scroll.content.types_container.spelling_card), ReviewType::Spelling),
            (ids!(content_scroll.content.types_container.recognition_card), ReviewType::Recognition),
            (ids!(content_scroll.content.types_container.phrase_card), ReviewType::Phrase),
        ];

        for (card_id, review_type) in cards {
            let selected = if review_type == self.selected_type { 1.0 } else { 0.0 };
            self.view.view(card_id).apply_over(
                cx,
                live! {
                    draw_bg: { selected: (selected) }
                    select_indicator = { draw_bg: { selected: (selected) } }
                },
            );
        }

        self.view.redraw(cx);
    }
}

impl TypeSelectorScreenRef {
    pub fn set_selected_type(&self, cx: &mut Cx, review_type: ReviewType) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.selected_type = review_type;
            inner.update_selection_ui(cx);
        }
    }
}
