//! Scene Center - Scene selection and management
//!
//! Layout based on design/kiro/03-场景中心-sketch.svg (optimized v2):
//! - Continue Learning section with progress tracking
//! - Smart AI recommendations section
//! - Today's featured scenes (dynamic from API)
//! - Classic dialogues section (dynamic from API)

use std::sync::mpsc;

use makepad_widgets::*;
use makepad_component::*;

use crate::asset_api::{ClassicDialogueSource, Scene, get_asset_api};

/// Scene category for filtering
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SceneCategory {
    #[default]
    All,
    Daily,
    Travel,
    Business,
}

impl SceneCategory {
    fn matches(&self, category_str: &str) -> bool {
        match self {
            SceneCategory::All => true,
            SceneCategory::Daily => category_str == "daily",
            SceneCategory::Travel => category_str == "travel",
            SceneCategory::Business => category_str == "business",
        }
    }
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Orange theme accent colors
    ACCENT_ORANGE = #f97316
    ACCENT_ORANGE_HOVER = #ea580c
    ACCENT_ORANGE_LIGHT = #fff7ed
    ORANGE_400 = #fb923c
    ORANGE_500 = #f97316

    // Difficulty badge colors
    GREEN_50 = #f0fdf4
    GREEN_600 = #16a34a
    AMBER_50 = #fffbeb
    AMBER_600 = #d97706
    RED_50 = #fef2f2
    RED_600 = #dc2626

    // ========================================================================
    // Design Tokens
    // ========================================================================

    CardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 16.0
            fn pixel(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }
    }

    PanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }
    }

    SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    BodyText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    MutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
            }
        }
    }

    // Category filter button base
    CategoryButton = <RoundedView> {
        width: Fit, height: 32
        padding: {left: 16, right: 16}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance dark_mode: 0.0
            border_radius: 16.0
            fn pixel(self) -> vec4 {
                let orange = vec4(0.976, 0.451, 0.086, 1.0); // #f97316
                let white = vec4(1.0, 1.0, 1.0, 1.0);
                return mix(white, orange, self.selected);
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand
    }

    // Template for scenario card in today's section (improved)
    ScenesCardTemplate = <CardBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 8
        cursor: Hand

        // Icon at top
        icon = <Label> {
            text: "🍽️"
            draw_text: {
                text_style: <FONT_BOLD>{ font_size: 36.0 }
            }
        }

        // Chinese title
        title = <Label> {
            text: "场景名称"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        // English subtitle
        title_en = <Label> {
            text: "Scene Name"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                }
            }
        }

        // Bottom row with difficulty badge and duration
        info_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8
            align: {y: 0.5}

            // Difficulty badge
            difficulty_badge = <RoundedView> {
                width: Fit, height: 20
                padding: {left: 8, right: 8}
                show_bg: true
                draw_bg: {
                    instance badge_color: 0.0  // 0=green, 1=amber, 2=red
                    border_radius: 10.0
                    fn pixel(self) -> vec4 {
                        let green = vec4(0.941, 0.992, 0.957, 1.0);  // #f0fdf4
                        let amber = vec4(1.0, 0.984, 0.922, 1.0);    // #fffbeb
                        let red = vec4(0.996, 0.949, 0.949, 1.0);    // #fef2f2
                        if self.badge_color < 0.5 { return green; }
                        else if self.badge_color < 1.5 { return amber; }
                        else { return red; }
                    }
                }
                align: {x: 0.5, y: 0.5}

                badge_label = <Label> {
                    text: "⭐"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 10.0 }
                    }
                }
            }

            // Duration with clock
            duration = <Label> {
                text: "🕐 5分钟"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                    }
                }
            }
        }
    }

    // Template for classic dialogue card
    ClassicCardTemplate = <CardBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Right
        spacing: 12
        cursor: Hand

        icon_area = <RoundedView> {
            width: 56, height: 56
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 8.0
                fn pixel(self) -> vec4 {
                    let light_color = vec4(0.1, 0.1, 0.18, 1.0);
                    let dark_color = vec4(0.2, 0.2, 0.3, 1.0);
                    return mix(light_color, dark_color, self.dark_mode);
                }
            }
            align: {x: 0.5, y: 0.5}

            icon = <Label> {
                text: "🎥"
                draw_text: {
                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                }
            }
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4
            align: {y: 0.5}

            title = <Label> {
                text: "《电影名》"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            description = <Label> {
                text: "场景描述"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        return mix((ACCENT_ORANGE), (ORANGE_400), self.dark_mode);
                    }
                }
            }
        }

        chevron = <Label> {
            text: "›"
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 18.0 }
                color: (SLATE_300)
            }
        }
    }

    // ========================================================================
    // Scene Center Main Widget
    // ========================================================================

    pub Scenes = {{Scenes}} <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Gradient: orange-50 (#fff7ed) → amber-50 (#fffbeb) → yellow-50 (#fefce8)
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0);
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);

                let t = self.pos.x;
                let light_color = vec4(0.0);
                if t < 0.5 {
                    light_color = mix(orange_50, amber_50, t * 2.0);
                } else {
                    light_color = mix(amber_50, yellow_50, (t - 0.5) * 2.0);
                }
                return mix(light_color, dark_bg, self.dark_mode);
            }
        }

        // Scrollable content
        scene_list = <ScrollYView> {
            width: Fill, height: Fill
            flow: Down
            spacing: 20
            padding: {left: 40, right: 40, top: 30, bottom: 30}

            // Header
            header = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8
                visible: false

                title = <Label> {
                    text: "🎭 场景中心"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 28.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                subtitle = <Label> {
                    text: "沉浸式场景模拟 · AI智能推荐 · 经典对白学习 · 多口音体验"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }
            }

            // Search and Filters
            search_bar = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12
                align: {y: 0.5}

                search_input = <RoundedView> {
                    width: 300, height: 40
                    padding: {left: 12, right: 12}
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}
                    show_bg: true
                    draw_bg: {
                        border_radius: 8.0
                        color: (WHITE)
                    }

                    search_icon = <Label> {
                        text: "🔍"
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        }
                    }

                    search_text_input = <TextInput> {
                        width: Fill, height: Fit
                        empty_text: "搜索场景..."
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 13.0 }
                            color: (TEXT_PRIMARY)
                        }
                    }
                }

                filter_chips = <View> {
                    width: Fit, height: 40
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}

                    filter_all = <CategoryButton> {
                        draw_bg: { selected: 1.0 }
                        label = <Label> {
                            text: "全部"
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                color: (WHITE)
                            }
                        }
                    }

                    filter_daily = <CategoryButton> {
                        draw_bg: { selected: 0.0 }
                        label = <Label> {
                            text: "日常生活"
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                color: (TEXT_SECONDARY)
                            }
                        }
                    }

                    filter_travel = <CategoryButton> {
                        draw_bg: { selected: 0.0 }
                        label = <Label> {
                            text: "旅行出行"
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                color: (TEXT_SECONDARY)
                            }
                        }
                    }

                    filter_business = <CategoryButton> {
                        draw_bg: { selected: 0.0 }
                        label = <Label> {
                            text: "商务职场"
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                color: (TEXT_SECONDARY)
                            }
                        }
                    }
                }
            }

            // Continue Learning Section
            continue_learning_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_title = <SectionTitle> {
                    text: "📚 继续学习"
                }

                continue_card = <CardBase> {
                    width: Fill, height: Fit
                    padding: 16
                    flow: Down
                    spacing: 12

                    // Inner content with gradient background
                    inner_content = <RoundedView> {
                        width: Fill, height: Fit
                        padding: 16
                        flow: Right
                        spacing: 16
                        align: {y: 0.5}
                        show_bg: true
                        draw_bg: {
                            border_radius: 12.0
                            fn pixel(self) -> vec4 {
                                // Gradient from orange-50 to amber-50
                                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                                return mix(orange_50, amber_50, self.pos.x);
                            }
                        }

                        icon = <Label> {
                            text: "🏨"
                            draw_text: {
                                text_style: <FONT_BOLD>{ font_size: 48.0 }
                            }
                        }

                        content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 6

                            title = <Label> {
                                text: "酒店入住"
                                draw_text: {
                                    text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                                    color: (TEXT_PRIMARY)
                                }
                            }

                            progress = <Label> {
                                text: "进度 60% · 还剩 3 个对话"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    color: (TEXT_MUTED)
                                }
                            }

                            task = <Label> {
                                text: "下一个任务：前台预订房间"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                    color: (TEXT_SECONDARY)
                                }
                            }
                        }

                        button = <Button> {
                            width: Fit, height: 40
                            padding: {left: 16, right: 16}
                            text: "继续学习 ›"
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                    sdf.fill(vec4(0.976, 0.451, 0.086, 1.0)); // #f97316
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                color: (WHITE)
                            }
                        }
                    }
                }
            }

            // Today's Scenes Section (Dynamic) - Grid Layout
            today_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_title = <SectionTitle> {
                    text: "🌟 今日精选"
                }

                // Loading state
                loading_label = <View> {
                    width: Fit, height: Fit
                    visible: false
                    label = <Label> {
                        text: "加载中..."
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                // Grid layout for scenes (using nested views for columns)
                today_cards = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    // Row 1
                    row1 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        card0 = <ScenesCardTemplate> {}
                        card1 = <ScenesCardTemplate> {}
                        card2 = <ScenesCardTemplate> {}
                        card3 = <ScenesCardTemplate> {}
                    }

                    // Row 2
                    row2 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        card4 = <ScenesCardTemplate> {}
                        card5 = <ScenesCardTemplate> {}
                        card6 = <ScenesCardTemplate> {}
                        card7 = <ScenesCardTemplate> {}
                    }
                }
            }

            // Classic Dialogues Section (Dynamic) - 2 Column Grid
            classic_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_header = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}

                    section_title = <SectionTitle> {
                        text: "🎬 经典对白"
                    }

                    section_subtitle = <Label> {
                        text: "从电影/美剧中学习地道表达"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                // Loading state
                classic_loading_label = <View> {
                    width: Fit, height: Fit
                    visible: false
                    label = <Label> {
                        text: "加载中..."
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                // 2-column grid for classic dialogues
                classic_cards = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    // Row 1
                    classic_row1 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        classic0 = <ClassicCardTemplate> {}
                        classic1 = <ClassicCardTemplate> {}
                    }

                    // Row 2
                    classic_row2 = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        classic2 = <ClassicCardTemplate> {}
                        classic3 = <ClassicCardTemplate> {}
                    }
                }
            }
        }
    }
}

/// Data fetch result types
enum FetchResult {
    Scenes(Result<Vec<Scene>, String>),
    ClassicSources(Result<Vec<ClassicDialogueSource>, String>),
}

#[derive(Live, LiveHook, Widget)]
pub struct Scenes {
    #[deref]
    view: View,

    #[rust]
    scenes: Vec<Scene>,

    #[rust]
    filtered_scenes: Vec<Scene>,

    #[rust]
    classic_sources: Vec<ClassicDialogueSource>,

    #[rust]
    scenes_loading: bool,

    #[rust]
    classic_loading: bool,

    #[rust]
    data_loaded: bool,

    #[rust]
    fetch_rx: Option<mpsc::Receiver<FetchResult>>,

    #[rust]
    search_query: String,

    #[rust]
    active_category: SceneCategory,
}

impl Widget for Scenes {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        // Check for text changes in search input
        let search_input = self.view.text_input(ids!(search_bar.search_input.search_text_input));
        let current_text = search_input.text();
        if current_text != self.search_query {
            self.search_query = current_text;
            self.filter_scenes();
            self.update_scene_cards(cx);
            self.view.redraw(cx);
        }

        // Handle category filter clicks
        for (widget_id, category) in [
            (ids!(search_bar.filter_chips.filter_all), SceneCategory::All),
            (ids!(search_bar.filter_chips.filter_daily), SceneCategory::Daily),
            (ids!(search_bar.filter_chips.filter_travel), SceneCategory::Travel),
            (ids!(search_bar.filter_chips.filter_business), SceneCategory::Business),
        ] {
            if self.view.view(widget_id).finger_up(&actions).is_some() {
                if self.active_category != category {
                    self.active_category = category;
                    self.update_category_buttons(cx);
                    self.filter_scenes();
                    self.update_scene_cards(cx);
                    self.view.redraw(cx);
                }
            }
        }

        // Collect fetch results first to avoid borrow issues
        let mut scenes_result: Option<Result<Vec<Scene>, String>> = None;
        let mut classic_result: Option<Result<Vec<ClassicDialogueSource>, String>> = None;

        if let Some(rx) = &self.fetch_rx {
            while let Ok(result) = rx.try_recv() {
                match result {
                    FetchResult::Scenes(r) => scenes_result = Some(r),
                    FetchResult::ClassicSources(r) => classic_result = Some(r),
                }
            }
        }

        // Process scenes result
        if let Some(result) = scenes_result {
            match result {
                Ok(scenes) => {
                    self.scenes = scenes;
                    self.scenes_loading = false;
                    self.filter_scenes();
                    self.view
                        .view(ids!(today_section.loading_label))
                        .set_visible(cx, false);
                    self.update_scene_cards(cx);
                    self.view.redraw(cx);
                }
                Err(e) => {
                    eprintln!("Failed to fetch scenes: {}", e);
                    self.scenes_loading = false;
                    self.view
                        .label(ids!(today_section.loading_label.label))
                        .set_text(cx, &format!("加载失败: {}", e));
                }
            }
        }

        // Process classic sources result
        if let Some(result) = classic_result {
            match result {
                Ok(sources) => {
                    self.classic_sources = sources;
                    self.classic_loading = false;
                    self.view
                        .view(ids!(classic_section.classic_loading_label))
                        .set_visible(cx, false);
                    self.update_classic_cards(cx);
                    self.view.redraw(cx);
                }
                Err(e) => {
                    eprintln!("Failed to fetch classic sources: {}", e);
                    self.classic_loading = false;
                    self.view
                        .label(ids!(classic_section.classic_loading_label.label))
                        .set_text(cx, &format!("加载失败: {}", e));
                }
            }
        }

        // Trigger initial data load on first draw
        if let Event::Draw(_) = event {
            if !self.data_loaded {
                self.data_loaded = true;
                self.load_data(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Scenes {
    /// Filter scenes based on search query and category
    fn filter_scenes(&mut self) {
        let query = self.search_query.to_lowercase();

        self.filtered_scenes = self.scenes.iter()
            .filter(|scene| {
                // Category filter
                let category_str = scene.category.as_deref().unwrap_or("daily");
                if !self.active_category.matches(category_str) {
                    return false;
                }

                // Search filter
                if query.is_empty() {
                    return true;
                }

                let name_zh_matches = scene.name_zh.to_lowercase().contains(&query);
                let name_en_matches = scene.name_en.to_lowercase().contains(&query);

                name_zh_matches || name_en_matches
            })
            .cloned()
            .collect();
    }

    /// Update category button visual states
    fn update_category_buttons(&mut self, cx: &mut Cx) {
        // Color values for selected/unselected state
        let white = vec4(1.0, 1.0, 1.0, 1.0);
        let gray = vec4(0.392, 0.455, 0.545, 1.0); // #64748b

        let buttons = [
            (ids!(search_bar.filter_chips.filter_all), SceneCategory::All),
            (ids!(search_bar.filter_chips.filter_daily), SceneCategory::Daily),
            (ids!(search_bar.filter_chips.filter_travel), SceneCategory::Travel),
            (ids!(search_bar.filter_chips.filter_business), SceneCategory::Business),
        ];

        for (widget_id, category) in buttons {
            let is_selected = self.active_category == category;
            let selected_val = if is_selected { 1.0f64 } else { 0.0f64 };
            let button = self.view.view(widget_id);

            // Update background
            button.apply_over(cx, live! {
                draw_bg: { selected: (selected_val) }
            });

            // Update label color using apply_over with color values
            let label = button.label(ids!(label));
            let text_color = if is_selected { white } else { gray };
            label.apply_over(cx, live! { draw_text: { color: (text_color) } });
        }
    }

    /// Update scene cards with filtered data
    fn update_scene_cards(&mut self, cx: &mut Cx) {
        let card_ids = [
            ids!(today_section.today_cards.row1.card0),
            ids!(today_section.today_cards.row1.card1),
            ids!(today_section.today_cards.row1.card2),
            ids!(today_section.today_cards.row1.card3),
            ids!(today_section.today_cards.row2.card4),
            ids!(today_section.today_cards.row2.card5),
            ids!(today_section.today_cards.row2.card6),
            ids!(today_section.today_cards.row2.card7),
        ];

        for (i, card_id) in card_ids.iter().enumerate() {
            let card = self.view.view(*card_id);

            if i < self.filtered_scenes.len() {
                let scene = &self.filtered_scenes[i];

                // Show card
                card.set_visible(cx, true);

                // Set icon
                let icon = scene.icon_emoji.as_deref().unwrap_or("🎭");
                card.label(ids!(icon)).set_text(cx, icon);

                // Set Chinese title
                card.label(ids!(title)).set_text(cx, &scene.name_zh);

                // Set English title
                card.label(ids!(title_en)).set_text(cx, &scene.name_en);

                // Set difficulty badge
                let difficulty = scene.difficulty_level.as_deref().unwrap_or("intermediate");
                let (stars, badge_color) = match difficulty {
                    "beginner" => ("⭐", 0.0),
                    "intermediate" => ("⭐⭐", 1.0),
                    "advanced" => ("⭐⭐⭐", 2.0),
                    _ => ("⭐⭐", 1.0),
                };
                card.label(ids!(info_row.difficulty_badge.badge_label)).set_text(cx, stars);
                card.view(ids!(info_row.difficulty_badge)).apply_over(cx, live! {
                    draw_bg: { badge_color: (badge_color) }
                });

                // Set duration (use fixed value since Scene doesn't have duration field)
                card.label(ids!(info_row.duration)).set_text(cx, "🕐 5分钟");
            } else {
                // Hide card if no data
                card.set_visible(cx, false);
            }
        }
    }

    /// Update classic dialogue cards
    fn update_classic_cards(&mut self, cx: &mut Cx) {
        let card_ids = [
            ids!(classic_section.classic_cards.classic_row1.classic0),
            ids!(classic_section.classic_cards.classic_row1.classic1),
            ids!(classic_section.classic_cards.classic_row2.classic2),
            ids!(classic_section.classic_cards.classic_row2.classic3),
        ];

        for (i, card_id) in card_ids.iter().enumerate() {
            let card = self.view.view(*card_id);

            if i < self.classic_sources.len() {
                let source = &self.classic_sources[i];

                // Show card
                card.set_visible(cx, true);

                // Set icon based on source type
                let icon = match source.source_type.as_str() {
                    "movie" => "🎥",
                    "tv_show" => "📺",
                    "ted_talk" => "🎤",
                    _ => "🎬",
                };
                card.label(ids!(icon_area.icon)).set_text(cx, icon);

                // Set title
                let title = format!("《{}》", source.title);
                card.label(ids!(content.title)).set_text(cx, &title);

                // Set description
                let desc = source.description_zh.as_deref().unwrap_or("经典场景");
                card.label(ids!(content.description)).set_text(cx, desc);
            } else {
                // Hide card if no data
                card.set_visible(cx, false);
            }
        }
    }

    /// Load data from the API
    fn load_data(&mut self, cx: &mut Cx) {
        self.scenes_loading = true;
        self.classic_loading = true;

        // Show loading labels
        self.view
            .view(ids!(today_section.loading_label))
            .set_visible(cx, true);
        self.view
            .view(ids!(classic_section.classic_loading_label))
            .set_visible(cx, true);

        let (tx, rx) = mpsc::channel();
        self.fetch_rx = Some(rx);

        // Spawn async task to fetch data
        let tx1 = tx.clone();
        let tx2 = tx;

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(api) = get_asset_api() {
                    if let Ok(client) = api.read() {
                        // Fetch scenes
                        let scenes_result = client.list_scenes(None, None, Some(10)).await;
                        let _ = tx1.send(FetchResult::Scenes(scenes_result));

                        // Fetch classic sources
                        let sources_result = client.list_classic_sources(None, Some(10)).await;
                        let _ = tx2.send(FetchResult::ClassicSources(sources_result));
                    }
                }
            });
        });

        self.view.redraw(cx);
    }
}

impl ScenesRef {
    /// Refresh data from API
    pub fn refresh_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.data_loaded = false;
            inner.load_data(cx);
        }
    }
}
