use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use widgets::theme::*;

    use crate::scenes::review::components::ReviewTabButton;
    use crate::scenes::review::components::SectionTitle;
    use crate::scenes::review::due_screen::DueScreen;
    use crate::scenes::review::mistakes_screen::MistakesScreen;
    use crate::scenes::review::mastered_screen::MasteredScreen;
    use crate::scenes::review::stats_screen::StatsScreen;

    pub ReviewScene = {{ReviewScene}} {
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
                    <Label> {
                        text: "📚 复习中心"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 18.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <View> { width: Fill }
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
pub struct ReviewScene {
    #[deref]
    view: View,
    #[rust]
    tab: ReviewTab,
}

impl Widget for ReviewScene {
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

impl ReviewScene {
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
