//! Dictionary Translation Screen - Word lookup and translation
//!
//! Features based on website DictPage.tsx:
//! - Tabs for "词典查询" (Dictionary Lookup) and "内容翻译" (Content Translation)
//! - Three-column layout: navigation, content, history
//! - Search bar with instant results
//! - Translation panel with language style options

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Orange theme colors matching website (from-orange-500 to-amber-500)
    DICT_ACCENT = #f97316         // Orange-500
    DICT_ACCENT_HOVER = #ea580c   // Orange-600
    DICT_ACCENT_BORDER = #fdba74  // Orange-300
    ORANGE_50 = #fff7ed           // Light orange background
    ORANGE_100 = #ffedd5          // For input selection
    ORANGE_600 = #ea580c          // Darker orange
    ORANGE_800 = #9a3412          // Very dark orange for text
    ORANGE_900 = #7c2d12          // Dark orange for titles
    AMBER_50 = #fffbeb            // Light amber
    AMBER_500 = #f59e0b           // Amber accent

    // ========================================================================
    // Design Tokens - matches website card styling (bg-white rounded-xl shadow-lg)
    // ========================================================================

    // Card base with shadow - matches website "bg-white rounded-xl shadow-lg"
    DictCardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0  // rounded-xl = 12px

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow effect (shadow-lg approximation)
                let shadow_color = vec4(0.0, 0.0, 0.0, 0.08);
                sdf.box(2.0, 4.0, self.rect_size.x - 4.0, self.rect_size.y - 2.0, 12.0);
                sdf.fill(shadow_color);

                // Main card
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0);
                let dark_bg = (SLATE_800);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));

                return sdf.result;
            }
        }
    }

    // Panel for secondary content - matches "bg-gray-50"
    DictPanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light_bg = vec4(0.976, 0.980, 0.984, 1.0); // gray-50
                let dark_bg = (SLATE_700);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));
                return sdf.result;
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

    // Tab Button - matches website TabsTrigger style
    // NOTE: Must be defined before usage in DefinitionsCard, RelatedCard, etc.
    DictTabButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        margin: {right: 4}

        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                if self.active > 0.5 {
                    // Active: orange-100 bg with semibold text
                    let light_active = vec4(1.0, 0.929, 0.835, 1.0); // orange-100
                    let dark_active = (SLATE_700);
                    sdf.fill(mix(light_active, dark_active, self.dark_mode));
                } else if self.hover > 0.5 {
                    // Hover: gray-100
                    let light_hover = vec4(0.953, 0.961, 0.969, 1.0); // gray-100
                    let dark_hover = (SLATE_700);
                    sdf.fill(mix(light_hover, dark_hover, self.dark_mode));
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance active: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                // Active: orange-800 text, Inactive: gray-600 text
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
    // Search Bar Component - matches website search form
    // ========================================================================

    SearchBar = <View> {
        width: Fill, height: Fit
        padding: 0
        flow: Right
        spacing: 8
        align: {y: 0.5}

        // Search input container with border - matches website input style
        search_input_container = <RoundedView> {
            width: Fill, height: 40
            flow: Right
            spacing: 8
            align: {y: 0.5}
            padding: {left: 12, right: 12}
            show_bg: true

            draw_bg: {
                instance dark_mode: 0.0
                instance focus: 0.0
                border_radius: 8.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let light_bg = vec4(1.0, 1.0, 1.0, 1.0);
                    let dark_bg = (SLATE_800);
                    let bg = mix(light_bg, dark_bg, self.dark_mode);

                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill(bg);

                    // Border: gray-300 normally, orange-500 on focus
                    let light_border = vec4(0.835, 0.847, 0.863, 1.0); // gray-300
                    let focus_border = vec4(0.976, 0.451, 0.086, 1.0); // orange-500
                    let dark_border = (SLATE_600);
                    let border_color = mix(
                        mix(light_border, dark_border, self.dark_mode),
                        focus_border,
                        self.focus
                    );
                    sdf.stroke(border_color, 1.0);

                    return sdf.result;
                }
            }

            // Text input
            search_input = <TextInput> {
                width: Fill, height: Fit
                empty_text: "输入英/汉字词..."

                draw_bg: {
                    color: #0000
                }

                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }

                draw_cursor: {
                    color: (DICT_ACCENT)
                }

                draw_selection: {
                    color: (ORANGE_100)
                }
            }
        }

        // Search button - orange bg matching website
        search_btn = <Button> {
            width: Fit, height: 38
            text: "🔍 查询"
            padding: {left: 16, right: 16}

            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    // orange-600 (#ea580c) -> orange-700 (#c2410c) on hover
                    let base = vec4(0.918, 0.345, 0.047, 1.0);   // orange-600
                    let hover = vec4(0.761, 0.255, 0.047, 1.0);  // orange-700
                    let color = mix(base, hover, self.hover);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill(color);
                    return sdf.result;
                }
            }

            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
        }
    }

    // ========================================================================
    // Search History Card
    // ========================================================================

    // History item - matches website history list style
    SearchHistoryItem = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 8, bottom: 8}
        flow: Right
        spacing: 8
        align: {y: 0.5}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Active: orange-100 bg, Hover: gray-100 bg
                let light_normal = vec4(0.0, 0.0, 0.0, 0.0);
                let light_hover = vec4(0.953, 0.961, 0.969, 1.0);  // gray-100
                let light_active = vec4(1.0, 0.929, 0.835, 1.0);   // orange-100
                let dark_normal = vec4(0.0, 0.0, 0.0, 0.0);
                let dark_hover = (SLATE_700);
                let dark_active = (SLATE_600);

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let active_color = mix(light_active, dark_active, self.dark_mode);

                let base = mix(normal, hover_color, self.hover);
                let color = mix(base, active_color, self.active);

                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        // Favorite star button
        favorite_star = <Label> {
            text: "☆"
            draw_text: {
                instance favorited: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    // Yellow-500 when favorited, gray-300 otherwise
                    let unfavorited = vec4(0.835, 0.847, 0.863, 1.0); // gray-300
                    let favorited = vec4(0.961, 0.620, 0.043, 1.0);   // yellow-500
                    return mix(unfavorited, favorited, self.favorited);
                }
            }
        }

        history_word = <Label> {
            text: "serendipity"
            draw_text: {
                instance dark_mode: 0.0
                instance active: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    // Active: orange-800, Normal: gray-700/gray-300
                    let light_normal = vec4(0.263, 0.290, 0.333, 1.0);  // gray-700
                    let light_active = vec4(0.604, 0.176, 0.071, 1.0);   // orange-800
                    let dark_normal = (TEXT_PRIMARY_DARK);
                    let dark_active = vec4(1.0, 0.929, 0.835, 1.0); // orange-100

                    let normal = mix(light_normal, dark_normal, self.dark_mode);
                    let active_color = mix(light_active, dark_active, self.dark_mode);
                    return mix(normal, active_color, self.active);
                }
            }
        }

        <View> { width: Fill }

        // Delete button (hidden, shows on hover)
        delete_btn = <View> {
            width: Fit, height: Fit
            visible: false
            <Label> {
                text: "×"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    color: (SLATE_400)
                }
            }
        }
    }

    SearchHistoryCard = <DictCardBase> {
        width: Fill, height: Fill
        padding: 16
        flow: Down
        spacing: 12

        // Header - matches website history panel
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            spacing: 8

            // History icon
            <Label> {
                text: "🕐"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                }
            }

            <Label> {
                text: "搜索历史"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        let light = vec4(0.263, 0.290, 0.333, 1.0); // gray-700
                        let dark = (TEXT_PRIMARY_DARK);
                        return mix(light, dark, self.dark_mode);
                    }
                }
            }

            <View> { width: Fill }

            clear_history_btn = <Button> {
                width: Fit, height: Fit
                text: "🗑️"
                padding: {left: 6, right: 6, top: 4, bottom: 4}

                draw_bg: {
                    instance hover: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        // Transparent, red-50 on hover
                        let normal = vec4(0.0, 0.0, 0.0, 0.0);
                        let hover = vec4(0.996, 0.949, 0.949, 1.0); // red-50
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        sdf.fill(mix(normal, hover, self.hover));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
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
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        align: {y: 0.5}

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Orange accent for active/hover states
                let bg = mix((WHITE), (SLATE_700), self.dark_mode);
                let active_bg = mix((SLATE_100), (SLATE_600), self.dark_mode);
                let hover_bg = mix(bg, active_bg, self.active);
                let final_bg = mix(hover_bg, active_bg, self.hover * 0.5);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(final_bg);
                // Left border indicator when active
                if self.active > 0.5 {
                    sdf.box(0., 4.0, 3.0, self.rect_size.y - 8.0, 1.5);
                    sdf.fill((DICT_ACCENT));
                }
                return sdf.result;
            }
        }

        anchor_text = <Label> {
            text: "词典释义"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
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
                    color: (DICT_ACCENT)
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
    // Badge Components - matches website badge styling
    // ========================================================================

    // Difficulty/frequency/type badge
    WordBadge = <RoundedView> {
        width: Fit, height: 22
        padding: {left: 8, right: 8}
        show_bg: true
        draw_bg: {
            instance badge_type: 0.0  // 0=orange, 1=amber, 2=yellow
            border_radius: 11.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let orange_100 = vec4(1.0, 0.929, 0.835, 1.0);
                let amber_100 = vec4(0.996, 0.941, 0.780, 1.0);
                let yellow_100 = vec4(0.996, 0.988, 0.839, 1.0);
                let bg = vec4(0.0);
                if self.badge_type < 0.5 { bg = orange_100; }
                else if self.badge_type < 1.5 { bg = amber_100; }
                else { bg = yellow_100; }
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 11.0);
                sdf.fill(bg);
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}

        badge_label = <Label> {
            draw_text: {
                instance badge_type: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    let orange_800 = vec4(0.604, 0.176, 0.071, 1.0);
                    let amber_800 = vec4(0.569, 0.365, 0.020, 1.0);
                    let yellow_800 = vec4(0.520, 0.420, 0.012, 1.0);
                    if self.badge_type < 0.5 { return orange_800; }
                    else if self.badge_type < 1.5 { return amber_800; }
                    else { return yellow_800; }
                }
            }
        }
    }

    // Audio play button
    AudioPlayButton = <View> {
        width: 24, height: 24
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let normal = vec4(0.0, 0.0, 0.0, 0.0);
                let hover_bg = vec4(1.0, 0.929, 0.835, 1.0); // orange-100
                sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 11.0);
                sdf.fill(mix(normal, hover_bg, self.hover));
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand

        <Label> {
            text: "🔊"
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
            }
        }
    }

    // Word form tag
    WordFormTag = <RoundedView> {
        width: Fit, height: 22
        padding: {left: 8, right: 8}
        show_bg: true
        draw_bg: {
            border_radius: 11.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 11.0);
                sdf.fill(vec4(1.0, 0.929, 0.835, 1.0)); // orange-100
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}

        form_label = <Label> {
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                color: #9a3412 // orange-800
            }
        }
    }

    // Dictionary source badge
    DictSourceBadge = <RoundedView> {
        width: Fit, height: 22
        padding: {left: 8, right: 8}
        show_bg: true
        draw_bg: {
            border_radius: 11.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 11.0);
                sdf.fill(vec4(0.996, 0.941, 0.780, 1.0)); // amber-100
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}

        source_label = <Label> {
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                color: (AMBER_500)
            }
        }
    }

    // ========================================================================
    // Word Detail Card - matches website WordResult component
    // ========================================================================

    WordDetailCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 12
        visible: false

        // Header row: word + favorite + badges
        word_header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            spacing: 8

            // Word and favorite
            word_title_row = <View> {
                width: Fit, height: Fit
                flow: Right
                align: {y: 0.5}
                spacing: 8

                word_title = <Label> {
                    text: "hello"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 28.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                favorite_btn = <View> {
                    width: 28, height: 28
                    cursor: Hand
                    align: {x: 0.5, y: 0.5}

                    favorite_star = <Label> {
                        text: "☆"
                        draw_text: {
                            instance favorited: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 20.0 }
                            fn get_color(self) -> vec4 {
                                let unfav = vec4(0.835, 0.847, 0.863, 1.0); // gray-300
                                let fav = vec4(0.961, 0.620, 0.043, 1.0);   // yellow-500
                                return mix(unfav, fav, self.favorited);
                            }
                        }
                    }
                }
            }

            <View> { width: Fill }

            // Badges row
            badges_row = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 6

                difficulty_badge = <WordBadge> {
                    visible: false
                    badge_label = { text: "难度: 3" }
                }

                frequency_badge = <WordBadge> {
                    visible: false
                    draw_bg: { badge_type: 1.0 }
                    badge_label = { draw_text: { badge_type: 1.0 }, text: "频率: 高" }
                }

                word_type_badge = <WordBadge> {
                    visible: false
                    draw_bg: { badge_type: 2.0 }
                    badge_label = { draw_text: { badge_type: 2.0 }, text: "名词" }
                }
            }
        }

        // Pronunciations row
        pronunciations_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 16
            visible: false

            uk_pron = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 4
                align: {y: 0.5}
                visible: false

                <Label> {
                    text: "英"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        color: (SLATE_500)
                    }
                }

                uk_ipa = <Label> {
                    text: "/həˈləʊ/"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        color: (SLATE_600)
                    }
                }

                uk_audio_btn = <AudioPlayButton> {}
            }

            us_pron = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 4
                align: {y: 0.5}
                visible: false

                <Label> {
                    text: "美"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        color: (SLATE_500)
                    }
                }

                us_ipa = <Label> {
                    text: "/həˈloʊ/"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        color: (SLATE_600)
                    }
                }

                us_audio_btn = <AudioPlayButton> {}
            }
        }

        // Word forms row
        word_forms_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 6
            visible: false

            form_1 = <WordFormTag> { visible: false, form_label = { text: "plural: hellos" } }
            form_2 = <WordFormTag> { visible: false, form_label = { text: "past: -ed" } }
            form_3 = <WordFormTag> { visible: false, form_label = { text: "gerund: -ing" } }
        }

        // Dictionary sources row
        dict_sources_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 6
            visible: false

            source_1 = <DictSourceBadge> { visible: false, source_label = { text: "牛津" } }
            source_2 = <DictSourceBadge> { visible: false, source_label = { text: "柯林斯" } }
            source_3 = <DictSourceBadge> { visible: false, source_label = { text: "朗文" } }
        }
    }

    // ========================================================================
    // Definitions Section - matches website definitions tabs
    // ========================================================================

    // Definition item with left border accent
    DefinitionItem = <View> {
        width: Fill, height: Fit
        padding: {left: 12}
        flow: Down
        spacing: 4
        show_bg: true
        draw_bg: {
            instance lang: 0.0  // 0=Chinese (orange), 1=English (blue)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Left border only
                let orange = vec4(0.976, 0.451, 0.086, 1.0); // orange-500
                let blue = vec4(0.231, 0.510, 0.965, 1.0);   // blue-500
                let border_color = mix(orange, blue, self.lang);
                sdf.box(0., 0., 2.0, self.rect_size.y, 1.0);
                sdf.fill(border_color);
                return sdf.result;
            }
        }

        // Tags row: POS, register, region
        tags_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 6
            align: {y: 0.5}

            pos_tag = <RoundedView> {
                width: Fit, height: 20
                padding: {left: 6, right: 6}
                visible: false
                show_bg: true
                draw_bg: {
                    instance lang: 0.0
                    border_radius: 4.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let orange = vec4(0.992, 0.796, 0.545, 1.0); // orange-200
                        let blue = vec4(0.749, 0.859, 0.992, 1.0);   // blue-200
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        sdf.fill(mix(orange, blue, self.lang));
                        return sdf.result;
                    }
                }
                align: {x: 0.5, y: 0.5}

                pos_label = <Label> {
                    draw_text: {
                        instance lang: 0.0
                        text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            let orange = vec4(0.604, 0.176, 0.071, 1.0); // orange-800
                            let blue = vec4(0.118, 0.306, 0.627, 1.0);   // blue-800
                            return mix(orange, blue, self.lang);
                        }
                    }
                }
            }

            register_label = <View> {
                width: Fit, height: Fit
                visible: false
                <Label> {
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        color: (SLATE_500)
                    }
                }
            }

            region_label = <View> {
                width: Fit, height: Fit
                visible: false
                <Label> {
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        color: (SLATE_500)
                    }
                }
            }
        }

        // Definition text
        definition_text = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        // Usage notes
        usage_notes = <View> {
            width: Fill, height: Fit
            visible: false
            <Label> {
                width: Fill
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    color: (SLATE_500)
                }
            }
        }
    }

    // Definitions card with tabs
    DefinitionsCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 12
        visible: false

        // Tab buttons
        def_tabs_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 0
            margin: {bottom: 8}

            def_tab_bg = <RoundedView> {
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
                        let light = vec4(0.961, 0.953, 0.945, 1.0);
                        let dark = (SLATE_800);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill(mix(light, dark, self.dark_mode));
                        return sdf.result;
                    }
                }

                zh_def_tab = <DictTabButton> {
                    text: "中文释义"
                    draw_bg: { active: 1.0 }
                    draw_text: { active: 1.0 }
                }

                en_def_tab = <DictTabButton> {
                    text: "英文释义"
                }
            }
        }

        // Chinese definitions list
        zh_definitions = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 12

            zh_def_1 = <DefinitionItem> { visible: false }
            zh_def_2 = <DefinitionItem> { visible: false }
            zh_def_3 = <DefinitionItem> { visible: false }
        }

        // English definitions list (hidden by default)
        en_definitions = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 12
            visible: false

            en_def_1 = <DefinitionItem> { draw_bg: { lang: 1.0 }, visible: false }
            en_def_2 = <DefinitionItem> { draw_bg: { lang: 1.0 }, visible: false }
            en_def_3 = <DefinitionItem> { draw_bg: { lang: 1.0 }, visible: false }
        }
    }

    // ========================================================================
    // Example Sentences Section
    // ========================================================================

    // Single sentence item
    SentenceItem = <RoundedView> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 4
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

        sentence_text = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        sentence_source = <View> {
            width: Fit, height: Fit
            visible: false
            <Label> {
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    color: (SLATE_500)
                }
            }
        }
    }

    // Sentences card
    SentencesCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 12
        visible: false

        // Section title
        <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            <Label> {
                text: "使用例句"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Sentences list
        sentences_list = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            sent_1 = <SentenceItem> { visible: false }
            sent_2 = <SentenceItem> { visible: false }
            sent_3 = <SentenceItem> { visible: false }
            sent_4 = <SentenceItem> { visible: false }
            sent_5 = <SentenceItem> { visible: false }
        }

        // Show more button
        show_more_btn = <Button> {
            width: Fill, height: Fit
            visible: false
            padding: {top: 8, bottom: 8}
            text: "查看更多"

            draw_bg: {
                fn pixel(self) -> vec4 { return #0000; }
            }

            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (DICT_ACCENT)
            }
        }
    }

    // ========================================================================
    // Related Words Section
    // ========================================================================

    // Related word tag (clickable)
    RelatedWordTag = <RoundedView> {
        width: Fit, height: 28
        padding: {left: 12, right: 12}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance tag_type: 0.0  // 0=synonym (green), 1=antonym (red), 2=related (blue)
            instance hover: 0.0
            border_radius: 14.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let green_100 = vec4(0.863, 0.988, 0.906, 1.0);
                let green_200 = vec4(0.733, 0.969, 0.816, 1.0);
                let red_100 = vec4(0.996, 0.886, 0.886, 1.0);
                let red_200 = vec4(0.996, 0.792, 0.792, 1.0);
                let blue_100 = vec4(0.859, 0.922, 0.996, 1.0);
                let blue_200 = vec4(0.749, 0.859, 0.992, 1.0);

                let normal = vec4(0.0);
                let hover_color = vec4(0.0);
                if self.tag_type < 0.5 {
                    normal = green_100; hover_color = green_200;
                } else if self.tag_type < 1.5 {
                    normal = red_100; hover_color = red_200;
                } else {
                    normal = blue_100; hover_color = blue_200;
                }
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 14.0);
                sdf.fill(mix(normal, hover_color, self.hover));
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}

        tag_label = <Label> {
            draw_text: {
                instance tag_type: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    let green_800 = vec4(0.133, 0.533, 0.267, 1.0);
                    let red_800 = vec4(0.600, 0.118, 0.118, 1.0);
                    let blue_800 = vec4(0.118, 0.306, 0.627, 1.0);
                    if self.tag_type < 0.5 { return green_800; }
                    else if self.tag_type < 1.5 { return red_800; }
                    else { return blue_800; }
                }
            }
        }
    }

    // Related words card with tabs
    RelatedCard = <DictCardBase> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 12
        visible: false

        // Tab buttons
        rel_tabs_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 0
            margin: {bottom: 8}

            rel_tab_bg = <RoundedView> {
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
                        let light = vec4(0.961, 0.953, 0.945, 1.0);
                        let dark = (SLATE_800);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill(mix(light, dark, self.dark_mode));
                        return sdf.result;
                    }
                }

                synonyms_tab = <DictTabButton> {
                    text: "近义词"
                    draw_bg: { active: 1.0 }
                    draw_text: { active: 1.0 }
                }

                antonyms_tab = <DictTabButton> {
                    text: "反义词"
                }

                related_tab = <DictTabButton> {
                    text: "相关词"
                }
            }
        }

        // Synonyms content
        synonyms_content = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8

            syn_1 = <RelatedWordTag> { visible: false }
            syn_2 = <RelatedWordTag> { visible: false }
            syn_3 = <RelatedWordTag> { visible: false }
            syn_4 = <RelatedWordTag> { visible: false }
        }

        // Antonyms content (hidden)
        antonyms_content = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8
            visible: false

            ant_1 = <RelatedWordTag> { draw_bg: { tag_type: 1.0 }, visible: false, tag_label = { draw_text: { tag_type: 1.0 } } }
            ant_2 = <RelatedWordTag> { draw_bg: { tag_type: 1.0 }, visible: false, tag_label = { draw_text: { tag_type: 1.0 } } }
        }

        // Related words content (hidden)
        related_content = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 8
            visible: false

            rel_1 = <RelatedWordTag> { draw_bg: { tag_type: 2.0 }, visible: false, tag_label = { draw_text: { tag_type: 2.0 } } }
            rel_2 = <RelatedWordTag> { draw_bg: { tag_type: 2.0 }, visible: false, tag_label = { draw_text: { tag_type: 2.0 } } }
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
    // Translation Options - matches website translation options panel
    // ========================================================================

    // Tone option radio button
    ToneOption = <View> {
        width: Fill, height: Fit
        padding: {left: 8, right: 8, top: 6, bottom: 6}
        flow: Right
        align: {y: 0.5}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let orange_200 = vec4(0.992, 0.796, 0.545, 1.0);
                let gray_50 = vec4(0.976, 0.980, 0.984, 1.0);
                let transparent = vec4(0.0, 0.0, 0.0, 0.0);

                // Selected: orange-50 with border, Hover: gray-50, Normal: transparent
                let normal = mix(transparent, gray_50, self.hover);
                let selected_bg = orange_50;
                let bg = mix(normal, selected_bg, self.selected);

                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(bg);
                if self.selected > 0.5 {
                    sdf.stroke(orange_200, 1.0);
                }
                return sdf.result;
            }
        }

        // Radio indicator
        radio_indicator = <View> {
            width: 14, height: 14
            margin: {right: 8}
            show_bg: true
            draw_bg: {
                instance selected: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    // Outer circle
                    sdf.circle(c.x, c.y, 6.0);
                    let orange_500 = vec4(0.976, 0.451, 0.086, 1.0);
                    let gray_300 = vec4(0.835, 0.847, 0.863, 1.0);
                    sdf.fill(mix(gray_300, orange_500, self.selected));
                    // Inner circle (white)
                    sdf.circle(c.x, c.y, 4.0);
                    sdf.fill(#fff);
                    // Center dot when selected
                    if self.selected > 0.5 {
                        sdf.circle(c.x, c.y, 2.5);
                        sdf.fill(orange_500);
                    }
                    return sdf.result;
                }
            }
        }

        option_text = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            option_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <View> { width: Fill }

            option_desc = <Label> {
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    color: (SLATE_400)
                }
            }
        }
    }

    // Checkbox option
    CheckboxOption = <View> {
        width: Fill, height: Fit
        padding: {left: 8, right: 8, top: 8, bottom: 8}
        flow: Right
        align: {y: 0.5}
        cursor: Hand

        checkbox_indicator = <View> {
            width: 16, height: 16
            margin: {right: 8}
            show_bg: true
            draw_bg: {
                instance checked: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let orange_500 = vec4(0.976, 0.451, 0.086, 1.0);
                    let gray_200 = vec4(0.898, 0.906, 0.922, 1.0);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                    sdf.fill(mix(#fff, orange_500, self.checked));
                    sdf.stroke(mix(gray_200, orange_500, self.checked), 1.0);
                    // Checkmark when checked
                    if self.checked > 0.5 {
                        // Simple checkmark using lines
                        sdf.move_to(4.0, 8.0);
                        sdf.line_to(7.0, 11.0);
                        sdf.line_to(12.0, 5.0);
                        sdf.stroke(#fff, 1.5);
                    }
                    return sdf.result;
                }
            }
        }

        checkbox_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 2

            checkbox_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            checkbox_desc = <Label> {
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    color: (SLATE_400)
                }
            }
        }
    }

    // Translation options panel
    TranslationOptionsPanel = <DictCardBase> {
        width: 260, height: Fit
        padding: 16
        flow: Down
        spacing: 12

        // Tone selection section
        tone_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 6

            <Label> {
                text: "语言风格"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    color: (SLATE_600)
                }
            }

            tone_options = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 4

                tone_original = <ToneOption> {
                    draw_bg: { selected: 1.0 }
                    radio_indicator = { draw_bg: { selected: 1.0 } }
                    option_text = {
                        option_label = { text: "保持原样" }
                        option_desc = { text: "忠实原文" }
                    }
                }

                tone_casual = <ToneOption> {
                    option_text = {
                        option_label = { text: "日常口语" }
                        option_desc = { text: "轻松自然" }
                    }
                }

                tone_business = <ToneOption> {
                    option_text = {
                        option_label = { text: "商务职场" }
                        option_desc = { text: "正式专业" }
                    }
                }

                tone_academic = <ToneOption> {
                    option_text = {
                        option_label = { text: "学术正式" }
                        option_desc = { text: "严谨规范" }
                    }
                }

                tone_literary = <ToneOption> {
                    option_text = {
                        option_label = { text: "文学优美" }
                        option_desc = { text: "富有文采" }
                    }
                }

                tone_simple = <ToneOption> {
                    option_text = {
                        option_label = { text: "简单易懂" }
                        option_desc = { text: "通俗直白" }
                    }
                }
            }
        }

        // Divider
        <View> {
            width: Fill, height: 1
            margin: {top: 4, bottom: 4}
            show_bg: true
            draw_bg: { color: (DIVIDER) }
        }

        // Format section
        format_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 6

            <Label> {
                text: "格式处理"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    color: (SLATE_600)
                }
            }

            format_options = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 4

                format_preserve = <ToneOption> {
                    draw_bg: { selected: 1.0 }
                    radio_indicator = { draw_bg: { selected: 1.0 } }
                    option_text = {
                        option_label = { text: "保留格式" }
                        option_desc = { text: "保持原有结构" }
                    }
                }

                format_reformat = <ToneOption> {
                    option_text = {
                        option_label = { text: "重新排版" }
                        option_desc = { text: "优化可读性" }
                    }
                }
            }
        }

        // Divider
        <View> {
            width: Fill, height: 1
            margin: {top: 4, bottom: 4}
            show_bg: true
            draw_bg: { color: (DIVIDER) }
        }

        // Auto-correct option
        auto_correct_option = <CheckboxOption> {
            checkbox_content = {
                checkbox_label = { text: "自动纠错" }
                checkbox_desc = { text: "修正原文语法拼写错误" }
            }
        }
    }

    // ========================================================================
    // Translation Panel
    // ========================================================================

    TranslationPanel = <View> {
        width: Fill, height: Fill
        flow: Right
        spacing: 16

        // Left column - input/output cards
        translate_main = <View> {
            width: Fill, height: Fill
            flow: Down
            spacing: 16

            // Source text card
            source_card = <DictCardBase> {
                width: Fill, height: Fit
                padding: 20
                flow: Down
                spacing: 12

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    <DictSectionTitle> { text: "输入文本" }

                    auto_detect_badge = <RoundedView> {
                        width: Fit, height: 20
                        margin: {left: 8}
                        padding: {left: 8, right: 8}
                        show_bg: true
                        draw_bg: {
                            border_radius: 4.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.953, 0.961, 0.969, 1.0)); // gray-100
                                return sdf.result;
                            }
                        }
                        align: {x: 0.5, y: 0.5}

                        <Label> {
                            text: "自动检测语言"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (SLATE_500)
                            }
                        }
                    }

                    <View> { width: Fill }
                    char_count = <DictMutedText> { text: "0/5000" }
                }

                source_input = <TextInput> {
                    width: Fill, height: 120
                    padding: {left: 16, right: 16, top: 12, bottom: 12}
                    text: "输入中文或英文，自动检测并翻译..."

                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                            let bg = mix((SLATE_50), (SLATE_700), self.dark_mode);
                            sdf.fill(bg);
                            return sdf.result;
                        }
                    }

                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                translate_btn_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    hint_text = <DictMutedText> { text: "支持中英文互译，AI 自动识别语言" }
                    <View> { width: Fill }

                    translate_btn = <Button> {
                        width: Fit, height: 40
                        text: "🌐 翻译"
                        padding: {left: 20, right: 20}

                        draw_bg: {
                            instance hover: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let base = vec4(0.976, 0.451, 0.086, 1.0);   // orange-500
                                let hover = vec4(0.918, 0.345, 0.047, 1.0); // orange-600
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                sdf.fill(mix(base, hover, self.hover));
                                return sdf.result;
                            }
                        }

                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            color: (WHITE)
                        }
                    }
                }
            }

            // Result text card
            result_card = <DictCardBase> {
                width: Fill, height: Fit
                padding: 20
                flow: Down
                spacing: 12

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}

                    <DictSectionTitle> { text: "翻译结果" }

                    detected_lang_badge = <RoundedView> {
                        width: Fit, height: 20
                        margin: {left: 8}
                        padding: {left: 8, right: 8}
                        visible: false
                        show_bg: true
                        draw_bg: {
                            border_radius: 4.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(vec4(1.0, 0.969, 0.929, 1.0)); // orange-50
                                return sdf.result;
                            }
                        }
                        align: {x: 0.5, y: 0.5}

                        detected_lang_label = <Label> {
                            text: "中文 → 英文"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (DICT_ACCENT)
                            }
                        }
                    }

                    <View> { width: Fill }
                    copy_btn = <Button> {
                        width: Fit, height: Fit
                        text: "📋 复制"
                        padding: {left: 12, right: 12, top: 6, bottom: 6}

                        draw_bg: {
                            instance hover: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let normal = vec4(0.0, 0.0, 0.0, 0.0);
                                let hover = vec4(1.0, 0.969, 0.929, 1.0); // orange-50
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(mix(normal, hover, self.hover));
                                return sdf.result;
                            }
                        }

                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            color: (DICT_ACCENT)
                        }
                    }
                }

                result_text = <Label> {
                    width: Fill
                    text: "翻译结果将显示在这里"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }
            }

            // Tips
            tips_card = <RoundedView> {
                width: Fill, height: Fit
                padding: 16
                show_bg: true
                draw_bg: {
                    border_radius: 8.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill(#fffbeb); // amber-50
                        sdf.stroke(#fde68a, 1.0); // amber-200
                        return sdf.result;
                    }
                }

                tips_label = <Label> {
                    width: Fill
                    text: "💡 提示：翻译功能使用 AI 模型，自动检测输入语言并翻译为对应语言。"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        color: #924010
                    }
                }
            }
        }

        // Right column - options panel
        translate_options = <TranslationOptionsPanel> {}
    }

    // ========================================================================
    // Main Dictionary Screen - matches website DictPage.tsx
    // ========================================================================

    pub DictionaryScreen = {{DictionaryScreen}} {
        width: Fill, height: Fill
        flow: Down

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Gradient matching website: from-orange-50 via-amber-50 to-yellow-50
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);  // #fff7ed
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);   // #fffbeb
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0); // #fefce8
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);   // gray-900

                // Create gradient based on position
                let t = self.pos.x + self.pos.y * 0.5;
                let light_color = vec4(0.0);
                if t < 0.5 {
                    light_color = mix(orange_50, amber_50, t * 2.0);
                } else {
                    light_color = mix(amber_50, yellow_50, (t - 0.5) * 2.0);
                }
                return mix(light_color, dark_bg, self.dark_mode);
            }
        }

        // Page header with title and tabs - matches website layout
        page_header = <View> {
            width: Fill, height: Fit
            flow: Down
            padding: {left: 16, right: 16, top: 16, bottom: 12}
            spacing: 12

            // Title row - centered, text-2xl font-bold text-orange-900
            title_row = <View> {
                width: Fill, height: Fit
                align: {x: 0.5}

                page_title = <Label> {
                    text: "📖 词典翻译"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 20.0 }
                        fn get_color(self) -> vec4 {
                            // orange-900 in light mode, white in dark mode
                            let light = vec4(0.486, 0.176, 0.071, 1.0); // #7c2d12
                            let dark = vec4(1.0, 1.0, 1.0, 1.0);
                            return mix(light, dark, self.dark_mode);
                        }
                    }
                }
            }

            // Tabs row - centered, matches website TabsList
            tabs_container = <View> {
                width: Fill, height: Fit
                align: {x: 0.5}

                tabs_bg = <RoundedView> {
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
                            // Muted background like website TabsList (bg-muted)
                            let light = vec4(0.961, 0.953, 0.945, 1.0); // gray-100/muted
                            let dark = (SLATE_800);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                            sdf.fill(mix(light, dark, self.dark_mode));
                            return sdf.result;
                        }
                    }

                    lookup_tab = <DictTabButton> {
                        text: "📚 词典查询"
                        draw_bg: { active: 1.0 }
                        draw_text: { active: 1.0 }
                    }

                    translate_tab = <DictTabButton> {
                        text: "🌐 内容翻译"
                    }
                }
            }
        }

        // Main content area
        main_content = <View> {
            width: Fill, height: Fill
            flow: Right

            // Left column - Navigation anchors (lookup mode only)
            nav_sidebar = <NavSidebar> {}

            // Center column - Search and Results / Translation
            center_column = <View> {
                width: Fill, height: Fill
                padding: {top: 0, bottom: 20, left: 24, right: 24}
                flow: Down
                spacing: 16

                // Lookup content (visible by default)
                lookup_content = <View> {
                    width: Fill, height: Fill
                    flow: Down
                    spacing: 16

                    // Search bar row
                    search_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        search_bar = <SearchBar> {
                            width: Fill
                        }
                    }

                    // Word detail scroll area
                    word_detail_scroll = <ScrollYView> {
                        width: Fill, height: Fill

                        word_detail_content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 16

                            // Word detail card (header with badges, pronunciations, forms)
                            word_detail = <WordDetailCard> {}

                            // Definitions section with tabs
                            definitions_card = <DefinitionsCard> {}

                            // Example sentences section
                            sentences_card = <SentencesCard> {}

                            // Related words section with tabs
                            related_card = <RelatedCard> {}

                            // Empty state (shown when no word selected)
                            empty_state = <View> {
                                width: Fill, height: 300
                                align: {x: 0.5, y: 0.5}

                                content = <View> {
                                    width: Fit, height: Fit
                                    flow: Down
                                    spacing: 12
                                    align: {x: 0.5}

                                    <Label> {
                                        text: "📖"
                                        draw_text: {
                                            text_style: <FONT_REGULAR>{ font_size: 48.0 }
                                            color: (SLATE_300)
                                        }
                                    }

                                    <Label> {
                                        text: "输入单词开始查询"
                                        draw_text: {
                                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                            color: (SLATE_400)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Translation content (hidden by default)
                translation_content = <TranslationPanel> {
                    visible: false
                }
            }

            // Right column - Search History (lookup mode only)
            right_column = <View> {
                width: 220, height: Fill
                padding: {top: 0, bottom: 24, left: 16, right: 16}
                flow: Down

                search_history = <SearchHistoryCard> {}
            }
        }
    }
}

use crate::dict_api::{get_dict_api, Word, WordQueryResponse, SearchHistoryEntry};
use crossbeam_channel::{Receiver, Sender, bounded};

/// Active tab for dictionary screen
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DictTab {
    #[default]
    Lookup,
    Translate,
}

/// DictionaryScreen widget
#[derive(Live, LiveHook, Widget)]
pub struct DictionaryScreen {
    #[deref]
    view: View,

    #[rust]
    active_tab: DictTab,

    #[rust]
    search_query: String,

    #[rust]
    search_results: Vec<Word>,

    #[rust]
    selected_word: Option<WordQueryResponse>,

    #[rust]
    is_searching: bool,

    #[rust]
    is_prefix_searching: bool,

    #[rust]
    lookup_result_receiver: Option<Receiver<Result<WordQueryResponse, String>>>,

    #[rust]
    lookup_result_sender: Option<Sender<Result<WordQueryResponse, String>>>,

    #[rust]
    search_result_receiver: Option<Receiver<Result<Vec<Word>, String>>>,

    #[rust]
    search_result_sender: Option<Sender<Result<Vec<Word>, String>>>,

    #[rust]
    translation_source: String,

    #[rust]
    translation_result: String,
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
        // Handle tab switching
        if self.button(ids!(page_header.tabs_container.tabs_bg.lookup_tab)).clicked(actions) {
            self.switch_tab(cx, DictTab::Lookup);
        }
        if self.button(ids!(page_header.tabs_container.tabs_bg.translate_tab)).clicked(actions) {
            self.switch_tab(cx, DictTab::Translate);
        }

        // Handle search input changes (lookup tab)
        if let Some(text) = self
            .text_input(ids!(main_content.center_column.lookup_content.search_row.search_bar.search_input_container.search_input))
            .changed(actions)
        {
            self.search_query = text.clone();
            ::log::info!("Search query: {}", self.search_query);
            // Trigger search as user types (for autocomplete)
            if !self.search_query.trim().is_empty() && self.search_query.len() >= 2 {
                self.perform_search(cx, self.search_query.clone());
            }
        }

        // Handle search button click (lookup tab)
        if self.button(ids!(main_content.center_column.lookup_content.search_row.search_bar.search_btn)).clicked(actions) {
            if !self.search_query.trim().is_empty() {
                self.perform_lookup(cx, self.search_query.clone());
            }
        }

        // Handle clear history
        if self
            .button(ids!(main_content.right_column.search_history.clear_history_btn))
            .clicked(actions)
        {
            self.clear_search_history(cx);
        }

        // Handle translate button click (translate tab)
        if self.button(ids!(main_content.center_column.translation_content.source_card.translate_btn_row.translate_btn)).clicked(actions) {
            // TODO: Implement translation API call
            ::log::info!("Translate button clicked");
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

        // Check for prefix search results from channel
        if let Some(ref receiver) = self.search_result_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.is_prefix_searching = false;
                match result {
                    Ok(words) => {
                        ::log::info!("Search found {} results", words.len());
                        self.search_results = words.clone();
                        self.update_search_results_display(cx, &words);
                    }
                    Err(e) => {
                        ::log::error!("Search error: {}", e);
                    }
                }
            }
        }
    }
}

impl DictionaryScreen {
    /// Switch between lookup and translate tabs
    fn switch_tab(&mut self, cx: &mut Cx, tab: DictTab) {
        if self.active_tab == tab {
            return;
        }
        self.active_tab = tab;

        let (lookup_active, translate_active) = match tab {
            DictTab::Lookup => (1.0, 0.0),
            DictTab::Translate => (0.0, 1.0),
        };

        // Update tab button styles
        self.view.button(ids!(page_header.tabs_container.tabs_bg.lookup_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (lookup_active) }
                draw_text: { active: (lookup_active) }
            },
        );
        self.view.button(ids!(page_header.tabs_container.tabs_bg.translate_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (translate_active) }
                draw_text: { active: (translate_active) }
            },
        );

        // Toggle content visibility
        let show_lookup = tab == DictTab::Lookup;
        self.view.view(ids!(main_content.center_column.lookup_content))
            .set_visible(cx, show_lookup);
        self.view.view(ids!(main_content.center_column.translation_content))
            .set_visible(cx, !show_lookup);

        // Toggle sidebar visibility (only show in lookup mode)
        self.view.view(ids!(main_content.nav_sidebar))
            .set_visible(cx, show_lookup);
        self.view.view(ids!(main_content.right_column))
            .set_visible(cx, show_lookup);

        self.view.redraw(cx);
    }

    /// Apply dark mode to all components
    pub fn apply_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }

    /// Perform prefix search (for autocomplete as user types)
    fn perform_search(&mut self, _cx: &mut Cx, query: String) {
        if self.is_prefix_searching {
            return;
        }
        self.is_prefix_searching = true;

        // Create a channel for communication
        let (sender, receiver) = bounded(1);
        self.search_result_sender = Some(sender.clone());
        self.search_result_receiver = Some(receiver);

        // Get the API client
        let api = match get_dict_api() {
            Some(api) => api.read().unwrap().clone(),
            None => {
                ::log::error!("Dictionary API not initialized");
                self.is_prefix_searching = false;
                return;
            }
        };

        // Spawn async search task
        let query_clone = query.clone();
        let sender_clone = sender.clone();
        let api_clone = api.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match api_clone.search(&query_clone, Some(10)).await {
                    Ok(results) => {
                        let _ = sender_clone.send(Ok(results));
                    }
                    Err(e) => {
                        let _ = sender_clone.send(Err(e));
                    }
                }
            });
        });
    }

    /// Update the UI with search results (multiple word matches)
    /// For now, just stores the results - the main display shows full word detail on lookup
    fn update_search_results_display(&mut self, _cx: &mut Cx, results: &[Word]) {
        if results.is_empty() {
            ::log::info!("No search results found");
            return;
        }

        // Store results for potential autocomplete feature
        // Currently, the UI shows full word detail on explicit lookup via search button
        ::log::info!("Search found {} results, first: {}", results.len(), results[0].word);
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

    /// Update the UI with lookup result - populates all the new word detail components
    fn update_result_display(&mut self, cx: &mut Cx, result: &WordQueryResponse) {
        let word = &result.word;

        // Hide empty state, show word detail card
        self.view.view(ids!(main_content.center_column.lookup_content.word_detail_scroll.word_detail_content.empty_state))
            .set_visible(cx, false);

        // =================================================================
        // Word Detail Card
        // =================================================================
        let word_detail = self.view.view(ids!(main_content.center_column.lookup_content.word_detail_scroll.word_detail_content.word_detail));
        word_detail.set_visible(cx, true);

        // Set word title
        word_detail.label(ids!(word_header.word_title_row.word_title))
            .set_text(cx, &word.word);

        // Set badges
        if let Some(difficulty) = word.difficulty {
            let badge = word_detail.view(ids!(word_header.badges_row.difficulty_badge));
            badge.set_visible(cx, true);
            badge.label(ids!(badge_label)).set_text(cx, &format!("难度: {}", difficulty));
        }

        if let Some(frequency) = word.frequency {
            let freq_text = match frequency {
                1..=3 => "低",
                4..=6 => "中",
                _ => "高",
            };
            let badge = word_detail.view(ids!(word_header.badges_row.frequency_badge));
            badge.set_visible(cx, true);
            badge.label(ids!(badge_label)).set_text(cx, &format!("频率: {}", freq_text));
        }

        if let Some(ref word_type) = word.word_type {
            let badge = word_detail.view(ids!(word_header.badges_row.word_type_badge));
            badge.set_visible(cx, true);
            badge.label(ids!(badge_label)).set_text(cx, word_type);
        }

        // Set pronunciations from pronunciations array
        let mut has_pron = false;
        // Find UK and US pronunciations
        let uk_prons: Vec<_> = result.pronunciations.iter()
            .filter(|p| p.dialect.as_deref() == Some("uk") || p.dialect.as_deref() == Some("UK"))
            .collect();
        let us_prons: Vec<_> = result.pronunciations.iter()
            .filter(|p| p.dialect.as_deref() == Some("us") || p.dialect.as_deref() == Some("US"))
            .collect();
        // If no dialect specified, use the first pronunciation for both
        let default_pron = result.pronunciations.first();

        if let Some(pron) = uk_prons.first().or(default_pron.as_ref()) {
            let uk_pron = word_detail.view(ids!(pronunciations_row.uk_pron));
            uk_pron.set_visible(cx, true);
            uk_pron.label(ids!(uk_ipa)).set_text(cx, &format!("/{}/", pron.ipa));
            has_pron = true;
        }
        if let Some(pron) = us_prons.first().or(if uk_prons.is_empty() { default_pron.as_ref() } else { None }) {
            let us_pron = word_detail.view(ids!(pronunciations_row.us_pron));
            us_pron.set_visible(cx, true);
            us_pron.label(ids!(us_ipa)).set_text(cx, &format!("/{}/", pron.ipa));
            has_pron = true;
        }
        if has_pron {
            word_detail.view(ids!(pronunciations_row)).set_visible(cx, true);
        }

        // Set word forms
        if !result.forms.is_empty() {
            word_detail.view(ids!(word_forms_row)).set_visible(cx, true);
            let form_ids = [
                ids!(word_forms_row.form_1),
                ids!(word_forms_row.form_2),
                ids!(word_forms_row.form_3),
            ];
            for (i, form) in result.forms.iter().take(3).enumerate() {
                let form_view = word_detail.view(form_ids[i]);
                form_view.set_visible(cx, true);
                let form_type = form.form_type.as_deref().unwrap_or("form");
                form_view.label(ids!(form_label)).set_text(cx, &format!("{}: {}", form_type, form.form));
            }
        }

        // =================================================================
        // Definitions Card
        // =================================================================
        // Filter definitions by language
        let zh_defs: Vec<_> = result.definitions.iter()
            .filter(|d| d.language == "zh")
            .collect();
        let en_defs: Vec<_> = result.definitions.iter()
            .filter(|d| d.language == "en")
            .collect();

        if !zh_defs.is_empty() || !en_defs.is_empty() {
            let def_card = self.view.view(ids!(main_content.center_column.lookup_content.word_detail_scroll.word_detail_content.definitions_card));
            def_card.set_visible(cx, true);

            // Populate Chinese definitions
            let zh_def_ids = [
                ids!(zh_definitions.zh_def_1),
                ids!(zh_definitions.zh_def_2),
                ids!(zh_definitions.zh_def_3),
            ];
            for (i, def) in zh_defs.iter().take(3).enumerate() {
                let def_view = def_card.view(zh_def_ids[i]);
                def_view.set_visible(cx, true);
                def_view.label(ids!(definition_text)).set_text(cx, &def.definition);

                if let Some(ref pos) = def.part_of_speech {
                    let pos_tag = def_view.view(ids!(tags_row.pos_tag));
                    pos_tag.set_visible(cx, true);
                    pos_tag.label(ids!(pos_label)).set_text(cx, pos);
                }

                if let Some(ref register) = def.register {
                    let label = def_view.label(ids!(tags_row.register_label));
                    label.set_visible(cx, true);
                    label.set_text(cx, register);
                }
            }
        }

        // =================================================================
        // Example Sentences Card
        // =================================================================
        if !result.sentences.is_empty() {
            let sent_card = self.view.view(ids!(main_content.center_column.lookup_content.word_detail_scroll.word_detail_content.sentences_card));
            sent_card.set_visible(cx, true);

            let sent_ids = [
                ids!(sentences_list.sent_1),
                ids!(sentences_list.sent_2),
                ids!(sentences_list.sent_3),
                ids!(sentences_list.sent_4),
                ids!(sentences_list.sent_5),
            ];
            for (i, sentence) in result.sentences.iter().take(5).enumerate() {
                let sent_view = sent_card.view(sent_ids[i]);
                sent_view.set_visible(cx, true);
                sent_view.label(ids!(sentence_text)).set_text(cx, &sentence.sentence);

                if let Some(ref source) = sentence.source {
                    let source_label = sent_view.label(ids!(sentence_source));
                    source_label.set_visible(cx, true);
                    source_label.set_text(cx, &format!("— {}", source));
                }
            }

            // Show "more" button if there are more than 5 sentences
            if result.sentences.len() > 5 {
                sent_card.button(ids!(show_more_btn)).set_visible(cx, true);
                sent_card.button(ids!(show_more_btn)).set_text(cx, &format!("查看更多 ({} 条)", result.sentences.len() - 5));
            }
        }

        // =================================================================
        // Related Words Card (synonyms, antonyms, word family)
        // =================================================================
        // Filter relations by type
        let synonyms: Vec<_> = result.relations.iter()
            .filter(|r| r.relation_type.as_deref() == Some("synonym"))
            .collect();
        let antonyms: Vec<_> = result.relations.iter()
            .filter(|r| r.relation_type.as_deref() == Some("antonym"))
            .collect();
        let family: Vec<_> = result.relations.iter()
            .filter(|r| {
                let rt = r.relation_type.as_deref();
                rt != Some("synonym") && rt != Some("antonym")
            })
            .collect();

        let has_synonyms = !synonyms.is_empty();
        let has_antonyms = !antonyms.is_empty();
        let has_family = !family.is_empty();

        if has_synonyms || has_antonyms || has_family {
            let rel_card = self.view.view(ids!(main_content.center_column.lookup_content.word_detail_scroll.word_detail_content.related_card));
            rel_card.set_visible(cx, true);

            // Synonyms
            if has_synonyms {
                let syn_ids = [
                    ids!(synonyms_content.syn_1),
                    ids!(synonyms_content.syn_2),
                    ids!(synonyms_content.syn_3),
                    ids!(synonyms_content.syn_4),
                ];
                for (i, syn) in synonyms.iter().take(4).enumerate() {
                    let syn_view = rel_card.view(syn_ids[i]);
                    syn_view.set_visible(cx, true);
                    syn_view.label(ids!(tag_label)).set_text(cx, &syn.related_word);
                }
            }

            // Antonyms
            if has_antonyms {
                let ant_ids = [
                    ids!(antonyms_content.ant_1),
                    ids!(antonyms_content.ant_2),
                ];
                for (i, ant) in antonyms.iter().take(2).enumerate() {
                    let ant_view = rel_card.view(ant_ids[i]);
                    ant_view.set_visible(cx, true);
                    ant_view.label(ids!(tag_label)).set_text(cx, &ant.related_word);
                }
            }

            // Word family (as related words)
            if has_family {
                let rel_ids = [
                    ids!(related_content.rel_1),
                    ids!(related_content.rel_2),
                ];
                for (i, rel) in family.iter().take(2).enumerate() {
                    let rel_view = rel_card.view(rel_ids[i]);
                    rel_view.set_visible(cx, true);
                    rel_view.label(ids!(tag_label)).set_text(cx, &rel.related_word);
                }
            }
        }

        self.view.redraw(cx);
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
