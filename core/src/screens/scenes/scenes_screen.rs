//! Role Play Screen - Scene selection and immersive role-play learning
//!
//! Layout based on website frontend StagesPage.tsx:
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

    // Orange/Amber theme colors matching website
    ORANGE_50 = #fff7ed
    ORANGE_100 = #ffedd5
    ORANGE_400 = #fb923c
    ORANGE_500 = #f97316
    ORANGE_600 = #ea580c
    ORANGE_700 = #c2410c
    AMBER_50 = #fffbeb
    AMBER_500 = #f59e0b
    YELLOW_50 = #fefce8

    // Difficulty badge colors
    GREEN_50 = #f0fdf4
    GREEN_600 = #16a34a
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

    // Category filter button - matches website filter chips
    // Selected: orange-500 bg with white text
    // Unselected: gray-100 bg with gray-600 text
    CategoryButton = <RoundedView> {
        width: Fit, height: 32
        padding: {left: 12, right: 12}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance hover: 0.0
            instance dark_mode: 0.0
            border_radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Selected: orange-500, Hover: orange-50, Normal: gray-100
                let orange_500 = vec4(0.976, 0.451, 0.086, 1.0);
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let gray_100 = vec4(0.953, 0.961, 0.969, 1.0);
                let dark_selected = (SLATE_600);
                let dark_hover = (SLATE_700);
                let dark_normal = (SLATE_800);

                let light_normal = mix(gray_100, orange_50, self.hover);
                let dark_base = mix(dark_normal, dark_hover, self.hover);
                let normal = mix(light_normal, dark_base, self.dark_mode);
                let selected_color = mix(orange_500, dark_selected, self.dark_mode);

                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(mix(normal, selected_color, self.selected));
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand
    }

    // Scene card - matches website StageCard component
    // "bg-white border rounded-xl p-4 hover:shadow-lg hover:border-orange-200"
    ScenesCardTemplate = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 8
        cursor: Hand
        show_bg: true

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            border_radius: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow on hover
                if self.hover > 0.5 {
                    let shadow = vec4(0.0, 0.0, 0.0, 0.08);
                    sdf.box(2.0, 4.0, self.rect_size.x - 4.0, self.rect_size.y - 2.0, 12.0);
                    sdf.fill(shadow);
                }

                // Card background
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0);
                let dark_bg = (SLATE_800);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));

                // Border - gray normally, orange-200 on hover
                let light_border = vec4(0.898, 0.906, 0.922, 1.0); // gray-200
                let hover_border = vec4(0.992, 0.796, 0.545, 1.0); // orange-200
                let dark_border = (SLATE_700);
                let border = mix(
                    mix(light_border, dark_border, self.dark_mode),
                    hover_border,
                    self.hover
                );
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }

        // Icon at top - text-4xl
        icon = <Label> {
            text: "🍽️"
            draw_text: {
                text_style: <FONT_BOLD>{ font_size: 32.0 }
            }
        }

        // Chinese title - font-semibold text-gray-900
        title = <Label> {
            text: "场景名称"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    let light = vec4(0.110, 0.118, 0.149, 1.0); // gray-900
                    let dark = (TEXT_PRIMARY_DARK);
                    return mix(light, dark, self.dark_mode);
                }
            }
        }

        // English subtitle - text-sm text-gray-500
        title_en = <Label> {
            text: "Scene Name"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    let light = vec4(0.420, 0.447, 0.502, 1.0); // gray-500
                    let dark = (SLATE_400);
                    return mix(light, dark, self.dark_mode);
                }
            }
        }

        // Bottom row with difficulty badge
        info_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8
            align: {y: 0.5}
            margin: {top: 4}

            // Difficulty badge - rounded-full with colored bg
            difficulty_badge = <RoundedView> {
                width: Fit, height: 22
                padding: {left: 8, right: 8}
                show_bg: true
                draw_bg: {
                    instance badge_color: 0.0  // 0=green, 1=amber, 2=red
                    border_radius: 11.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let green = vec4(0.941, 0.992, 0.957, 1.0);   // green-50
                        let amber = vec4(1.0, 0.984, 0.922, 1.0);     // amber-50
                        let red = vec4(0.996, 0.949, 0.949, 1.0);     // red-50
                        let bg = vec4(0.0);
                        if self.badge_color < 0.5 { bg = green; }
                        else if self.badge_color < 1.5 { bg = amber; }
                        else { bg = red; }
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 11.0);
                        sdf.fill(bg);
                        return sdf.result;
                    }
                }
                align: {x: 0.5, y: 0.5}

                badge_label = <Label> {
                    text: "⭐"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    }
                }
            }

            <View> { width: Fill }

            // Play button (shows on hover in website)
            play_btn = <View> {
                width: 28, height: 28
                show_bg: true
                draw_bg: {
                    instance hover: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Ghost button style
                        let normal = vec4(0.0, 0.0, 0.0, 0.0);
                        let hover_bg = vec4(0.953, 0.961, 0.969, 1.0); // gray-100
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                        sdf.fill(mix(normal, hover_bg, self.hover));
                        return sdf.result;
                    }
                }
                align: {x: 0.5, y: 0.5}

                <Label> {
                    text: "▶"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        color: (SLATE_600)
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
    // Scenes Screen Main Widget - matches website StagesPage.tsx
    // ========================================================================

    pub Scenes = {{Scenes}} <View> {
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

        // Scrollable content
        scene_list = <ScrollYView> {
            width: Fill, height: Fill
            flow: Down
            spacing: 24
            padding: {left: 16, right: 16, top: 16, bottom: 24}

            // Header with title - matches website layout
            header = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12
                align: {y: 0.5}
                margin: {bottom: 8}

                title = <Label> {
                    text: "🎭 角色扮演"
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

                subtitle = <Label> {
                    text: "沉浸式场景模拟 · AI智能推荐"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        fn get_color(self) -> vec4 {
                            let light = vec4(0.420, 0.447, 0.502, 1.0); // gray-500
                            let dark = (SLATE_400);
                            return mix(light, dark, self.dark_mode);
                        }
                    }
                }
            }

            // Search and Filters card - matches website white card with border
            search_bar = <RoundedView> {
                width: Fill, height: Fit
                padding: 12
                flow: Down
                spacing: 8
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    border_radius: 12.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Shadow
                        let shadow = vec4(0.0, 0.0, 0.0, 0.04);
                        sdf.box(1.0, 2.0, self.rect_size.x - 2.0, self.rect_size.y - 1.0, 12.0);
                        sdf.fill(shadow);
                        // Card bg
                        let light = vec4(1.0, 1.0, 1.0, 1.0);
                        let dark = (SLATE_800);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                        sdf.fill(mix(light, dark, self.dark_mode));
                        // Border
                        let border = mix(vec4(0.898, 0.906, 0.922, 1.0), (SLATE_700), self.dark_mode);
                        sdf.stroke(border, 1.0);
                        return sdf.result;
                    }
                }

                // Row 1: Search and Filters
                search_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}

                    // Search input
                    search_input = <RoundedView> {
                        width: Fill, height: 36
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
                                let light = vec4(0.976, 0.980, 0.984, 1.0); // gray-50
                                let dark = (SLATE_700);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                sdf.fill(mix(light, dark, self.dark_mode));
                                return sdf.result;
                            }
                        }

                        search_icon = <Label> {
                            text: "🔍"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            }
                        }

                        search_text_input = <TextInput> {
                            width: Fill, height: Fit
                            empty_text: "搜索场景..."
                            draw_bg: { color: #0000 }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Filter chips
                    filter_chips = <View> {
                        width: Fit, height: 36
                        flow: Right
                        spacing: 6
                        align: {y: 0.5}

                        filter_all = <CategoryButton> {
                            draw_bg: { selected: 1.0 }
                            label = <Label> {
                                text: "全部"
                                draw_text: {
                                    instance selected: 1.0
                                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        let unselected = vec4(0.298, 0.333, 0.388, 1.0); // gray-600
                                        let selected_color = vec4(1.0, 1.0, 1.0, 1.0);
                                        return mix(unselected, selected_color, self.selected);
                                    }
                                }
                            }
                        }

                        filter_daily = <CategoryButton> {
                            draw_bg: { selected: 0.0 }
                            label = <Label> {
                                text: "日常生活"
                                draw_text: {
                                    instance selected: 0.0
                                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        let unselected = vec4(0.298, 0.333, 0.388, 1.0);
                                        let selected_color = vec4(1.0, 1.0, 1.0, 1.0);
                                        return mix(unselected, selected_color, self.selected);
                                    }
                                }
                            }
                        }

                        filter_travel = <CategoryButton> {
                            draw_bg: { selected: 0.0 }
                            label = <Label> {
                                text: "旅行出行"
                                draw_text: {
                                    instance selected: 0.0
                                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        let unselected = vec4(0.298, 0.333, 0.388, 1.0);
                                        let selected_color = vec4(1.0, 1.0, 1.0, 1.0);
                                        return mix(unselected, selected_color, self.selected);
                                    }
                                }
                            }
                        }

                        filter_business = <CategoryButton> {
                            draw_bg: { selected: 0.0 }
                            label = <Label> {
                                text: "商务职场"
                                draw_text: {
                                    instance selected: 0.0
                                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        let unselected = vec4(0.298, 0.333, 0.388, 1.0);
                                        let selected_color = vec4(1.0, 1.0, 1.0, 1.0);
                                        return mix(unselected, selected_color, self.selected);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Continue Learning Section - matches website layout (inside search card)
            continue_learning_section = <RoundedView> {
                width: Fill, height: Fit
                padding: 12
                flow: Right
                spacing: 12
                align: {y: 0.5}
                cursor: Hand
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    instance hover: 0.0
                    border_radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Gradient from orange-50 to amber-50
                        let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                        let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                        let orange_100 = vec4(1.0, 0.929, 0.835, 1.0);
                        let amber_100 = vec4(0.996, 0.941, 0.780, 1.0);
                        let dark_bg = (SLATE_700);
                        let dark_hover = (SLATE_600);

                        // Light mode: gradient, Dark mode: slate
                        let light_start = mix(orange_50, orange_100, self.hover);
                        let light_end = mix(amber_50, amber_100, self.hover);
                        let light_color = mix(light_start, light_end, self.pos.x);
                        let dark_color = mix(dark_bg, dark_hover, self.hover);

                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill(mix(light_color, dark_color, self.dark_mode));
                        return sdf.result;
                    }
                }

                // Icon
                continue_icon = <Label> {
                    text: "🏨"
                    draw_text: {
                        text_style: <FONT_BOLD>{ font_size: 24.0 }
                    }
                }

                // Content
                continue_content = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 2

                    continue_title_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        continue_title = <Label> {
                            text: "酒店入住"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    let light = vec4(0.110, 0.118, 0.149, 1.0); // gray-900
                                    let dark = (TEXT_PRIMARY_DARK);
                                    return mix(light, dark, self.dark_mode);
                                }
                            }
                        }

                        continue_hint = <Label> {
                            text: "开始学习"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    let light = vec4(0.420, 0.447, 0.502, 1.0); // gray-500
                                    let dark = (SLATE_400);
                                    return mix(light, dark, self.dark_mode);
                                }
                            }
                        }
                    }
                }

                // Button
                continue_btn = <Button> {
                    width: Fit, height: 28
                    padding: {left: 12, right: 12}
                    text: "继续学习 ›"
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let base = vec4(0.976, 0.451, 0.086, 1.0);  // orange-500
                            let hover_color = vec4(0.918, 0.345, 0.047, 1.0); // orange-600
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                            sdf.fill(mix(base, hover_color, self.hover));
                            return sdf.result;
                        }
                    }
                    draw_text: {
                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                        color: (WHITE)
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
        let search_input = self.view.text_input(ids!(scene_list.search_bar.search_row.search_input.search_text_input));
        let current_text = search_input.text();
        if current_text != self.search_query {
            self.search_query = current_text;
            self.filter_scenes();
            self.update_scene_cards(cx);
            self.view.redraw(cx);
        }

        // Handle category filter clicks
        for (widget_id, category) in [
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_all), SceneCategory::All),
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_daily), SceneCategory::Daily),
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_travel), SceneCategory::Travel),
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_business), SceneCategory::Business),
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
                        .view(ids!(scene_list.today_section.loading_label))
                        .set_visible(cx, false);
                    self.update_scene_cards(cx);
                    self.view.redraw(cx);
                }
                Err(e) => {
                    eprintln!("Failed to fetch scenes: {}", e);
                    self.scenes_loading = false;
                    self.view
                        .label(ids!(scene_list.today_section.loading_label.label))
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
                        .view(ids!(scene_list.classic_section.classic_loading_label))
                        .set_visible(cx, false);
                    self.update_classic_cards(cx);
                    self.view.redraw(cx);
                }
                Err(e) => {
                    eprintln!("Failed to fetch classic sources: {}", e);
                    self.classic_loading = false;
                    self.view
                        .label(ids!(scene_list.classic_section.classic_loading_label.label))
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
        let buttons = [
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_all), SceneCategory::All),
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_daily), SceneCategory::Daily),
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_travel), SceneCategory::Travel),
            (ids!(scene_list.search_bar.search_row.filter_chips.filter_business), SceneCategory::Business),
        ];

        for (widget_id, category) in buttons {
            let is_selected = self.active_category == category;
            let selected_val = if is_selected { 1.0f64 } else { 0.0f64 };
            let button = self.view.view(widget_id);

            // Update background
            button.apply_over(cx, live! {
                draw_bg: { selected: (selected_val) }
            });

            // Update label selected state
            let label = button.label(ids!(label));
            label.apply_over(cx, live! { draw_text: { selected: (selected_val) } });
        }
    }

    /// Update scene cards with filtered data
    fn update_scene_cards(&mut self, cx: &mut Cx) {
        let card_ids = [
            ids!(scene_list.today_section.today_cards.row1.card0),
            ids!(scene_list.today_section.today_cards.row1.card1),
            ids!(scene_list.today_section.today_cards.row1.card2),
            ids!(scene_list.today_section.today_cards.row1.card3),
            ids!(scene_list.today_section.today_cards.row2.card4),
            ids!(scene_list.today_section.today_cards.row2.card5),
            ids!(scene_list.today_section.today_cards.row2.card6),
            ids!(scene_list.today_section.today_cards.row2.card7),
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
            ids!(scene_list.classic_section.classic_cards.classic_row1.classic0),
            ids!(scene_list.classic_section.classic_cards.classic_row1.classic1),
            ids!(scene_list.classic_section.classic_cards.classic_row2.classic2),
            ids!(scene_list.classic_section.classic_cards.classic_row2.classic3),
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
            .view(ids!(scene_list.today_section.loading_label))
            .set_visible(cx, true);
        self.view
            .view(ids!(scene_list.classic_section.classic_loading_label))
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
