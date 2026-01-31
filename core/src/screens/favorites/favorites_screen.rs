//! # Favorites Screen
//!
//! Display and manage user's favorite words and sentences.
//! Matches website FavoritesPage styling.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Colors matching website
    ORANGE_50 = #fff7ed
    ORANGE_100 = #ffedd5
    ORANGE_800 = #9a3412
    AMBER_50 = #fffbeb
    YELLOW_50 = #fefce8

    // Tab button - matches website TabsTrigger style
    FavTab = <Button> {
        width: Fit, height: Fit
        padding: {left: 14, right: 14, top: 8, bottom: 8}
        margin: {right: 4}

        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                if self.active > 0.5 {
                    // Active: orange-100 bg
                    let light = vec4(1.0, 0.929, 0.835, 1.0); // orange-100
                    let dark = (SLATE_700);
                    sdf.fill(mix(light, dark, self.dark_mode));
                } else if self.hover > 0.5 {
                    // Hover: gray-100
                    let light = vec4(0.953, 0.961, 0.969, 1.0);
                    let dark = (SLATE_700);
                    sdf.fill(mix(light, dark, self.dark_mode));
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance active: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                // Active: orange-800, Inactive: gray-600
                let light_inactive = vec4(0.298, 0.333, 0.388, 1.0); // gray-600
                let light_active = vec4(0.604, 0.176, 0.071, 1.0);   // orange-800
                let dark_inactive = (SLATE_400);
                let dark_active = vec4(1.0, 0.929, 0.835, 1.0); // orange-100

                let inactive = mix(light_inactive, dark_inactive, self.dark_mode);
                let active_color = mix(light_active, dark_active, self.dark_mode);
                return mix(inactive, active_color, self.active);
            }
        }
    }

    // Favorite item card - matches website card styling with shadow
    FavItemCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 14
        flow: Right
        align: {y: 0.5}
        spacing: 12
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            border_radius: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Subtle shadow on hover
                if self.hover > 0.5 {
                    let shadow = vec4(0.0, 0.0, 0.0, 0.06);
                    sdf.box(1.0, 2.0, self.rect_size.x - 2.0, self.rect_size.y - 1.0, 12.0);
                    sdf.fill(shadow);
                }

                // Card bg
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0);
                let light_hover = vec4(0.976, 0.980, 0.984, 1.0); // gray-50
                let dark_bg = (SLATE_800);
                let dark_hover = (SLATE_700);

                let light = mix(light_bg, light_hover, self.hover);
                let dark = mix(dark_bg, dark_hover, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(mix(light, dark, self.dark_mode));

                return sdf.result;
            }
        }

        // Play button for pronunciation
        play_btn = <View> {
            width: 32, height: 32
            show_bg: true
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let normal = vec4(0.976, 0.451, 0.086, 0.1);  // orange-500 with low opacity
                    let hover_bg = vec4(0.976, 0.451, 0.086, 0.2);
                    sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 14.0);
                    sdf.fill(mix(normal, hover_bg, self.hover));
                    return sdf.result;
                }
            }
            align: {x: 0.5, y: 0.5}
            cursor: Hand

            <Label> {
                text: "🔊"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                }
            }
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            word = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                    fn get_color(self) -> vec4 {
                        let light = vec4(0.110, 0.118, 0.149, 1.0); // gray-900
                        let dark = (TEXT_PRIMARY_DARK);
                        return mix(light, dark, self.dark_mode);
                    }
                }
            }

            meaning = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        let light = vec4(0.420, 0.447, 0.502, 1.0); // gray-500
                        let dark = (TEXT_SECONDARY_DARK);
                        return mix(light, dark, self.dark_mode);
                    }
                }
            }
        }

        delete_btn = <Button> {
            width: 28, height: 28
            text: "×"
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let normal = vec4(0.0, 0.0, 0.0, 0.0);
                    let hover_bg = vec4(0.996, 0.949, 0.949, 1.0); // red-50
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                    sdf.fill(mix(normal, hover_bg, self.hover));
                    return sdf.result;
                }
            }
            draw_text: {
                instance hover: 0.0
                text_style: <FONT_REGULAR>{ font_size: 16.0 }
                fn get_color(self) -> vec4 {
                    let normal = vec4(0.596, 0.631, 0.678, 1.0); // gray-400
                    let hover_color = vec4(0.863, 0.196, 0.196, 1.0); // red-600
                    return mix(normal, hover_color, self.hover);
                }
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
            text: "❤️"
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
                // Gradient matching website: from-orange-50 via-amber-50 to-yellow-50
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0);
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);

                let t = self.pos.x + self.pos.y * 0.3;
                let light_color = vec4(0.0);
                if t < 0.5 {
                    light_color = mix(orange_50, amber_50, t * 2.0);
                } else {
                    light_color = mix(amber_50, yellow_50, (t - 0.5) * 2.0);
                }
                return mix(light_color, dark_bg, self.dark_mode);
            }
        }

        // Header
        header = <View> {
            width: Fill, height: Fit
            padding: {left: 16, right: 16, top: 16, bottom: 12}
            flow: Down
            spacing: 12

            title_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}
                spacing: 12

                title = <Label> {
                    text: "❤️ 收藏夹"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 20.0 }
                        fn get_color(self) -> vec4 {
                            let light = vec4(0.110, 0.118, 0.149, 1.0); // gray-900
                            let dark = (TEXT_PRIMARY_DARK);
                            return mix(light, dark, self.dark_mode);
                        }
                    }
                }

                <View> { width: Fill }

                // Search input with rounded style
                search_input = <RoundedView> {
                    width: 180, height: 36
                    padding: {left: 10, right: 10}
                    flow: Right
                    spacing: 6
                    align: {y: 0.5}
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 8.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let light = vec4(1.0, 1.0, 1.0, 1.0);
                            let dark = (SLATE_700);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                            sdf.fill(mix(light, dark, self.dark_mode));
                            // Border
                            let border = mix(vec4(0.835, 0.847, 0.863, 1.0), (SLATE_600), self.dark_mode);
                            sdf.stroke(border, 1.0);
                            return sdf.result;
                        }
                    }

                    <Label> {
                        text: "🔍"
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        }
                    }

                    search_text_input = <TextInput> {
                        width: Fill, height: Fit
                        empty_text: "搜索收藏..."
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                return vec4(0.0, 0.0, 0.0, 0.0);
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
            }

            // Tabs with muted background container
            tabs_container = <RoundedView> {
                width: Fit, height: Fit
                padding: 4
                flow: Right
                spacing: 0
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    border_radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let light = vec4(0.961, 0.953, 0.945, 1.0); // muted
                        let dark = (SLATE_800);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill(mix(light, dark, self.dark_mode));
                        return sdf.result;
                    }
                }

                words_tab = <FavTab> {
                    text: "📚 单词"
                    draw_bg: { active: 1.0 }
                    draw_text: { active: 1.0 }
                }

                sentences_tab = <FavTab> {
                    text: "📝 句子"
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
        if self.view.button(ids!(header.tabs_container.words_tab)).clicked(actions) {
            self.switch_tab(cx, FavTab::Words);
        }
        if self.view.button(ids!(header.tabs_container.sentences_tab)).clicked(actions) {
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
        self.view.button(ids!(header.tabs_container.words_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (words_active) }
                draw_text: { active: (words_active) }
            },
        );
        self.view.button(ids!(header.tabs_container.sentences_tab)).apply_over(
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
