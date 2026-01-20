use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Orange theme accent colors
    ACCENT_ORANGE = #f97316
    ORANGE_100 = #ffedd5
    ORANGE_300 = #fdba74
    ORANGE_900 = #7c2d12

    use crate::screens::review::components::ReviewTabButton;
    use crate::screens::review::components::SectionTitle;
    use crate::screens::review::components::StatsPanel;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;
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
                spacing: 16
                padding: {left: 16, right: 16, top: 16, bottom: 16}

                // Header card with title, subtitle, stats panel and start button
                header_card = <CardBase> {
                    width: Fill, height: Fit
                    padding: 20
                    flow: Down
                    spacing: 16

                    title_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        title_left = <View> {
                            width: Fit, height: Fit
                            flow: Right
                            align: {y: 1.0}
                            spacing: 12
                            <Label> {
                                text: "📚 温故知新"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_BOLD>{ font_size: 18.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                            <Label> {
                                text: "科学复习，牢记所学"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }
                        <View> { width: Fill }
                        start_review_btn = <PrimaryButton> { text: "🔄 开始复习" }
                    }
                    stats_panel = <StatsPanel> {}
                }

                // Tab area card
                tab_card = <CardBase> {
                    width: Fill, height: Fit
                    flow: Down

                    tab_header = <View> {
                        width: Fill, height: Fit
                        padding: {left: 20, right: 20, top: 12, bottom: 12}
                        flow: Right
                        align: {y: 0.5}
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                                sdf.fill(bg);
                                sdf.rect(0., self.rect_size.y - 1.0, self.rect_size.x, 1.0);
                                sdf.fill(border);
                                return sdf.result;
                            }
                        }
                        tabs = <View> {
                            width: Fit, height: Fit
                            flow: Right
                            align: {y: 0.5}
                            due_tab = <ReviewTabButton> {
                                text: "⏰ 待复习"
                                draw_bg: { selected: 1.0 }
                                draw_text: { selected: 1.0 }
                            }
                            mistakes_tab = <ReviewTabButton> { text: "❌ 易错点" }
                            mastered_tab = <ReviewTabButton> { text: "✅ 已掌握" }
                            stats_tab = <ReviewTabButton> { text: "📊 统计" }
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
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.due_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Due;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.mistakes_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Mistakes;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.mastered_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Mastered;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.stats_tab))
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
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.due_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_due { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_due { 1.0 } else { 0.0 }) }
                },
            );
        self.view
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.mistakes_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_mistakes { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_mistakes { 1.0 } else { 0.0 }) }
                },
            );
        self.view
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.mastered_tab))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (if is_mastered { 1.0 } else { 0.0 }) }
                    draw_text: { selected: (if is_mastered { 1.0 } else { 0.0 }) }
                },
            );
        self.view
            .button(ids!(content_scroll.content.tab_card.tab_header.tabs.stats_tab))
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
            .page_flip(ids!(content_scroll.content.tab_card.pages))
            .set_active_page(cx, page);

        self.view.redraw(cx);
    }
}
