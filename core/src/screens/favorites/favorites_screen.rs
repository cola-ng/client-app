//! # Favorites Screen
//!
//! Display and manage user's favorite words and sentences.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Tab button
    FavTab = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 10, bottom: 10}

        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                if self.active > 0.5 {
                    let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(bg);
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance active: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                let inactive = mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                let active_color = mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                return mix(inactive, active_color, self.active);
            }
        }
    }

    // Favorite item card
    FavItemCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Right
        align: {y: 0.5}
        spacing: 12
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            border_radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let light_bg = mix((WHITE), (SLATE_50), self.hover);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.hover);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));
                return sdf.result;
            }
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            word = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            meaning = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        delete_btn = <Button> {
            width: 32, height: 32
            text: "×"
            draw_bg: {
                fn pixel(self) -> vec4 { return (TRANSPARENT); }
            }
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 18.0 }
                color: (SLATE_400)
            }
        }
    }

    // Empty state
    EmptyState = <View> {
        width: Fill, height: 300
        flow: Down
        align: {x: 0.5, y: 0.5}
        spacing: 12

        icon = <Label> {
            text: "⭐"
            draw_text: {
                text_style: { font_size: 48.0 }
            }
        }

        title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        subtitle = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    pub FavoritesScreen = {{FavoritesScreen}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Header
        header = <View> {
            width: Fill, height: Fit
            padding: {left: 24, right: 24, top: 20, bottom: 16}
            flow: Down
            spacing: 16

            title_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}

                title = <Label> {
                    text: "我的收藏"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 24.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                <View> { width: Fill }

                // Search input
                search_input = <TextInput> {
                    width: 200, height: Fit
                    padding: {left: 12, right: 12, top: 8, bottom: 8}
                    empty_message: "搜索收藏..."
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                            let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                            sdf.fill(bg);
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }
            }

            // Tabs
            tabs = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 8

                words_tab = <FavTab> {
                    text: "单词"
                    draw_bg: { active: 1.0 }
                    draw_text: { active: 1.0 }
                }

                sentences_tab = <FavTab> {
                    text: "句子"
                }
            }
        }

        // Content
        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: {left: 24, right: 24, bottom: 24}
                spacing: 12

                // Words list
                words_list = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    // Sample items
                    item_1 = <FavItemCard> {
                        content = {
                            word = { text: "abandon" }
                            meaning = { text: "放弃，抛弃" }
                        }
                    }

                    item_2 = <FavItemCard> {
                        content = {
                            word = { text: "benefit" }
                            meaning = { text: "好处，利益" }
                        }
                    }

                    item_3 = <FavItemCard> {
                        content = {
                            word = { text: "challenge" }
                            meaning = { text: "挑战" }
                        }
                    }
                }

                // Sentences list (hidden by default)
                sentences_list = <View> {
                    width: Fill, height: Fit
                    visible: false
                    flow: Down
                    spacing: 8

                    sent_1 = <FavItemCard> {
                        content = {
                            word = { text: "Never abandon your dreams." }
                            meaning = { text: "永远不要放弃你的梦想。" }
                        }
                    }

                    sent_2 = <FavItemCard> {
                        content = {
                            word = { text: "Education has many benefits." }
                            meaning = { text: "教育有很多好处。" }
                        }
                    }
                }

                // Empty state (hidden by default)
                empty_state = <EmptyState> {
                    visible: false
                    title = { text: "暂无收藏" }
                    subtitle = { text: "在学习过程中收藏单词或句子" }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum FavoritesScreenAction {
    None,
    SearchChanged(String),
    DeleteWord(String),
    DeleteSentence(String),
    ItemClicked(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FavTab {
    #[default]
    Words,
    Sentences,
}

#[derive(Live, LiveHook, Widget)]
pub struct FavoritesScreen {
    #[deref]
    view: View,
    #[rust]
    current_tab: FavTab,
}

impl Widget for FavoritesScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle tab switching
        if self.view.button(ids!(header.tabs.words_tab)).clicked(actions) {
            self.switch_tab(cx, FavTab::Words);
        }
        if self.view.button(ids!(header.tabs.sentences_tab)).clicked(actions) {
            self.switch_tab(cx, FavTab::Sentences);
        }

        // Handle search input
        // Note: In a real app, would filter the list based on search text
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl FavoritesScreen {
    fn switch_tab(&mut self, cx: &mut Cx, tab: FavTab) {
        self.current_tab = tab;

        let (words_active, sentences_active) = match tab {
            FavTab::Words => (1.0, 0.0),
            FavTab::Sentences => (0.0, 1.0),
        };

        // Update tab styles
        self.view.button(ids!(header.tabs.words_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (words_active) }
                draw_text: { active: (words_active) }
            },
        );
        self.view.button(ids!(header.tabs.sentences_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (sentences_active) }
                draw_text: { active: (sentences_active) }
            },
        );

        // Toggle list visibility
        self.view.view(ids!(content_scroll.content.words_list))
            .set_visible(cx, tab == FavTab::Words);
        self.view.view(ids!(content_scroll.content.sentences_list))
            .set_visible(cx, tab == FavTab::Sentences);

        self.view.redraw(cx);
    }
}
