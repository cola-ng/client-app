//! Dictionary Screen - Word lookup and vocabulary explorer
//!
//! Features:
//! - Search bar with instant results
//! - Word of the day card
//! - Search history
//! - Word details panel with phonetics, definitions, examples
//! - Category browsing

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_component::*;

    use colang_widgets::theme::*;

    // ========================================================================
    // Design Tokens
    // ========================================================================

    // Card base with subtle shadow effect
    DictCardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 16.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }
    }

    // Panel for secondary content
    DictPanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }
    }

    // Section title
    DictSectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Body text
    DictBodyText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    // Muted text
    DictMutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
            }
        }
    }

    // ========================================================================
    // Search Bar Component
    // ========================================================================

    SearchBar = <DictCardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Right
        spacing: 12
        align: {y: 0.5}

        draw_bg: {
            border_radius: 24.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        // Search icon
        search_icon = <Label> {
            text: ""
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 18.0 }
                color: (SLATE_400)
            }
        }

        // Text input
        search_input = <TextInput> {
            width: Fill, height: Fit
            empty_message: "Search for a word..."

            draw_bg: {
                color: #0000
            }

            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 15.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }

            draw_cursor: {
                color: (ACCENT_INDIGO)
            }

            draw_selection: {
                color: (INDIGO_100)
            }
        }

        // Clear button (hidden by default)
        clear_btn = <Button> {
            width: 28, height: 28
            text: ""
            visible: false

            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let color = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 14.0);
                    sdf.fill(color);
                    return sdf.result;
                }
            }

            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                color: (SLATE_500)
            }
        }
    }

    // ========================================================================
    // Word of the Day Card
    // ========================================================================

    WordOfDayCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 12

        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 16.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Gradient background
                let light_start = vec4(0.925, 0.937, 1.0, 1.0); // Light indigo
                let light_end = vec4(0.976, 0.949, 1.0, 1.0);   // Light purple
                let dark_start = (SLATE_800);
                let dark_end = (SLATE_700);
                let start = mix(light_start, dark_start, self.dark_mode);
                let end = mix(light_end, dark_end, self.dark_mode);
                let color = mix(start, end, self.pos.x);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        // Header
        wod_header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            <Label> {
                text: "Word of the Day"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((ACCENT_INDIGO), (INDIGO_300), self.dark_mode);
                    }
                }
            }

            <View> { width: Fill }

            wod_refresh_btn = <Button> {
                width: Fit, height: Fit
                padding: {left: 8, right: 8, top: 4, bottom: 4}
                text: "Refresh"

                draw_bg: {
                    fn pixel(self) -> vec4 { return #0000; }
                }

                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                    color: (SLATE_500)
                }
            }
        }

        // Word display
        wod_word = <Label> {
            text: "serendipity"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 28.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        // Phonetic
        wod_phonetic = <View> {
            width: Fit, height: Fit
            flow: Right
            spacing: 8
            align: {y: 0.5}

            phonetic_text = <Label> {
                text: "/ˌserənˈdipəti/"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    color: (SLATE_500)
                }
            }

            play_audio_btn = <Button> {
                width: 28, height: 28
                text: ""

                draw_bg: {
                    instance hover: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let color = mix((INDIGO_100), (INDIGO_200), self.hover);
                        sdf.circle(14.0, 14.0, 14.0);
                        sdf.fill(color);
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    color: (ACCENT_INDIGO)
                }
            }
        }

        // Part of speech
        wod_pos = <DictPanelBase> {
            width: Fit, height: Fit
            padding: {left: 10, right: 10, top: 4, bottom: 4}

            draw_bg: {
                border_radius: 6.0
                fn get_color(self) -> vec4 {
                    return (INDIGO_100);
                }
            }

            <Label> {
                text: "noun"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                    color: (ACCENT_INDIGO)
                }
            }
        }

        // Definition
        wod_definition = <DictBodyText> {
            width: Fill
            text: "The occurrence of events by chance in a happy or beneficial way."
        }

        // Chinese translation
        wod_chinese = <Label> {
            text: "意外发现美好事物的能力；机缘巧合"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_600), (SLATE_400), self.dark_mode);
                }
            }
        }

        // Example
        wod_example = <DictPanelBase> {
            width: Fill, height: Fit
            padding: 12
            flow: Down
            spacing: 4

            <DictMutedText> { text: "Example" }

            example_en = <DictBodyText> {
                width: Fill
                text: "\"A fortunate stroke of serendipity brought them together.\""
            }

            example_zh = <Label> {
                text: "一次幸运的巧合让他们相遇了。"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    color: (SLATE_500)
                }
            }
        }
    }

    // ========================================================================
    // Search Result Item
    // ========================================================================

    SearchResultItem = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        flow: Down
        spacing: 4
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light = mix((WHITE), (SLATE_50), self.hover);
                let dark = mix((SLATE_800), (SLATE_700), self.hover);
                let color = mix(light, dark, self.dark_mode);
                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(color);
                return sdf.result;
            }
        }

        // Word row
        word_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8
            align: {y: 0.5}

            result_word = <Label> {
                text: "hello"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            result_phonetic = <Label> {
                text: "/həˈloʊ/"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    color: (SLATE_400)
                }
            }

            <View> { width: Fill }

            result_pos = <Label> {
                text: "excl."
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                    color: (ACCENT_INDIGO)
                }
            }
        }

        // Definition
        result_definition = <Label> {
            width: Fill
            text: "你好；喂"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ========================================================================
    // Search Results Panel
    // ========================================================================

    SearchResultsPanel = <DictCardBase> {
        width: Fill, height: Fill
        flow: Down

        // Results header
        results_header = <View> {
            width: Fill, height: Fit
            padding: 16
            flow: Right
            align: {y: 0.5}

            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let color = mix((SLATE_50), (SLATE_700), self.dark_mode);
                    sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                    sdf.fill(color);
                    return sdf.result;
                }
            }

            <DictSectionTitle> { text: "Search Results" }
            <View> { width: Fill }
            results_count = <DictMutedText> { text: "0 results" }
        }

        // Divider
        <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: { color: (DIVIDER) }
        }

        // Results list
        results_scroll = <ScrollYView> {
            width: Fill, height: Fill

            results_list = <View> {
                width: Fill, height: Fit
                flow: Down

                // Placeholder items (will be generated dynamically)
                result_item_1 = <SearchResultItem> {
                    word_row = {
                        result_word = { text: "hello" }
                        result_phonetic = { text: "/həˈloʊ/" }
                        result_pos = { text: "excl." }
                    }
                    result_definition = { text: "你好；喂" }
                }

                result_item_2 = <SearchResultItem> {
                    word_row = {
                        result_word = { text: "help" }
                        result_phonetic = { text: "/help/" }
                        result_pos = { text: "v." }
                    }
                    result_definition = { text: "帮助；援助；有助于" }
                }

                result_item_3 = <SearchResultItem> {
                    word_row = {
                        result_word = { text: "helicopter" }
                        result_phonetic = { text: "/ˈhelɪkɑːptər/" }
                        result_pos = { text: "n." }
                    }
                    result_definition = { text: "直升机" }
                }
            }
        }

        // Empty state
        empty_state = <View> {
            width: Fill, height: Fill
            visible: false
            align: {x: 0.5, y: 0.5}

            content = <View> {
                width: Fit, height: Fit
                flow: Down
                spacing: 12
                align: {x: 0.5}

                <Label> {
                    text: ""
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 48.0 }
                        color: (SLATE_300)
                    }
                }

                <Label> {
                    text: "Start typing to search"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        color: (SLATE_400)
                    }
                }
            }
        }
    }

    // ========================================================================
    // Category Tag
    // ========================================================================

    CategoryTag = <View> {
        width: Fit, height: Fit
        padding: {left: 14, right: 14, top: 8, bottom: 8}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance selected: 0.0
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light_normal = (SLATE_100);
                let light_selected = (INDIGO_100);
                let dark_normal = (SLATE_700);
                let dark_selected = (INDIGO_900);
                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let selected = mix(light_selected, dark_selected, self.dark_mode);
                let color = mix(normal, selected, self.selected);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        tag_label = <Label> {
            text: "Daily"
            draw_text: {
                instance dark_mode: 0.0
                instance selected: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    let light_normal = (SLATE_600);
                    let light_selected = (ACCENT_INDIGO);
                    let dark_normal = (SLATE_400);
                    let dark_selected = (INDIGO_300);
                    let normal = mix(light_normal, dark_normal, self.dark_mode);
                    let selected = mix(light_selected, dark_selected, self.dark_mode);
                    return mix(normal, selected, self.selected);
                }
            }
        }
    }

    // ========================================================================
    // Categories Section
    // ========================================================================

    CategoriesCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        <DictSectionTitle> { text: "Categories" }

        categories_wrap = <View> {
            width: Fill, height: Fit
            flow: RightWrap
            spacing: 8

            cat_all = <CategoryTag> {
                tag_label = { text: "All" }
                draw_bg: { selected: 1.0 }
            }

            cat_daily = <CategoryTag> {
                tag_label = { text: "Daily Life" }
            }

            cat_business = <CategoryTag> {
                tag_label = { text: "Business" }
            }

            cat_academic = <CategoryTag> {
                tag_label = { text: "Academic" }
            }

            cat_tech = <CategoryTag> {
                tag_label = { text: "Technology" }
            }

            cat_travel = <CategoryTag> {
                tag_label = { text: "Travel" }
            }
        }
    }

    // ========================================================================
    // Recent Searches
    // ========================================================================

    RecentSearchItem = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        spacing: 10
        align: {y: 0.5}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(#0000, (SLATE_50), self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        <Label> {
            text: ""
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                color: (SLATE_400)
            }
        }

        recent_word = <Label> {
            text: "serendipity"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        <Label> {
            text: ""
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: (SLATE_300)
            }
        }
    }

    RecentSearchesCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 8

        // Header
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            <DictSectionTitle> { text: "Recent Searches" }
            <View> { width: Fill }
            clear_history_btn = <Button> {
                width: Fit, height: Fit
                text: "Clear"

                draw_bg: {
                    fn pixel(self) -> vec4 { return #0000; }
                }

                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                    color: (SLATE_500)
                }
            }
        }

        // Recent items
        recent_1 = <RecentSearchItem> {
            recent_word = { text: "serendipity" }
        }

        recent_2 = <RecentSearchItem> {
            recent_word = { text: "ephemeral" }
        }

        recent_3 = <RecentSearchItem> {
            recent_word = { text: "ubiquitous" }
        }
    }

    // ========================================================================
    // Main Dictionary Screen
    // ========================================================================

    pub DictionaryScreen = {{DictionaryScreen}} {
        width: Fill, height: Fill

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Main content scroll
        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            main_content = <View> {
                width: Fill, height: Fit
                flow: Right
                padding: 20
                spacing: 20

                // Left column - Search and Results
                left_column = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 16

                    search_bar = <SearchBar> {}
                    search_results = <SearchResultsPanel> {
                        height: 500
                    }
                }

                // Right column - Word of Day, Categories, Recent
                right_column = <View> {
                    width: 380, height: Fit
                    flow: Down
                    spacing: 16

                    word_of_day = <WordOfDayCard> {}
                    categories = <CategoriesCard> {}
                    recent_searches = <RecentSearchesCard> {}
                }
            }
        }
    }
}

/// DictionaryScreen widget
#[derive(Live, LiveHook, Widget)]
pub struct DictionaryScreen {
    #[deref]
    view: View,

    #[rust]
    search_query: String,
}

impl Widget for DictionaryScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DictionaryScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Handle search input changes
        if let Some(text) = self
            .text_input(ids!(search_bar.search_input))
            .changed(actions)
        {
            self.search_query = text;
            // TODO: Trigger search API call
            println!("Search query: {}", self.search_query);
        }

        // Handle word of day refresh
        if self
            .button(ids!(word_of_day.wod_header.wod_refresh_btn))
            .clicked(actions)
        {
            // TODO: Fetch new random word
            println!("Refresh word of day");
        }

        // Handle audio play
        if self
            .button(ids!(word_of_day.wod_phonetic.play_audio_btn))
            .clicked(actions)
        {
            // TODO: Play pronunciation audio
            println!("Play audio");
        }

        // Handle clear history
        if self
            .button(ids!(recent_searches.clear_history_btn))
            .clicked(actions)
        {
            // TODO: Clear search history
            println!("Clear history");
        }
    }
}

impl DictionaryScreen {
    /// Apply dark mode to all components
    pub fn apply_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }
}
