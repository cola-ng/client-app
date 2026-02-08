//! # Tabs Component
//!
//! Tab navigation widget with horizontal/vertical layouts and badges.
//!
//! ## Features
//! - Horizontal and vertical layouts
//! - Scrollable tabs for many items
//! - Badge support
//! - Active state indicator

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // TAB BUTTON - Individual tab item
    // ==========================================================================
    pub TabButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        margin: 0

        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background (only on hover when not active)
                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let no_active_bg = mix((TRANSPARENT), hover_bg, self.hover * (1.0 - self.active));

                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(no_active_bg);

                // Active indicator (bottom border)
                if self.active > 0.5 {
                    sdf.rect(0., self.rect_size.y - 2., self.rect_size.x, 2.);
                    sdf.fill((ACCENT_PRIMARY));
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance active: 0.0
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }

            fn get_color(self) -> vec4 {
                let inactive_light = (SLATE_600);
                let inactive_dark = (SLATE_400);
                let active_light = (ACCENT_PRIMARY);
                let active_dark = (ACCENT_PRIMARY);

                let inactive = mix(inactive_light, inactive_dark, self.dark_mode);
                let active_color = mix(active_light, active_dark, self.dark_mode);

                return mix(inactive, active_color, self.active);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    // Tab with icon
    pub TabButtonIcon = <TabButton> {
        icon_walk: {width: 16, height: 16, margin: {right: 8}}
        draw_icon: {
            instance active: 0.0
            instance dark_mode: 0.0

            fn get_color(self) -> vec4 {
                let inactive_light = (SLATE_500);
                let inactive_dark = (SLATE_400);
                let active_color = (ACCENT_PRIMARY);

                let inactive = mix(inactive_light, inactive_dark, self.dark_mode);
                return mix(inactive, active_color, self.active);
            }
        }
    }

    // ==========================================================================
    // PILL TAB - Rounded pill style tabs
    // ==========================================================================
    pub TabPill = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 8, bottom: 8}
        margin: {right: 8}

        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 20.0);

                let light_inactive = (SLATE_200);
                let dark_inactive = (SLATE_700);
                let light_hover = (SLATE_300);
                let dark_hover = (SLATE_600);
                let active_color = (ACCENT_PRIMARY);

                let inactive = mix(light_inactive, dark_inactive, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);

                let base = mix(inactive, hover_color, self.hover);
                let final_color = mix(base, active_color, self.active);

                sdf.fill(final_color);
                return sdf.result;
            }
        }

        draw_text: {
            instance active: 0.0
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }

            fn get_color(self) -> vec4 {
                let inactive_light = (SLATE_600);
                let inactive_dark = (SLATE_300);
                let inactive = mix(inactive_light, inactive_dark, self.dark_mode);
                return mix(inactive, (WHITE), self.active);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    // ==========================================================================
    // SEGMENTED TABS - Connected buttons
    // ==========================================================================
    pub SegmentedTabGroup = <RoundedView> {
        width: Fit, height: Fit
        flow: Right
        padding: 4
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_200), (SLATE_700), self.dark_mode);
            }
        }
    }

    pub SegmentedTab = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 8, bottom: 8}

        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);

                let light_active = (WHITE);
                let dark_active = (SLATE_600);
                let active_bg = mix(light_active, dark_active, self.dark_mode);

                let final_bg = mix((TRANSPARENT), active_bg, self.active);
                sdf.fill(final_bg);

                return sdf.result;
            }
        }

        draw_text: {
            instance active: 0.0
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }

            fn get_color(self) -> vec4 {
                let inactive_light = (SLATE_600);
                let inactive_dark = (SLATE_400);
                let active_light = (SLATE_900);
                let active_dark = (WHITE);

                let inactive = mix(inactive_light, inactive_dark, self.dark_mode);
                let active_color = mix(active_light, active_dark, self.dark_mode);

                return mix(inactive, active_color, self.active);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    // ==========================================================================
    // TAB WITH BADGE
    // ==========================================================================
    pub TabButtonWithBadge = <View> {
        width: Fit, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let no_active_bg = mix((TRANSPARENT), hover_bg, self.hover * (1.0 - self.active));

                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(no_active_bg);

                if self.active > 0.5 {
                    sdf.rect(0., self.rect_size.y - 2., self.rect_size.x, 2.);
                    sdf.fill((ACCENT_PRIMARY));
                }

                return sdf.result;
            }
        }

        tab_label = <Label> {
            draw_text: {
                instance active: 0.0
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }

                fn get_color(self) -> vec4 {
                    let inactive_light = (SLATE_600);
                    let inactive_dark = (SLATE_400);
                    let active_color = (ACCENT_PRIMARY);
                    let inactive = mix(inactive_light, inactive_dark, self.dark_mode);
                    return mix(inactive, active_color, self.active);
                }
            }
        }

        tab_badge = <RoundedView> {
            width: Fit, height: Fit
            padding: {left: 6, right: 6, top: 2, bottom: 2}
            margin: {left: 8}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance border_radius: 10.0
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_RED), (RED_400), self.dark_mode);
                }
            }

            badge_text = <Label> {
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                    color: (WHITE)
                }
            }
        }
    }

    // ==========================================================================
    // TAB LIST CONTAINERS
    // ==========================================================================

    // Horizontal tab list
    pub TabList = <View> {
        width: Fill, height: Fit
        flow: Right
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Bottom border
                sdf.rect(0., self.rect_size.y - 1., self.rect_size.x, 1.);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(border);
                return sdf.result;
            }
        }
    }

    // Scrollable horizontal tabs
    pub TabListScroll = <ScrollXView> {
        width: Fill, height: Fit
        scroll_bars: <ScrollBars> { show_scroll_x: false, show_scroll_y: false }
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.rect(0., self.rect_size.y - 1., self.rect_size.x, 1.);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(border);
                return sdf.result;
            }
        }

        tabs = <View> {
            width: Fit, height: Fit
            flow: Right
        }
    }

    // Vertical tab list
    pub TabListVertical = <View> {
        width: Fit, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Right border
                sdf.rect(self.rect_size.x - 1., 0., 1., self.rect_size.y);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(border);
                return sdf.result;
            }
        }
    }

    // Vertical tab button
    pub TabButtonVertical = <Button> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        align: {x: 0.0, y: 0.5}

        draw_bg: {
            instance hover: 0.0
            instance active: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let no_active_bg = mix((TRANSPARENT), hover_bg, self.hover * (1.0 - self.active));

                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                sdf.fill(no_active_bg);

                // Active indicator (left border)
                if self.active > 0.5 {
                    sdf.rect(0., 0., 3., self.rect_size.y);
                    sdf.fill((ACCENT_PRIMARY));
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance active: 0.0
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }

            fn get_color(self) -> vec4 {
                let inactive_light = (SLATE_600);
                let inactive_dark = (SLATE_400);
                let active_color = (ACCENT_PRIMARY);
                let inactive = mix(inactive_light, inactive_dark, self.dark_mode);
                return mix(inactive, active_color, self.active);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }
}
