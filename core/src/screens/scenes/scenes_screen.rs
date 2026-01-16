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

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;use makepad_component::*;

    use widgets::theme::*;

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

    // Template for scenario card in today's section
    ScenesCardTemplate = <CardBase> {
        width: 180, height: 160
        padding: 12
        flow: Down
        spacing: 8
        cursor: Hand

        icon_area = <RoundedView> {
            width: Fill, height: 75
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 12.0
                fn pixel(self) -> vec4 {
                    let light_color = vec4(0.063, 0.725, 0.502, 0.2);
                    let dark_color = vec4(0.125, 0.65, 0.475, 0.3);
                    return mix(light_color, dark_color, self.dark_mode);
                }
            }
            align: {x: 0.5, y: 0.5}

            icon = <Label> {
                text: "🍽️"
                draw_text: {
                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                }
            }
        }

        title = <Label> {
            text: "场景名称"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        info = <Label> {
            text: "⭐⭐ · 5分钟"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                }
            }
        }
    }

    // Template for classic dialogue card
    ClassicCardTemplate = <CardBase> {
        width: 220, height: 100
        padding: 12
        flow: Right
        spacing: 12
        cursor: Hand

        icon_area = <RoundedView> {
            width: 60, height: 60
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
                    text_style: <FONT_BOLD>{ font_size: 28.0 }
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
                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
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
                        return mix((ACCENT_INDIGO), (INDIGO_400), self.dark_mode);
                    }
                }
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
                return mix((SLATE_50), (DARK_BG_DARK), self.dark_mode);
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

                search_input = <PanelBase> {
                    width: 300, height: 36
                    padding: {left: 12, right: 12}
                    flow: Right
                    align: {y: 0.5}

                    search_icon = <Label> {
                        text: "🔍"
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        }
                    }

                    search_placeholder = <Label> {
                        text: "搜索场景..."
                        draw_text: {
                            color: (TEXT_MUTED)
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        }
                    }
                }

                filter_chips = <View> {
                    width: Fit, height: 36
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}

                    filter_all = <RoundedView> {
                        width: Fit, height: 32
                        padding: {left: 12, right: 12}
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn get_color(self) -> vec4 {
                                return mix((ACCENT_INDIGO), (INDIGO_500), self.dark_mode);
                            }
                        }
                        align: {x: 0.5, y: 0.5}

                        filter_label = <Label> {
                            text: "全部"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (WHITE)
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
                    width: Fill, height: 120
                    padding: 16
                    flow: Right
                    spacing: 16
                    align: {y: 0.5}

                    icon_area = <RoundedView> {
                        width: 90, height: Fit
                        flow: Down
                        align: {x: 0.5, y: 0.5}

                        icon = <Label> {
                            text: "🏨"
                            draw_text: {
                                text_style: <FONT_BOLD>{ font_size: 48.0 }
                            }
                        }
                    }

                    content = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 8

                        title = <Label> {
                            text: "酒店入住"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        progress = <Label> {
                            text: "进度 60% · 还剩 3 个对话"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }

                        task = <Label> {
                            text: "下一个任务：前台预订房间"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        estimate = <Label> {
                            text: "预计耗时 8 分钟"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    button = <Button> {
                        width: 120, height: 40
                        text: "继续学习 →"
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let r = 8.0;
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, r);
                                let color = mix((ACCENT_INDIGO), (INDIGO_500), self.dark_mode);
                                sdf.fill(color);
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            color: (WHITE)
                        }
                    }
                }
            }

            // Today's Scenes Section (Dynamic)
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

                // Dynamic scenes list
                today_cards = <PortalList> {
                    width: Fill, height: 170
                    flow: Right
                    spacing: 12

                    scenetemplate = <ScenesCardTemplate> {}
                }
            }

            // Classic Dialogues Section (Dynamic)
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

                // Dynamic classic dialogues list
                classic_cards = <PortalList> {
                    width: Fill, height: 110
                    flow: Right
                    spacing: 16

                    classic_template = <ClassicCardTemplate> {}
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
    classic_sources: Vec<ClassicDialogueSource>,

    #[rust]
    scenes_loading: bool,

    #[rust]
    classic_loading: bool,

    #[rust]
    data_loaded: bool,

    #[rust]
    fetch_rx: Option<mpsc::Receiver<FetchResult>>,
}

impl Widget for Scenes {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Check for fetch results
        if let Some(rx) = &self.fetch_rx {
            while let Ok(result) = rx.try_recv() {
                match result {
                    FetchResult::Scenes(Ok(scenes)) => {
                        self.scenes = scenes;
                        self.scenes_loading = false;
                        self.view
                            .view(ids!(today_section.loading_label))
                            .set_visible(cx, false);
                        self.view.redraw(cx);
                    }
                    FetchResult::Scenes(Err(e)) => {
                        eprintln!("Failed to fetch scenes: {}", e);
                        self.scenes_loading = false;
                        self.view
                            .label(ids!(today_section.loading_label.label))
                            .set_text(cx, &format!("加载失败: {}", e));
                    }
                    FetchResult::ClassicSources(Ok(sources)) => {
                        self.classic_sources = sources;
                        self.classic_loading = false;
                        self.view
                            .view(ids!(classic_section.classic_loading_label))
                            .set_visible(cx, false);
                        self.view.redraw(cx);
                    }
                    FetchResult::ClassicSources(Err(e)) => {
                        eprintln!("Failed to fetch classic sources: {}", e);
                        self.classic_loading = false;
                        self.view
                            .label(ids!(classic_section.classic_loading_label.label))
                            .set_text(cx, &format!("加载失败: {}", e));
                    }
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
        let today_list_id = self.view.portal_list(ids!(today_cards)).widget_uid();
        let classic_list_id = self.view.portal_list(ids!(classic_cards)).widget_uid();

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                // Check which list this is by its live_id
                let list_id = list.widget_uid();

                if list_id == today_list_id {
                    // Render scenes
                    list.set_item_range(cx, 0, self.scenes.len());

                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < self.scenes.len() {
                            let item = list.item(cx, item_id, live_id!(scenetemplate));
                            let scenario = &self.scenes[item_id];

                            // Set icon
                            let icon = scenario.icon_emoji.as_deref().unwrap_or("🎭");
                            item.label(ids!(icon_area.icon)).set_text(cx, icon);

                            // Set title
                            item.label(ids!(title)).set_text(cx, &scenario.name_zh);

                            // Set info (difficulty and estimated time)
                            let difficulty = scenario.difficulty_level.as_deref().unwrap_or("中级");
                            let stars = match difficulty {
                                "beginner" => "⭐",
                                "intermediate" => "⭐⭐",
                                "advanced" => "⭐⭐⭐",
                                _ => "⭐⭐",
                            };
                            item.label(ids!(info))
                                .set_text(cx, &format!("{} · 5分钟", stars));

                            item.draw_all(cx, scope);
                        }
                    }
                } else if list_id == classic_list_id {
                    // Render classic sources
                    list.set_item_range(cx, 0, self.classic_sources.len());

                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id < self.classic_sources.len() {
                            let item = list.item(cx, item_id, live_id!(classic_template));
                            let source = &self.classic_sources[item_id];

                            // Set icon based on source type
                            let icon = match source.source_type.as_str() {
                                "movie" => "🎥",
                                "tv_show" => "📺",
                                "ted_talk" => "🎤",
                                _ => "🎬",
                            };
                            item.label(ids!(icon_area.icon)).set_text(cx, icon);

                            // Set title
                            let title = format!("《{}》", source.title);
                            item.label(ids!(content.title)).set_text(cx, &title);

                            // Set description
                            let desc = source.description_zh.as_deref().unwrap_or("经典场景");
                            item.label(ids!(content.description)).set_text(cx, desc);

                            item.draw_all(cx, scope);
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl Scenes {
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
