//! # Review Screen (温故知新)
//!
//! Main review hub page showing overview of all review types with count of items needing review.
//! Matches the website's ReviewPage.tsx design.

use makepad_widgets::*;
use std::sync::mpsc;

use crate::learn_api::{get_learn_api, ReviewStats};
use crate::models::Preferences;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // ========================================================================
    // Review Type Colors (matching website)
    // ========================================================================

    // Red for pronunciation
    RED_50 = #fef2f2
    RED_200 = #fecaca
    RED_600 = #dc2626

    // Blue for meaning
    BLUE_50 = #eff6ff
    BLUE_200 = #bfdbfe
    BLUE_600 = #3b82f6  // Using Tailwind blue-500 to avoid exponent parsing

    // Purple for spelling
    PURPLE_50 = #faf5ff
    PURPLE_200 = #d8b4fc
    PURPLE_600 = #a855f7  // Using Tailwind purple-500 to avoid exponent parsing

    // Green for recognition
    GREEN_50 = #f0fdf4
    GREEN_200 = #bbf7d0
    GREEN_600 = #16a34a

    // Amber for phrase
    AMBER_50 = #fffbeb
    AMBER_200 = #fde68a
    AMBER_600 = #d97706

    // Orange for smart review
    ORANGE_50 = #fff7ed
    ORANGE_100 = #ffedd5
    ORANGE_300 = #fdba74
    ORANGE_400 = #fb923c
    ORANGE_500 = #f97316
    ORANGE_600 = #ea580c

    // ========================================================================
    // Review Type Card Component
    // ========================================================================

    ReviewTypeCard = <View> {
        width: Fill, height: Fit
        padding: 16
        show_bg: true
        cursor: Hand

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            instance card_type: 0.0  // 0=red, 1=blue, 2=purple, 3=green, 4=amber

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                // Background colors based on card_type
                let bg = vec4(0.0);
                if self.card_type < 0.5 {
                    // Red (pronunciation)
                    bg = mix((RED_50), #1f1412, self.dark_mode);
                } else if self.card_type < 1.5 {
                    // Blue (meaning)
                    bg = mix((BLUE_50), #121820, self.dark_mode);
                } else if self.card_type < 2.5 {
                    // Purple (spelling)
                    bg = mix((PURPLE_50), #1a1420, self.dark_mode);
                } else if self.card_type < 3.5 {
                    // Green (recognition)
                    bg = mix((GREEN_50), #121a14, self.dark_mode);
                } else {
                    // Amber (phrase)
                    bg = mix((AMBER_50), #1a1812, self.dark_mode);
                }

                // Hover effect
                let hover_bg = mix(bg, vec4(bg.xyz * 0.95, 1.0), self.hover);
                sdf.fill(hover_bg);

                // Border color
                let border = vec4(0.0);
                if self.card_type < 0.5 {
                    border = mix((RED_200), #4a2020, self.dark_mode);
                } else if self.card_type < 1.5 {
                    border = mix((BLUE_200), #203040, self.dark_mode);
                } else if self.card_type < 2.5 {
                    border = mix((PURPLE_200), #302040, self.dark_mode);
                } else if self.card_type < 3.5 {
                    border = mix((GREEN_200), #204030, self.dark_mode);
                } else {
                    border = mix((AMBER_200), #403020, self.dark_mode);
                }
                sdf.stroke(border, 2.0);

                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.15}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.15}} apply: {draw_bg: {hover: 1.0}} }
            }
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12
            align: {y: 0.0}

            icon_container = <RoundedView> {
                width: 44, height: 44
                align: {x: 0.5, y: 0.5}
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    instance card_type: 0.0
                    border_radius: 8.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                        let bg = vec4(0.0);
                        if self.card_type < 0.5 {
                            bg = mix((RED_50), #2a1818, self.dark_mode);
                        } else if self.card_type < 1.5 {
                            bg = mix((BLUE_50), #182028, self.dark_mode);
                        } else if self.card_type < 2.5 {
                            bg = mix((PURPLE_50), #201828, self.dark_mode);
                        } else if self.card_type < 3.5 {
                            bg = mix((GREEN_50), #182018, self.dark_mode);
                        } else {
                            bg = mix((AMBER_50), #282018, self.dark_mode);
                        }
                        sdf.fill(bg);

                        return sdf.result;
                    }
                }

                icon_label = <Label> {
                    text: "🎤"
                    draw_text: {
                        text_style: { font_size: 20.0 }
                        fn get_color(self) -> vec4 {
                            return (TEXT_PRIMARY);
                        }
                    }
                }
            }

            text_content = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 4

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    title = <Label> {
                        text: "发音纠正"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> { width: Fill }

                    count_badge = <RoundedView> {
                        width: Fit, height: Fit
                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                        visible: false
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            instance card_type: 0.0
                            border_radius: 10.0

                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);

                                let bg = vec4(0.0);
                                if self.card_type < 0.5 {
                                    bg = mix((RED_50), #3a2020, self.dark_mode);
                                } else if self.card_type < 1.5 {
                                    bg = mix((BLUE_50), #203040, self.dark_mode);
                                } else if self.card_type < 2.5 {
                                    bg = mix((PURPLE_50), #302040, self.dark_mode);
                                } else if self.card_type < 3.5 {
                                    bg = mix((GREEN_50), #204030, self.dark_mode);
                                } else {
                                    bg = mix((AMBER_50), #403020, self.dark_mode);
                                }
                                sdf.fill(bg);

                                return sdf.result;
                            }
                        }

                        count_label = <Label> {
                            text: "0"
                            draw_text: {
                                instance dark_mode: 0.0
                                instance card_type: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    if self.card_type < 0.5 {
                                        return (RED_600);
                                    } else if self.card_type < 1.5 {
                                        return (BLUE_600);
                                    } else if self.card_type < 2.5 {
                                        return (PURPLE_600);
                                    } else if self.card_type < 3.5 {
                                        return (GREEN_600);
                                    } else {
                                        return (AMBER_600);
                                    }
                                }
                            }
                        }
                    }
                }

                description = <Label> {
                    text: "练习准确发音，AI评分反馈"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }
        }
    }

    // ========================================================================
    // Smart Review Card (with gradient)
    // ========================================================================

    SmartReviewCard = <View> {
        width: Fill, height: Fit
        padding: 16
        show_bg: true
        cursor: Hand

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                // Gradient background
                let light_start = (ORANGE_50);
                let light_end = (AMBER_50);
                let dark_start = #1a1510;
                let dark_end = #1a1812;

                let start = mix(light_start, dark_start, self.dark_mode);
                let end = mix(light_end, dark_end, self.dark_mode);
                let bg = mix(start, end, self.pos.x);

                // Hover effect
                let hover_bg = mix(bg, vec4(bg.xyz * 0.95, 1.0), self.hover);
                sdf.fill(hover_bg);

                // Border
                let border = mix((ORANGE_300), #5a4020, self.dark_mode);
                sdf.stroke(border, 2.0);

                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.15}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.15}} apply: {draw_bg: {hover: 1.0}} }
            }
        }

        content = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12
            align: {y: 0.0}

            icon_container = <RoundedView> {
                width: 44, height: 44
                align: {x: 0.5, y: 0.5}
                show_bg: true
                draw_bg: {
                    border_radius: 8.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                        // Gradient from orange to amber
                        let start = (ORANGE_400);
                        let end = (ORANGE_500);
                        let bg = mix(start, end, self.pos.y);
                        sdf.fill(bg);

                        return sdf.result;
                    }
                }

                icon_label = <Label> {
                    text: "✨"
                    draw_text: {
                        text_style: { font_size: 20.0 }
                        color: (WHITE)
                    }
                }
            }

            text_content = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 4

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    title = <Label> {
                        text: "智能选择"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> { width: Fill }

                    count_badge = <RoundedView> {
                        width: Fit, height: Fit
                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                        visible: false
                        show_bg: true
                        draw_bg: {
                            border_radius: 10.0
                            color: (ORANGE_100)
                        }

                        count_label = <Label> {
                            text: "0"
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                color: (ORANGE_600)
                            }
                        }
                    }
                }

                description = <Label> {
                    text: "根据遗忘曲线智能推荐复习内容"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }
        }
    }

    // ========================================================================
    // Loading Spinner
    // ========================================================================

    LoadingView = <View> {
        width: Fill, height: 200
        align: {x: 0.5, y: 0.5}
        flow: Down
        spacing: 16

        spinner = <View> {
            width: 32, height: 32
            show_bg: true
            draw_bg: {
                instance spin: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 2.0;

                    // Spinning arc
                    let angle = self.spin * 6.28318;
                    for i in 0..8 {
                        let a = angle + float(i) * 0.785;
                        let opacity = float(i) / 8.0;
                        let px = c.x + cos(a) * r;
                        let py = c.y + sin(a) * r;
                        sdf.circle(px, py, 3.0);
                        sdf.fill(vec4(0.97, 0.52, 0.09, opacity));
                    }

                    return sdf.result;
                }
            }

            animator: {
                spin = {
                    default: loop
                    loop = {
                        from: {all: Loop {duration: 1.0, end: 1.0}}
                        apply: {draw_bg: {spin: 1.0}}
                    }
                }
            }
        }

        loading_text = <Label> {
            text: "加载中..."
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ========================================================================
    // Login Required Card
    // ========================================================================

    LoginRequiredCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 32
        align: {x: 0.5}
        flow: Down
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        icon = <Label> {
            text: "📚"
            draw_text: {
                text_style: { font_size: 48.0 }
            }
        }

        title = <Label> {
            text: "温故知新"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        subtitle = <Label> {
            text: "科学的间隔重复系统，帮助你巩固所学知识"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }

        login_btn = <Button> {
            width: Fit, height: Fit
            padding: {left: 24, right: 24, top: 12, bottom: 12}
            text: "登录开始复习"
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill((ACCENT_PRIMARY));
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
    // Main Review Screen
    // ========================================================================

    pub ReviewScreen = {{ReviewScreen}} {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Gradient background matching website
                let light_start = (ORANGE_50);
                let light_mid = (AMBER_50);
                let light_end = #fefce8; // yellow-50
                let dark_bg = (DARK_BG_DARK);

                let light = mix(mix(light_start, light_mid, self.pos.x), light_end, self.pos.y);
                return mix(light, dark_bg, self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: 16
                spacing: 16

                // Main card container
                main_card = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 24
                    flow: Down
                    spacing: 24
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 12.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                            let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                            sdf.fill(bg);
                            // Shadow effect
                            return sdf.result;
                        }
                    }

                    // Header
                    header = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 1.0}
                        spacing: 12

                        title = <Label> {
                            text: "📚 温故知新"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_BOLD>{ font_size: 20.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        subtitle = <Label> {
                            text: "科学复习，牢记所学"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Loading state
                    loading_view = <LoadingView> {
                        visible: false
                    }

                    // Login required state
                    login_required = <LoginRequiredCard> {
                        visible: false
                    }

                    // Cards grid (3 columns on desktop, responsive)
                    cards_grid = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 12

                        // Row 1: Smart + Pronunciation + Meaning
                        row1 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 12

                            smart_card = <SmartReviewCard> {}
                            pronunciation_card = <ReviewTypeCard> {
                                draw_bg: { card_type: 0.0 }
                                content = {
                                    icon_container = {
                                        draw_bg: { card_type: 0.0 }
                                        icon_label = { text: "🎤" }
                                    }
                                    text_content = {
                                        header_row = {
                                            title = { text: "发音纠正" }
                                            count_badge = {
                                                draw_bg: { card_type: 0.0 }
                                                count_label = { draw_text: { card_type: 0.0 } }
                                            }
                                        }
                                        description = { text: "练习准确发音，AI评分反馈" }
                                    }
                                }
                            }
                            meaning_card = <ReviewTypeCard> {
                                draw_bg: { card_type: 1.0 }
                                content = {
                                    icon_container = {
                                        draw_bg: { card_type: 1.0 }
                                        icon_label = { text: "📖" }
                                    }
                                    text_content = {
                                        header_row = {
                                            title = { text: "词义复习" }
                                            count_badge = {
                                                draw_bg: { card_type: 1.0 }
                                                count_label = { draw_text: { card_type: 1.0 } }
                                            }
                                        }
                                        description = { text: "巩固单词释义，选择正确答案" }
                                    }
                                }
                            }
                        }

                        // Row 2: Spelling + Recognition + Phrase
                        row2 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 12

                            spelling_card = <ReviewTypeCard> {
                                draw_bg: { card_type: 2.0 }
                                content = {
                                    icon_container = {
                                        draw_bg: { card_type: 2.0 }
                                        icon_label = { text: "✏️" }
                                    }
                                    text_content = {
                                        header_row = {
                                            title = { text: "拼写练习" }
                                            count_badge = {
                                                draw_bg: { card_type: 2.0 }
                                                count_label = { draw_text: { card_type: 2.0 } }
                                            }
                                        }
                                        description = { text: "听音拼写，强化记忆" }
                                    }
                                }
                            }
                            recognition_card = <ReviewTypeCard> {
                                draw_bg: { card_type: 3.0 }
                                content = {
                                    icon_container = {
                                        draw_bg: { card_type: 3.0 }
                                        icon_label = { text: "🃏" }
                                    }
                                    text_content = {
                                        header_row = {
                                            title = { text: "单词识记" }
                                            count_badge = {
                                                draw_bg: { card_type: 3.0 }
                                                count_label = { draw_text: { card_type: 3.0 } }
                                            }
                                        }
                                        description = { text: "翻卡片复习，自评掌握程度" }
                                    }
                                }
                            }
                            phrase_card = <ReviewTypeCard> {
                                draw_bg: { card_type: 4.0 }
                                content = {
                                    icon_container = {
                                        draw_bg: { card_type: 4.0 }
                                        icon_label = { text: "🧩" }
                                    }
                                    text_content = {
                                        header_row = {
                                            title = { text: "短语搭配" }
                                            count_badge = {
                                                draw_bg: { card_type: 4.0 }
                                                count_label = { draw_text: { card_type: 4.0 } }
                                            }
                                        }
                                        description = { text: "填空练习，掌握常用搭配" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// Review Type Enum
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReviewType {
    Smart,
    Pronunciation,
    Meaning,
    Spelling,
    Recognition,
    Phrase,
}

impl ReviewType {
    pub fn api_name(&self) -> &'static str {
        match self {
            ReviewType::Smart => "smart",
            ReviewType::Pronunciation => "pronunciation",
            ReviewType::Meaning => "meaning",
            ReviewType::Spelling => "spelling",
            ReviewType::Recognition => "recognition",
            ReviewType::Phrase => "phrase",
        }
    }
}

// ============================================================================
// Actions
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum ReviewScreenAction {
    None,
    NavigateToSession { review_type: ReviewType },
    NavigateToLogin,
}

// ============================================================================
// Fetch Result
// ============================================================================

enum FetchResult {
    Stats(Result<ReviewStats, String>),
}

// ============================================================================
// Widget Implementation
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct ReviewScreen {
    #[deref]
    view: View,
    #[rust]
    data_loaded: bool,
    #[rust]
    is_loading: bool,
    #[rust]
    is_authenticated: bool,
    #[rust]
    stats: ReviewStats,
    #[rust]
    fetch_rx: Option<mpsc::Receiver<FetchResult>>,
}

impl Widget for ReviewScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Load data on first draw
        if let Event::Draw(_) = event {
            if !self.data_loaded {
                self.data_loaded = true;
                self.check_auth_and_load(cx);
            }
        }

        // Check for fetch results
        let results: Vec<_> = self.fetch_rx.as_ref()
            .map(|rx| rx.try_iter().collect())
            .unwrap_or_default();

        for result in results {
            match result {
                FetchResult::Stats(Ok(stats)) => {
                    self.stats = stats;
                    self.is_loading = false;
                    self.update_ui(cx);
                }
                FetchResult::Stats(Err(e)) => {
                    eprintln!("Failed to fetch review stats: {}", e);
                    self.is_loading = false;
                    self.update_ui(cx);
                }
            }
        }

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle card clicks
        let card_ids = [
            (ids!(content_scroll.content.main_card.cards_grid.row1.smart_card), ReviewType::Smart),
            (ids!(content_scroll.content.main_card.cards_grid.row1.pronunciation_card), ReviewType::Pronunciation),
            (ids!(content_scroll.content.main_card.cards_grid.row1.meaning_card), ReviewType::Meaning),
            (ids!(content_scroll.content.main_card.cards_grid.row2.spelling_card), ReviewType::Spelling),
            (ids!(content_scroll.content.main_card.cards_grid.row2.recognition_card), ReviewType::Recognition),
            (ids!(content_scroll.content.main_card.cards_grid.row2.phrase_card), ReviewType::Phrase),
        ];

        for (card_id, review_type) in card_ids {
            if self.view.view(card_id).finger_up(actions).is_some() {
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    ReviewScreenAction::NavigateToSession { review_type },
                );
            }
        }

        // Handle login button
        if self.view.button(ids!(content_scroll.content.main_card.login_required.login_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ReviewScreenAction::NavigateToLogin);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReviewScreen {
    fn check_auth_and_load(&mut self, cx: &mut Cx) {
        let prefs = Preferences::load();
        self.is_authenticated = prefs.auth_token.is_some();

        if self.is_authenticated {
            self.is_loading = true;
            self.load_stats(cx);
        }

        self.update_ui(cx);
    }

    fn load_stats(&mut self, _cx: &mut Cx) {
        let (tx, rx) = mpsc::channel();
        self.fetch_rx = Some(rx);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(api) = get_learn_api() {
                    if let Ok(client) = api.read() {
                        let result = client.get_review_stats().await;
                        let _ = tx.send(FetchResult::Stats(result));
                    }
                }
            });
        });
    }

    fn update_ui(&mut self, cx: &mut Cx) {
        // Show/hide based on auth state
        self.view.view(ids!(content_scroll.content.main_card.login_required))
            .set_visible(cx, !self.is_authenticated);
        self.view.view(ids!(content_scroll.content.main_card.cards_grid))
            .set_visible(cx, self.is_authenticated && !self.is_loading);
        self.view.view(ids!(content_scroll.content.main_card.loading_view))
            .set_visible(cx, self.is_authenticated && self.is_loading);

        if self.is_authenticated && !self.is_loading {
            self.update_counts(cx);
        }

        self.view.redraw(cx);
    }

    fn update_counts(&mut self, cx: &mut Cx) {
        let total = self.stats.total();

        // Smart card
        self.update_card_count(
            cx,
            ids!(content_scroll.content.main_card.cards_grid.row1.smart_card.content.text_content.header_row.count_badge),
            ids!(content_scroll.content.main_card.cards_grid.row1.smart_card.content.text_content.header_row.count_badge.count_label),
            total,
        );

        // Pronunciation card
        self.update_card_count(
            cx,
            ids!(content_scroll.content.main_card.cards_grid.row1.pronunciation_card.content.text_content.header_row.count_badge),
            ids!(content_scroll.content.main_card.cards_grid.row1.pronunciation_card.content.text_content.header_row.count_badge.count_label),
            self.stats.pronunciation,
        );

        // Meaning card
        self.update_card_count(
            cx,
            ids!(content_scroll.content.main_card.cards_grid.row1.meaning_card.content.text_content.header_row.count_badge),
            ids!(content_scroll.content.main_card.cards_grid.row1.meaning_card.content.text_content.header_row.count_badge.count_label),
            self.stats.meaning,
        );

        // Spelling card
        self.update_card_count(
            cx,
            ids!(content_scroll.content.main_card.cards_grid.row2.spelling_card.content.text_content.header_row.count_badge),
            ids!(content_scroll.content.main_card.cards_grid.row2.spelling_card.content.text_content.header_row.count_badge.count_label),
            self.stats.spelling,
        );

        // Recognition card
        self.update_card_count(
            cx,
            ids!(content_scroll.content.main_card.cards_grid.row2.recognition_card.content.text_content.header_row.count_badge),
            ids!(content_scroll.content.main_card.cards_grid.row2.recognition_card.content.text_content.header_row.count_badge.count_label),
            self.stats.recognition,
        );

        // Phrase card
        self.update_card_count(
            cx,
            ids!(content_scroll.content.main_card.cards_grid.row2.phrase_card.content.text_content.header_row.count_badge),
            ids!(content_scroll.content.main_card.cards_grid.row2.phrase_card.content.text_content.header_row.count_badge.count_label),
            self.stats.phrase,
        );
    }

    fn update_card_count(&mut self, cx: &mut Cx, badge_id: &[LiveId], label_id: &[LiveId], count: i32) {
        self.view.view(badge_id).set_visible(cx, count > 0);
        if count > 0 {
            self.view.label(label_id).set_text(cx, &count.to_string());
        }
    }
}
