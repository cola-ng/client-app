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
    // Simple Card Component for Review Types
    // ========================================================================

    SimpleCard = <View> {
        width: Fill, height: Fit
        padding: 16
        show_bg: true
        cursor: Hand

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            instance card_color: 0.0  // 0=orange, 1=red, 2=blue, 3=purple, 4=green, 5=amber

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                // Background colors based on card_color
                let bg = vec4(0.0);
                let border = vec4(0.0);

                if self.card_color < 0.5 {
                    // Orange (smart)
                    bg = mix(#fff7ed, #1a1510, self.dark_mode);
                    border = mix(#fdba74, #5a4020, self.dark_mode);
                } else if self.card_color < 1.5 {
                    // Red (pronunciation)
                    bg = mix(#fef2f2, #1f1412, self.dark_mode);
                    border = mix(#fecaca, #4a2020, self.dark_mode);
                } else if self.card_color < 2.5 {
                    // Blue (meaning)
                    bg = mix(#eff6ff, #121820, self.dark_mode);
                    border = mix(#bfdbfe, #203040, self.dark_mode);
                } else if self.card_color < 3.5 {
                    // Purple (spelling)
                    bg = mix(#faf5ff, #1a1420, self.dark_mode);
                    border = mix(#d8b4fc, #302040, self.dark_mode);
                } else if self.card_color < 4.5 {
                    // Green (recognition)
                    bg = mix(#f0fdf4, #121a14, self.dark_mode);
                    border = mix(#bbf7d0, #204030, self.dark_mode);
                } else {
                    // Amber (phrase)
                    bg = mix(#fffbeb, #1a1812, self.dark_mode);
                    border = mix(#fde68a, #403020, self.dark_mode);
                }

                let hover_bg = mix(bg, vec4(bg.xyz * 0.95, 1.0), self.hover);
                sdf.fill(hover_bg);
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
                let light_start = #fff7ed;
                let light_end = #fefce8;
                let dark_bg = (DARK_BG_DARK);
                let light = mix(light_start, light_end, self.pos.y);
                return mix(light, dark_bg, self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            main_container = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: 16
                spacing: 16

                // Main card
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
                    loading_view = <View> {
                        width: Fill, height: 200
                        visible: false
                        align: {x: 0.5, y: 0.5}
                        flow: Down
                        spacing: 16

                        <Label> {
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

                    // Login required state
                    login_required = <View> {
                        width: Fill, height: Fit
                        visible: false
                        padding: 32
                        align: {x: 0.5}
                        flow: Down
                        spacing: 16

                        <Label> {
                            text: "📚"
                            draw_text: { text_style: { font_size: 48.0 } }
                        }

                        <Label> {
                            text: "温故知新"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_BOLD>{ font_size: 20.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        <Label> {
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

                    // Cards grid
                    cards_grid = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 12

                        // Row 1
                        row1 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 12

                            // Smart card
                            smart_card = <SimpleCard> {
                                draw_bg: { card_color: 0.0 }
                                flow: Down
                                spacing: 8

                                header_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    <View> {
                                        width: 44, height: 44
                                        align: {x: 0.5, y: 0.5}
                                        show_bg: true
                                        draw_bg: {
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                let start = #fb923c;
                                                let end = #f97316;
                                                sdf.fill(mix(start, end, self.pos.y));
                                                return sdf.result;
                                            }
                                        }
                                        <Label> {
                                            text: "✨"
                                            draw_text: { text_style: { font_size: 20.0 }, color: (WHITE) }
                                        }
                                    }

                                    <Label> {
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

                                    smart_badge = <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                                        visible: false
                                        show_bg: true
                                        draw_bg: { border_radius: 10.0, color: #ffedd5 }
                                        smart_count = <Label> {
                                            text: "0"
                                            draw_text: {
                                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                                color: #ea580c
                                            }
                                        }
                                    }
                                }

                                <Label> {
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

                            // Pronunciation card
                            pronunciation_card = <SimpleCard> {
                                draw_bg: { card_color: 1.0 }
                                flow: Down
                                spacing: 8

                                header_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    <View> {
                                        width: 44, height: 44
                                        align: {x: 0.5, y: 0.5}
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                sdf.fill(mix(#fef2f2, #2a1818, self.dark_mode));
                                                return sdf.result;
                                            }
                                        }
                                        <Label> { text: "🎤", draw_text: { text_style: { font_size: 20.0 } } }
                                    }

                                    <Label> {
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

                                    pronunciation_badge = <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                                        visible: false
                                        show_bg: true
                                        draw_bg: { border_radius: 10.0, color: #fef2f2 }
                                        pronunciation_count = <Label> {
                                            text: "0"
                                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 }, color: #dc2626 }
                                        }
                                    }
                                }

                                <Label> {
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

                            // Meaning card
                            meaning_card = <SimpleCard> {
                                draw_bg: { card_color: 2.0 }
                                flow: Down
                                spacing: 8

                                header_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    <View> {
                                        width: 44, height: 44
                                        align: {x: 0.5, y: 0.5}
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                sdf.fill(mix(#eff6ff, #182028, self.dark_mode));
                                                return sdf.result;
                                            }
                                        }
                                        <Label> { text: "📖", draw_text: { text_style: { font_size: 20.0 } } }
                                    }

                                    <Label> {
                                        text: "词义复习"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    <View> { width: Fill }

                                    meaning_badge = <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                                        visible: false
                                        show_bg: true
                                        draw_bg: { border_radius: 10.0, color: #eff6ff }
                                        meaning_count = <Label> {
                                            text: "0"
                                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 }, color: #3b82f6 }
                                        }
                                    }
                                }

                                <Label> {
                                    text: "巩固单词释义，选择正确答案"
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

                        // Row 2
                        row2 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 12

                            // Spelling card
                            spelling_card = <SimpleCard> {
                                draw_bg: { card_color: 3.0 }
                                flow: Down
                                spacing: 8

                                header_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    <View> {
                                        width: 44, height: 44
                                        align: {x: 0.5, y: 0.5}
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                sdf.fill(mix(#faf5ff, #201828, self.dark_mode));
                                                return sdf.result;
                                            }
                                        }
                                        <Label> { text: "✏️", draw_text: { text_style: { font_size: 20.0 } } }
                                    }

                                    <Label> {
                                        text: "拼写练习"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    <View> { width: Fill }

                                    spelling_badge = <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                                        visible: false
                                        show_bg: true
                                        draw_bg: { border_radius: 10.0, color: #faf5ff }
                                        spelling_count = <Label> {
                                            text: "0"
                                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 }, color: #a855f7 }
                                        }
                                    }
                                }

                                <Label> {
                                    text: "听音拼写，强化记忆"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                        }
                                    }
                                }
                            }

                            // Recognition card
                            recognition_card = <SimpleCard> {
                                draw_bg: { card_color: 4.0 }
                                flow: Down
                                spacing: 8

                                header_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    <View> {
                                        width: 44, height: 44
                                        align: {x: 0.5, y: 0.5}
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                sdf.fill(mix(#f0fdf4, #182018, self.dark_mode));
                                                return sdf.result;
                                            }
                                        }
                                        <Label> { text: "🃏", draw_text: { text_style: { font_size: 20.0 } } }
                                    }

                                    <Label> {
                                        text: "单词识记"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    <View> { width: Fill }

                                    recognition_badge = <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                                        visible: false
                                        show_bg: true
                                        draw_bg: { border_radius: 10.0, color: #f0fdf4 }
                                        recognition_count = <Label> {
                                            text: "0"
                                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 }, color: #16a34a }
                                        }
                                    }
                                }

                                <Label> {
                                    text: "翻卡片复习，自评掌握程度"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                        }
                                    }
                                }
                            }

                            // Phrase card
                            phrase_card = <SimpleCard> {
                                draw_bg: { card_color: 5.0 }
                                flow: Down
                                spacing: 8

                                header_row = <View> {
                                    width: Fill, height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: {y: 0.5}

                                    <View> {
                                        width: 44, height: 44
                                        align: {x: 0.5, y: 0.5}
                                        show_bg: true
                                        draw_bg: {
                                            instance dark_mode: 0.0
                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                                sdf.fill(mix(#fffbeb, #282018, self.dark_mode));
                                                return sdf.result;
                                            }
                                        }
                                        <Label> { text: "🧩", draw_text: { text_style: { font_size: 20.0 } } }
                                    }

                                    <Label> {
                                        text: "短语搭配"
                                        draw_text: {
                                            instance dark_mode: 0.0
                                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                            fn get_color(self) -> vec4 {
                                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                            }
                                        }
                                    }

                                    <View> { width: Fill }

                                    phrase_badge = <RoundedView> {
                                        width: Fit, height: Fit
                                        padding: {left: 8, right: 8, top: 2, bottom: 2}
                                        visible: false
                                        show_bg: true
                                        draw_bg: { border_radius: 10.0, color: #fffbeb }
                                        phrase_count = <Label> {
                                            text: "0"
                                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 }, color: #d97706 }
                                        }
                                    }
                                }

                                <Label> {
                                    text: "填空练习，掌握常用搭配"
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
            (ids!(content_scroll.main_container.main_card.cards_grid.row1.smart_card), ReviewType::Smart),
            (ids!(content_scroll.main_container.main_card.cards_grid.row1.pronunciation_card), ReviewType::Pronunciation),
            (ids!(content_scroll.main_container.main_card.cards_grid.row1.meaning_card), ReviewType::Meaning),
            (ids!(content_scroll.main_container.main_card.cards_grid.row2.spelling_card), ReviewType::Spelling),
            (ids!(content_scroll.main_container.main_card.cards_grid.row2.recognition_card), ReviewType::Recognition),
            (ids!(content_scroll.main_container.main_card.cards_grid.row2.phrase_card), ReviewType::Phrase),
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
        if self.view.button(ids!(content_scroll.main_container.main_card.login_required.login_btn)).clicked(actions) {
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
            self.load_stats();
        }

        self.update_ui(cx);
    }

    fn load_stats(&mut self) {
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
        self.view.view(ids!(content_scroll.main_container.main_card.login_required))
            .set_visible(cx, !self.is_authenticated);
        self.view.view(ids!(content_scroll.main_container.main_card.cards_grid))
            .set_visible(cx, self.is_authenticated && !self.is_loading);
        self.view.view(ids!(content_scroll.main_container.main_card.loading_view))
            .set_visible(cx, self.is_authenticated && self.is_loading);

        if self.is_authenticated && !self.is_loading {
            self.update_counts(cx);
        }

        self.view.redraw(cx);
    }

    fn update_counts(&mut self, cx: &mut Cx) {
        let total = self.stats.total();

        // Smart card - badge is now under header_row
        self.update_badge(cx, ids!(content_scroll.main_container.main_card.cards_grid.row1.smart_card.header_row.smart_badge),
                          ids!(content_scroll.main_container.main_card.cards_grid.row1.smart_card.header_row.smart_badge.smart_count), total);

        // Pronunciation
        self.update_badge(cx, ids!(content_scroll.main_container.main_card.cards_grid.row1.pronunciation_card.header_row.pronunciation_badge),
                          ids!(content_scroll.main_container.main_card.cards_grid.row1.pronunciation_card.header_row.pronunciation_badge.pronunciation_count), self.stats.pronunciation);

        // Meaning
        self.update_badge(cx, ids!(content_scroll.main_container.main_card.cards_grid.row1.meaning_card.header_row.meaning_badge),
                          ids!(content_scroll.main_container.main_card.cards_grid.row1.meaning_card.header_row.meaning_badge.meaning_count), self.stats.meaning);

        // Spelling
        self.update_badge(cx, ids!(content_scroll.main_container.main_card.cards_grid.row2.spelling_card.header_row.spelling_badge),
                          ids!(content_scroll.main_container.main_card.cards_grid.row2.spelling_card.header_row.spelling_badge.spelling_count), self.stats.spelling);

        // Recognition
        self.update_badge(cx, ids!(content_scroll.main_container.main_card.cards_grid.row2.recognition_card.header_row.recognition_badge),
                          ids!(content_scroll.main_container.main_card.cards_grid.row2.recognition_card.header_row.recognition_badge.recognition_count), self.stats.recognition);

        // Phrase
        self.update_badge(cx, ids!(content_scroll.main_container.main_card.cards_grid.row2.phrase_card.header_row.phrase_badge),
                          ids!(content_scroll.main_container.main_card.cards_grid.row2.phrase_card.header_row.phrase_badge.phrase_count), self.stats.phrase);
    }

    fn update_badge(&mut self, cx: &mut Cx, badge_id: &[LiveId], label_id: &[LiveId], count: i32) {
        self.view.view(badge_id).set_visible(cx, count > 0);
        if count > 0 {
            self.view.label(label_id).set_text(cx, &count.to_string());
        }
    }
}
