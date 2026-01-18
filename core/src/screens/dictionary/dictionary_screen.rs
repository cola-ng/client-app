//! Dictionary Screen - Word lookup and vocabulary explorer
//!
//! Features:
//! - Search bar with instant results
//! - Navigation sidebar for quick access to word details
//! - Search history with favorite marking
//! - Word details panel with phonetics, definitions, examples

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

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
            text: "Your text here"

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

        // Search button
        search_btn = <Button> {
            width: Fit, height: 32
            text: "Search"
            padding: {left: 16, right: 16}

            draw_bg: {
                instance dark_mode: 0.0
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let base_color = (ACCENT_INDIGO);
                    let color = mix(base_color, (INDIGO_600), self.hover);
                    sdf.rounded_box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                    sdf.fill(color);
                    return sdf.result;
                }
            }

            draw_text: {
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                color: (WHITE)
            }
        }
    }

    // ========================================================================
    // Search History Card
    // ========================================================================

    SearchHistoryItem = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 8, bottom: 8}
        flow: Right
        spacing: 6
        align: {y: 0.5}

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

        history_word = <Label> {
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

        favorite_star = <Label> {
            text: ""
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: (SLATE_300)
            }
        }
    }

    SearchHistoryCard = <DictCardBase> {
        width: Fill, height: Fill
        padding: 16
        flow: Down
        spacing: 8

        // Header
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            <DictSectionTitle> { text: "Search History" }
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

        // History items
        history_scroll = <ScrollYView> {
            width: Fill, height: Fill

            history_list = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 4

                history_1 = <SearchHistoryItem> {
                    history_word = { text: "serendipity" }
                    favorite_star = { text: "" }
                }

                history_2 = <SearchHistoryItem> {
                    history_word = { text: "ephemeral" }
                    favorite_star = { text: "" }
                }

                history_3 = <SearchHistoryItem> {
                    history_word = { text: "ubiquitous" }
                    favorite_star = { text: "" }
                }

                history_4 = <SearchHistoryItem> {
                    history_word = { text: "eloquent" }
                    favorite_star = { text: "" }
                }

                history_5 = <SearchHistoryItem> {
                    history_word = { text: "pragmatic" }
                    favorite_star = { text: "" }
                }

                history_6 = <SearchHistoryItem> {
                    history_word = { text: "resilience" }
                    favorite_star = { text: "" }
                }
            }
        }
    }

    // ========================================================================
    // Navigation Anchor Item
    // ========================================================================

    NavAnchorItem = <View> {
        width: Fill, height: Fit
        padding: {left: 8, right: 8, top: 6, bottom: 6}
        flow: Right
        align: {y: 0.5}

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                let active_bg = mix((INDIGO_50), (SLATE_600), self.dark_mode);
                let hover_bg = mix(bg, active_bg, self.active);
                let final_bg = mix(hover_bg, active_bg, self.hover * 0.5);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(final_bg);
                return sdf.result;
            }
        }

        anchor_text = <Label> {
            text: "词典释义"
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
    // Navigation Sidebar
    // ========================================================================

    NavSidebar = <View> {
        width: 200, height: Fill
        padding: {top: 20, bottom: 20}
        flow: Down
        spacing: 4

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(color);
                return sdf.result;
            }
        }

        <DictSectionTitle> {
            text: "导航"
            padding: {left: 8, right: 8, top: 4, bottom: 8}
        }

        nav_anchors = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 2

            anchor_definitions = <NavAnchorItem> {
                anchor_text = { text: "词典释义" }
            }

            anchor_examples = <NavAnchorItem> {
                anchor_text = { text: "例句" }
            }

            anchor_usage = <NavAnchorItem> {
                anchor_text = { text: "用法" }
            }

            anchor_encyclopedia = <NavAnchorItem> {
                anchor_text = { text: "百科" }
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

                // Dynamic result item (hidden until search results arrive)
                result_item_1 = <SearchResultItem> {
                    visible: false
                }
            }
        }

        // Empty state
        empty_state = <View> {
            width: Fill, height: Fill
            visible: true
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

        // Main content area
        main_content = <View> {
            width: Fill, height: Fill
            flow: Right

            // Left column - Navigation anchors
            nav_sidebar = <NavSidebar> {}

            // Center column - Search and Results
            center_column = <View> {
                width: Fill, height: Fill
                padding: {top: 20, bottom: 20, left: 20, right: 20}
                flow: Down
                spacing: 16

                search_bar = <SearchBar> {}
                search_results = <SearchResultsPanel> {
                    height: Fill
                }
            }

            // Right column - Search History
            right_column = <View> {
                width: 200, height: Fill
                padding: {top: 20, bottom: 20, left: 16, right: 16}
                flow: Down

                search_history = <SearchHistoryCard> {}
            }
        }
    }
}

use crate::dict_api::{get_dict_api, Word, WordQueryResponse, SearchHistoryEntry};
use crossbeam_channel::{Receiver, Sender, bounded};

/// DictionaryScreen widget
#[derive(Live, LiveHook, Widget)]
pub struct DictionaryScreen {
    #[deref]
    view: View,

    #[rust]
    search_query: String,

    #[rust]
    search_results: Vec<Word>,

    #[rust]
    selected_word: Option<WordQueryResponse>,

    #[rust]
    is_searching: bool,

    #[rust]
    lookup_result_receiver: Option<Receiver<Result<WordQueryResponse, String>>>,

    #[rust]
    lookup_result_sender: Option<Sender<Result<WordQueryResponse, String>>>,
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
            self.search_query = text.clone();
            ::log::info!("Search query: {}", self.search_query);
        }

        // Handle search button click
        if self.button(ids!(search_bar.search_btn)).clicked(actions) {
            if !self.search_query.trim().is_empty() {
                self.perform_lookup(cx, self.search_query.clone());
            }
        }

        // Handle clear history
        if self
            .button(ids!(search_history.clear_history_btn))
            .clicked(actions)
        {
            self.clear_search_history(cx);
        }

        // Check for lookup results from channel
        if let Some(ref receiver) = self.lookup_result_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.is_searching = false;
                match result {
                    Ok(word_response) => {
                        ::log::info!("Lookup success for: {}", word_response.word.word);
                        self.selected_word = Some(word_response.clone());
                        self.update_result_display(cx, &word_response);
                    }
                    Err(e) => {
                        ::log::error!("Lookup error: {}", e);
                    }
                }
            }
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

    /// Perform a dictionary lookup (search for exact word and display details)
    fn perform_lookup(&mut self, _cx: &mut Cx, query: String) {
        if self.is_searching {
            return;
        }
        self.is_searching = true;

        // Create a channel for communication
        let (sender, receiver) = bounded(1);
        self.lookup_result_sender = Some(sender.clone());
        self.lookup_result_receiver = Some(receiver);

        // Get the API client
        let api = match get_dict_api() {
            Some(api) => api.read().unwrap().clone(),
            None => {
                ::log::error!("Dictionary API not initialized");
                self.is_searching = false;
                return;
            }
        };

        // Spawn async lookup task
        let query_clone = query.clone();
        let sender_clone = sender.clone();
        let api_clone = api.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match api_clone.lookup(&query_clone).await {
                    Ok(result) => {
                        // Save to search history in background
                        let _ = api_clone.save_search_history(&query_clone).await;
                        // Send result back to main thread via channel
                        let _ = sender_clone.send(Ok(result));
                    }
                    Err(e) => {
                        // Send error back to main thread
                        let _ = sender_clone.send(Err(e));
                    }
                }
            });
        });
    }

    /// Update the UI with lookup result
    fn update_result_display(&mut self, cx: &mut Cx, result: &WordQueryResponse) {
        let word = &result.word;

        // Get phonetic info from forms (if available)
        let phonetic_text = result.forms.first().and_then(|form| {
            if let Some(uk) = &form.phonetic_uk {
                Some(format!("英 /{}/", uk))
            } else if let Some(us) = &form.phonetic_us {
                Some(format!("美 /{}/", us))
            } else {
                None
            }
        }).unwrap_or_default();

        // Get part of speech and definition from first definition
        let primary_definition = result.definitions.first();
        let pos_text = primary_definition
            .and_then(|d| d.part_of_speech.clone())
            .unwrap_or_else(|| word.word_type.clone().unwrap_or_default());

        let definition_text = primary_definition
            .map(|d| d.definition_zh.clone())
            .unwrap_or_default();

        // Update the UI with the fetched data
        self.view.apply_over(
            cx,
            live! {
                search_results = {
                    empty_state = { visible: false }
                    results_scroll = {
                        results_list = {
                            result_item_1 = {
                                visible: true,
                                word_row = {
                                    result_word = { text: (word.word.clone()) }
                                    result_phonetic = { text: (phonetic_text) }
                                    result_pos = { text: (pos_text) }
                                }
                                result_definition = { text: (definition_text) }
                            }
                        }
                    }
                }
            }
        );
    }

    /// Clear search history
    fn clear_search_history(&mut self, cx: &mut Cx) {
        let api = match get_dict_api() {
            Some(api) => api.read().unwrap().clone(),
            None => {
                ::log::error!("Dictionary API not initialized");
                return;
            }
        };

        let api_clone = api.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match api_clone.clear_search_history().await {
                    Ok(_) => {
                        ::log::info!("Search history cleared");
                    }
                    Err(e) => {
                        ::log::error!("Failed to clear search history: {}", e);
                    }
                }
            });
        });
    }
}
