//! # Dropdown Menu Component
//!
//! Dropdown menu widget with items and separators.
//!
//! ## Features
//! - Menu items with icons
//! - Separators for grouping
//! - Hover states
//! - Dark mode support

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // DROPDOWN MENU CONTAINER
    // ==========================================================================
    pub DropdownMenu = <RoundedView> {
        width: 200, height: Fit
        flow: Down
        padding: 6
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow
                sdf.box(2., 4., self.rect_size.x, self.rect_size.y, 8.0);
                let shadow = mix(vec4(0., 0., 0., 0.1), vec4(0., 0., 0., 0.25), self.dark_mode);
                sdf.fill(shadow);

                // Main container
                sdf.box(0., 0., self.rect_size.x - 2., self.rect_size.y - 2., 8.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);

                // Border
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }
    }

    // Wider dropdown menu
    pub DropdownMenuWide = <DropdownMenu> {
        width: 280
    }

    // ==========================================================================
    // MENU ITEM - Clickable item with optional icon
    // ==========================================================================
    pub MenuItem = <Button> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        align: {x: 0.0, y: 0.5}
        icon_walk: {width: 16, height: 16, margin: {right: 10}}

        draw_bg: {
            instance hover: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let bg = mix((TRANSPARENT), hover_bg, self.hover);

                sdf.fill(bg);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }

        draw_icon: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_500), (SLATE_400), self.dark_mode);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.08}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.08}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    // Destructive menu item (red text)
    pub MenuItemDestructive = <MenuItem> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((ACCENT_RED), (RED_400), self.dark_mode);
            }
        }
        draw_icon: {
            fn get_color(self) -> vec4 {
                return (ACCENT_RED);
            }
        }
    }

    // Disabled menu item
    pub MenuItemDisabled = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        align: {y: 0.5}
        spacing: 10

        item_icon = <Icon> {
            draw_icon: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix((SLATE_300), (SLATE_600), self.dark_mode);
                }
            }
            icon_walk: {width: 16, height: 16}
        }

        item_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_400), (SLATE_500), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // MENU SEPARATOR - Horizontal divider
    // ==========================================================================
    pub MenuSeparator = <View> {
        width: Fill, height: 1
        margin: {top: 4, bottom: 4, left: 8, right: 8}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_200), (SLATE_700), self.dark_mode);
            }
        }
    }

    // ==========================================================================
    // MENU HEADER - Non-clickable section header
    // ==========================================================================
    pub MenuHeader = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 8, bottom: 4}

        header_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_500), (SLATE_400), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // MENU ITEM WITH BADGE
    // ==========================================================================
    pub MenuItemWithBadge = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        align: {y: 0.5}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let bg = mix((TRANSPARENT), hover_bg, self.hover);

                sdf.fill(bg);
                return sdf.result;
            }
        }

        item_icon = <Icon> {
            draw_icon: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix((SLATE_500), (SLATE_400), self.dark_mode);
                }
            }
            icon_walk: {width: 16, height: 16, margin: {right: 10}}
        }

        item_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_700), (SLATE_200), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        item_badge = <RoundedView> {
            width: Fit, height: Fit
            padding: {left: 6, right: 6, top: 2, bottom: 2}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance border_radius: 10.0
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
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
    // MENU ITEM WITH SHORTCUT
    // ==========================================================================
    pub MenuItemWithShortcut = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        align: {y: 0.5}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let bg = mix((TRANSPARENT), hover_bg, self.hover);

                sdf.fill(bg);
                return sdf.result;
            }
        }

        item_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_700), (SLATE_200), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        shortcut_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_400), (SLATE_500), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // CHECKBOX MENU ITEM
    // ==========================================================================
    pub MenuItemCheckbox = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        align: {y: 0.5}
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let bg = mix((TRANSPARENT), hover_bg, self.hover);

                sdf.fill(bg);
                return sdf.result;
            }
        }

        checkbox = <View> {
            width: 16, height: 16
            margin: {right: 10}
            show_bg: true
            draw_bg: {
                instance checked: 0.0
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Checkbox box
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                    let unchecked_bg = mix((WHITE), (SLATE_700), self.dark_mode);
                    let checked_bg = (ACCENT_PRIMARY);
                    let bg = mix(unchecked_bg, checked_bg, self.checked);
                    let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                    let final_border = mix(border, checked_bg, self.checked);

                    sdf.fill(bg);
                    sdf.stroke(final_border, 1.0);

                    // Checkmark
                    if self.checked > 0.5 {
                        sdf.move_to(c.x - 3., c.y);
                        sdf.line_to(c.x - 1., c.y + 3.);
                        sdf.line_to(c.x + 4., c.y - 2.);
                        sdf.stroke((WHITE), 1.5);
                    }

                    return sdf.result;
                }
            }
        }

        item_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_700), (SLATE_200), self.dark_mode);
                }
            }
        }
    }
}
