//! MainBody Widget - Main application layout container
//!
//! The MainBody provides the base layer for MoFA Studio with:
//! - Header with logo, title, theme toggle, and user profile
//! - Content area with app pages (FM, Settings, etc.)
//! - Tab overlay for modal-like Profile/Settings tabs

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Import fonts and colors from shared theme
    use colang_widgets::theme::FONT_REGULAR;
    use colang_widgets::theme::FONT_MEDIUM;
    use colang_widgets::theme::FONT_SEMIBOLD;
    use colang_widgets::theme::FONT_BOLD;
    use colang_widgets::theme::DARK_BG;
    use colang_widgets::theme::PANEL_BG;
    use colang_widgets::theme::ACCENT_INDIGO;
    use colang_widgets::theme::TEXT_PRIMARY;
    use colang_widgets::theme::TEXT_SECONDARY;
    use colang_widgets::theme::HOVER_BG;
    use colang_widgets::theme::TRANSPARENT;
    use colang_widgets::theme::SLATE_50;
    use colang_widgets::theme::SLATE_200;
    use colang_widgets::theme::SLATE_400;
    use colang_widgets::theme::SLATE_500;
    use colang_widgets::theme::SLATE_700;
    use colang_widgets::theme::SLATE_800;
    use colang_widgets::theme::GRAY_300;
    use colang_widgets::theme::GRAY_600;
    use colang_widgets::theme::INDIGO_100;
    use colang_widgets::theme::DARK_BG_DARK;
    use colang_widgets::theme::PANEL_BG_DARK;
    use colang_widgets::theme::TEXT_PRIMARY_DARK;
    use colang_widgets::theme::TEXT_SECONDARY_DARK;

    use colang_core::screens::conversation::conversation_screen::ConversationScreen;
    use colang_core::screens::dictionary::dictionary_screen::DictionaryScreen;
    use colang_core::screens::home::home_screen::HomeScreen;
    use colang_core::screens::reading::reading_screen::ReadingScreen;
    use colang_core::screens::review::review_screen::ReviewScreen;
    use colang_core::screens::scenes::scenes_screen::Scenes;
    use colang_core::screens::settings::settings_screen::SettingsScreen;
    use crate::widgets::tabs::TabWidget;
    use crate::widgets::tabs::TabBar;
    use colang_widgets::debug_panel::DebugPanel;

    // Logo image
    COLANG_LOGO = dep("crate://self/resources/colang-logo.png")

    pub MainBody = {{MainBody}} <View> {
        width: Fill, height: Fill
        flow: Overlay
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Base layer - header + content area
        base = <View> {
            width: Fill, height: Fill
            flow: Down

            // Header
            header = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12
                align: {y: 0.5}
                padding: {left: 20, right: 20, top: 15, bottom: 15}
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                    }
                }

                hamburger_placeholder = <View> {
                    width: 21, height: 21
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let cy = self.rect_size.y * 0.5;
                            let cx = self.rect_size.x * 0.5;
                            sdf.move_to(cx - 5.0, cy - 4.0);
                            sdf.line_to(cx + 5.0, cy - 4.0);
                            sdf.stroke((SLATE_500), 1.5);
                            sdf.move_to(cx - 5.0, cy);
                            sdf.line_to(cx + 5.0, cy);
                            sdf.stroke((SLATE_500), 1.5);
                            sdf.move_to(cx - 5.0, cy + 4.0);
                            sdf.line_to(cx + 5.0, cy + 4.0);
                            sdf.stroke((SLATE_500), 1.5);
                            return sdf.result;
                        }
                    }
                }

                logo = <Image> {
                    width: 40, height: 40
                    source: (COLANG_LOGO)
                }

                title = <Label> {
                    text: "开朗英语"
                    draw_text: {
                        color: (TEXT_PRIMARY)
                        text_style: <FONT_BOLD>{ font_size: 24.0 }
                    }
                }

                <View> { width: Fill, height: 1 }

                page_title_container = <View> {
                    width: Fit, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 8

                    page_icon = <Label> {
                        text: "🏠"
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 16.0 }
                            color: (SLATE_500)
                        }
                    }

                    page_title = <Label> {
                        text: "首页"
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                            color: (TEXT_SECONDARY)
                        }
                    }
                }

                <View> { width: Fill, height: 1 }

                // Debug button - opens debug console
                debug_btn = <View> {
                    width: 36, height: 36
                    align: {x: 0.5, y: 0.5}
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let cx = self.rect_size.x * 0.5;
                            let cy = self.rect_size.y * 0.5;
                            // Hover circle background
                            sdf.circle(cx, cy, 16.0);
                            let light_hover = (HOVER_BG);
                            let dark_hover = (SLATE_700);
                            let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                            sdf.fill(mix((TRANSPARENT), hover_color, self.hover));
                            // Bug/terminal icon
                            let icon_color = mix((SLATE_500), (SLATE_400), self.dark_mode);
                            // Terminal box
                            sdf.box(cx - 7.0, cy - 5.0, 14.0, 10.0, 2.0);
                            sdf.stroke(icon_color, 1.2);
                            // Prompt symbol >_
                            sdf.move_to(cx - 4.0, cy - 1.0);
                            sdf.line_to(cx - 1.0, cy + 1.0);
                            sdf.line_to(cx - 4.0, cy + 3.0);
                            sdf.stroke(icon_color, 1.2);
                            sdf.move_to(cx + 1.0, cy + 3.0);
                            sdf.line_to(cx + 5.0, cy + 3.0);
                            sdf.stroke(icon_color, 1.2);
                            return sdf.result;
                        }
                    }
                }

                // Theme toggle button
                theme_toggle = <View> {
                    width: 36, height: 36
                    align: {x: 0.5, y: 0.5}
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let cx = self.rect_size.x * 0.5;
                            let cy = self.rect_size.y * 0.5;
                            sdf.circle(cx, cy, 16.0);
                            sdf.fill(mix((TRANSPARENT), (HOVER_BG), self.hover));
                            return sdf.result;
                        }
                    }

                    // Sun icon (light mode) - amber color
                    sun_icon = <View> {
                        width: 20, height: 20
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let c = self.rect_size * 0.5;
                                let amber = vec4(0.961, 0.624, 0.043, 1.0);  // AMBER_500 #f59f0b
                                // Sun circle
                                sdf.circle(c.x, c.y, 4.0);
                                sdf.fill(amber);
                                // Sun rays
                                let ray_len = 2.5;
                                let ray_dist = 6.5;
                                sdf.move_to(c.x, c.y - ray_dist);
                                sdf.line_to(c.x, c.y - ray_dist - ray_len);
                                sdf.stroke(amber, 1.5);
                                sdf.move_to(c.x, c.y + ray_dist);
                                sdf.line_to(c.x, c.y + ray_dist + ray_len);
                                sdf.stroke(amber, 1.5);
                                sdf.move_to(c.x - ray_dist, c.y);
                                sdf.line_to(c.x - ray_dist - ray_len, c.y);
                                sdf.stroke(amber, 1.5);
                                sdf.move_to(c.x + ray_dist, c.y);
                                sdf.line_to(c.x + ray_dist + ray_len, c.y);
                                sdf.stroke(amber, 1.5);
                                return sdf.result;
                            }
                        }
                    }

                    // Moon icon (dark mode - hidden by default) - indigo color
                    moon_icon = <View> {
                        width: 20, height: 20
                        visible: false
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let c = self.rect_size * 0.5;
                                let indigo = vec4(0.388, 0.400, 0.945, 1.0);  // INDIGO_500 #6366f1
                                let white = vec4(1.0, 1.0, 1.0, 1.0);
                                sdf.circle(c.x, c.y, 6.0);
                                sdf.fill(indigo);
                                sdf.circle(c.x + 3.5, c.y - 2.5, 4.5);
                                sdf.fill(white);
                                return sdf.result;
                            }
                        }
                    }
                }

                user_profile_container = <View> {
                    width: Fit, height: Fill
                    flow: Right
                    align: {x: 0.5, y: 0.5}
                    spacing: 4

                    login_btn = <Button> {
                        width: Fit, height: 32
                        padding: {left: 12, right: 12, top: 8, bottom: 8}
                        align: {x: 0.5, y: 0.5}
                        text: "登录"

                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }

                        draw_bg: {
                            instance hover: 0.0
                            instance pressed: 0.0
                            instance dark_mode: 0.0

                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let r = 8.0;
                                let bg_light = (SLATE_50);
                                let bg_dark = (SLATE_800);
                                let hover_light = (SLATE_200);
                                let hover_dark = (SLATE_700);
                                let border_light = (GRAY_300);
                                let border_dark = (SLATE_700);
                                let base = mix(bg_light, bg_dark, self.dark_mode);
                                let hover = mix(hover_light, hover_dark, self.dark_mode);
                                let fill = mix(base, hover, self.hover);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, r);
                                sdf.fill(fill);
                                let border = mix(border_light, border_dark, self.dark_mode);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, r);
                                sdf.stroke(border, 1.0);
                                return sdf.result;
                            }
                        }

                        animator: {
                            hover = {
                                default: off
                                off = { from: {all: Forward {duration: 0.08}} apply: {draw_bg: {hover: 0.0}} }
                                on = { from: {all: Forward {duration: 0.08}} apply: {draw_bg: {hover: 1.0}} }
                            }
                            pressed = {
                                default: off
                                off = { from: {all: Forward {duration: 0.05}} apply: {draw_bg: {pressed: 0.0}} }
                                on = { from: {all: Forward {duration: 0.02}} apply: {draw_bg: {pressed: 1.0}} }
                            }
                        }
                    }
                }

                close_app_btn = <Button> {
                    width: 32, height: 32
                    margin: {left: 8}
                    padding: 0
                    align: {x: 0.5, y: 0.5}

                    draw_text: {
                        text_style: <FONT_BOLD>{ font_size: 18.0 }
                        fn get_color(self) -> vec4 {
                            return mix((SLATE_500), (SLATE_400), self.hover);
                        }
                    }
                    text: "×"

                    draw_bg: {
                        instance hover: 0.0
                        instance pressed: 0.0
                        instance dark_mode: 0.0

                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let r = 6.0;
                            let bg_light = (TRANSPARENT);
                            let bg_dark = (TRANSPARENT);
                            let hover_light = (SLATE_200);
                            let hover_dark = (SLATE_700);
                            let base = mix(bg_light, bg_dark, self.dark_mode);
                            let hover = mix(hover_light, hover_dark, self.dark_mode);
                            let fill = mix(base, hover, self.hover);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, r);
                            sdf.fill(fill);
                            return sdf.result;
                        }
                    }

                    animator: {
                        hover = {
                            default: off
                            off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                            on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
                        }
                        pressed = {
                            default: off
                            off = { from: {all: Forward {duration: 0.05}} apply: {draw_bg: {pressed: 0.0}} }
                            on = { from: {all: Forward {duration: 0.02}} apply: {draw_bg: {pressed: 1.0}} }
                        }
                    }
                }
            }

            // Content area
            content_area = <View> {
                width: Fill, height: Fill
                flow: Right
                padding: 20

                main_content = <View> {
                    width: Fill, height: Fill
                    flow: Down

                    content = <View> {
                        width: Fill, height: Fill
                        flow: Overlay

                        home_screen = <HomeScreen> {
                            width: Fill, height: Fill
                            visible: true
                        }

                        conversation_screen = <ConversationScreen> {
                            width: Fill, height: Fill
                            visible: false
                        }

                        scenes_screen = <Scenes> {
                            width: Fill, height: Fill
                            visible: false
                        }

                        review_screen = <ReviewScreen> {
                            width: Fill, height: Fill
                            visible: false
                        }

                        reading_screen = <ReadingScreen> {
                            width: Fill, height: Fill
                            visible: false
                        }

                        dictionary_screen = <DictionaryScreen> {
                            width: Fill, height: Fill
                            visible: false
                        }

                        settings_screen = <SettingsScreen> {
                            width: Fill, height: Fill
                            visible: false
                        }
                    }
                }

                debug_splitter = <View> {
                    width: 6, height: Fill
                    visible: true
                    show_bg: true
                    draw_bg: {
                        instance hover: 0.0
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let light = (SLATE_200);
                            let dark = (SLATE_700);
                            let base = mix(light, dark, self.dark_mode);
                            let fill = mix((TRANSPARENT), base, self.hover);
                            sdf.box(2.0, 0.0, 2.0, self.rect_size.y, 1.0);
                            sdf.fill(fill);
                            return sdf.result;
                        }
                    }
                }

                debug_panel = <DebugPanel> {}
            }
        }

        // Tab overlay - modal layer for Profile/Settings
        tab_overlay = <View> {
            width: Fill, height: Fill
            flow: Down
            visible: false
            margin: {top: 70}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
                }
            }

            tab_bar = <TabBar> {
                profile_tab = <TabWidget> {
                    visible: false
                    tab_label = { text: "Profile" }
                }
                settings_tab = <TabWidget> {
                    visible: false
                    tab_label = { text: "Settings" }
                }
            }

            tab_content = <View> {
                width: Fill, height: Fill
                flow: Overlay
                padding: 20

                profile_page = <RoundedView> {
                    padding: 20
                    width: Fill, height: Fill
                    visible: false
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 8.0
                        fn get_color(self) -> vec4 {
                            return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                        }
                    }
                    padding: 24
                    flow: Down
                    spacing: 16

                    profile_title = <Label> {
                        text: "User Profile"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 20.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    profile_divider = <View> {
                        width: Fill, height: 1
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                return mix((SLATE_200), (SLATE_700), self.dark_mode);
                            }
                        }
                    }

                    profile_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 16
                        align: {y: 0.5}

                        profile_avatar = <View> {
                            width: 64, height: 64
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let c = self.rect_size * 0.5;
                                    sdf.circle(c.x, c.y, 30.0);
                                    let bg = mix((INDIGO_100), (SLATE_700), self.dark_mode);
                                    sdf.fill(bg);
                                    return sdf.result;
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            <Icon> {
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/icons/user.svg")
                                    fn get_color(self) -> vec4 { return (ACCENT_INDIGO); }
                                }
                                icon_walk: {width: 32, height: 32}
                            }
                        }

                        profile_info = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 4

                            profile_name = <Label> {
                                text: "Demo User"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                            profile_email = <Label> {
                                text: "demo@mofa.studio"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }
                    }

                    profile_coming_soon = <Label> {
                        text: "Profile settings coming soon..."
                        margin: {top: 20}
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((SLATE_400), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                settings_tab_page = <SettingsScreen> {
                    width: Fill, height: Fill
                    visible: false
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MainBody {
    #[deref]
    view: View,
}

impl Widget for MainBody {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
