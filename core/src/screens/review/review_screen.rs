use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_component::*;

    use ::widgets::theme::*;

    use crate::screens::review::components::ReviewTabButton;
    use crate::screens::review::components::SectionTitle;
    use crate::screens::review::due_screen::DueScreen;
    use crate::screens::review::mistakes_screen::MistakesScreen;
    use crate::screens::review::mastered_screen::MasteredScreen;
    use crate::screens::review::stats_screen::StatsScreen;

    pub ReviewScreen = {{ReviewScreen}} {
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
                spacing: 12
                padding: {left: 16, right: 16, top: 16, bottom: 16}

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    tabs = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        due_tab = <ReviewTabButton> {
                            text: "待复习"
                            draw_bg: { selected: 1.0 }
                            draw_text: { selected: 1.0 }
                        }
                        mistakes_tab = <ReviewTabButton> { text: "易错点" }
                        mastered_tab = <ReviewTabButton> { text: "已掌握" }
                        stats_tab = <ReviewTabButton> { text: "统计" }
                    }
                    <View> { width: Fill }
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

                pages = <PageFlip> {
                    width: Fill, height: Fit
                    active_page: due_page
                    due_page = <DueScreen> {}
                    mistakes_page = <MistakesScreen> {}
                    mastered_page = <MasteredScreen> {}
                    stats_page = <StatsScreen> {}
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum ReviewTab {
    #[default]
    Due,
    Mistakes,
    Mastered,
    Stats,
}

#[derive(Live, LiveHook, Widget)]
pub struct ReviewScreen {
    #[deref]
    view: View,
    #[rust]
    tab: ReviewTab,
}

impl Widget for ReviewScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.due_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Due;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.mistakes_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Mistakes;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.mastered_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Mastered;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.stats_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Stats;
            self.apply_tab_state(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReviewScreen {
    fn apply_tab_state(&mut self, cx: &mut Cx) {
        let is_due = self.tab == ReviewTab::Due;
        let is_mistakes = self.tab == ReviewTab::Mistakes;
        let is_mastered = self.tab == ReviewTab::Mastered;
        let is_stats = self.tab == ReviewTab::Stats;

        self.view
            .button(ids!(content_scroll.content.header_row.tabs.due_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_due { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_due { 1.0 } else { 0.0 }) }
                },
            );
        self.view
            .button(ids!(content_scroll.content.header_row.tabs.mistakes_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_mistakes { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_mistakes { 1.0 } else { 0.0 }) }
                },
            );
        self.view
            .button(ids!(content_scroll.content.header_row.tabs.mastered_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_mastered { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_mastered { 1.0 } else { 0.0 }) }
                },
            );
        self.view
            .button(ids!(content_scroll.content.header_row.tabs.stats_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_stats { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_stats { 1.0 } else { 0.0 }) }
                },
            );

        let page = match self.tab {
            ReviewTab::Due => live_id!(due_page),
            ReviewTab::Mistakes => live_id!(mistakes_page),
            ReviewTab::Mastered => live_id!(mastered_page),
            ReviewTab::Stats => live_id!(stats_page),
        };

        self.view
            .page_flip(ids!(content_scroll.content.pages))
            .set_active_page(cx, page);

        self.view.redraw(cx);
    }
}
