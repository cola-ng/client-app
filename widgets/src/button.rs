//! # Button Component
//!
//! Reusable button widget with multiple variants and sizes.
//!
//! ## Variants
//! - Primary: Orange/accent background, white text
//! - Secondary: Subtle background with border
//! - Outline: Transparent with border
//! - Ghost: Transparent, text only
//! - Destructive: Red background for dangerous actions
//!
//! ## Sizes
//! - Small: Compact padding, smaller text
//! - Medium: Default size
//! - Large: More padding, larger text

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // Base button styles shared by all variants
    ButtonBase = <Button> {
        width: Fit, height: Fit
        align: {x: 0.5, y: 0.5}

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

    // ==========================================================================
    // PRIMARY BUTTON - Orange/accent background
    // ==========================================================================
    pub PrimaryButton = <ButtonBase> {
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0
            instance disabled: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let normal = (ACCENT_PRIMARY);
                let hovered = (ACCENT_PRIMARY_HOVER);
                let pressed_color = vec4(0.78, 0.30, 0.05, 1.0); // darker orange
                let disabled_color = mix((SLATE_300), (SLATE_600), self.dark_mode);

                let base = mix(normal, hovered, self.hover);
                let active = mix(base, pressed_color, self.pressed);
                let final_color = mix(active, disabled_color, self.disabled);

                sdf.fill(final_color);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance disabled: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_400), self.disabled);
            }
        }
    }

    pub PrimaryButtonSmall = <PrimaryButton> {
        padding: {left: 12, right: 12, top: 6, bottom: 6}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
    }

    pub PrimaryButtonLarge = <PrimaryButton> {
        padding: {left: 24, right: 24, top: 14, bottom: 14}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 15.0 } }
    }

    // ==========================================================================
    // SECONDARY BUTTON - Subtle background with border
    // ==========================================================================
    pub SecondaryButton = <ButtonBase> {
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0
            instance disabled: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_bg = (SLATE_100);
                let dark_bg = (SLATE_700);
                let light_hover = (SLATE_200);
                let dark_hover = (SLATE_600);
                let light_border = (SLATE_300);
                let dark_border = (SLATE_600);

                let bg = mix(light_bg, dark_bg, self.dark_mode);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let border = mix(light_border, dark_border, self.dark_mode);

                let final_bg = mix(bg, hover_bg, self.hover);
                sdf.fill(final_bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance disabled: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }
    }

    pub SecondaryButtonSmall = <SecondaryButton> {
        padding: {left: 12, right: 12, top: 6, bottom: 6}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
    }

    pub SecondaryButtonLarge = <SecondaryButton> {
        padding: {left: 24, right: 24, top: 14, bottom: 14}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 15.0 } }
    }

    // ==========================================================================
    // OUTLINE BUTTON - Transparent with border
    // ==========================================================================
    pub OutlineButton = <ButtonBase> {
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0
            instance disabled: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_800);
                let light_border = (SLATE_300);
                let dark_border = (SLATE_600);

                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let border = mix(light_border, dark_border, self.dark_mode);

                let final_bg = mix((TRANSPARENT), hover_bg, self.hover);
                sdf.fill(final_bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance disabled: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }
    }

    pub OutlineButtonSmall = <OutlineButton> {
        padding: {left: 12, right: 12, top: 6, bottom: 6}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
    }

    // ==========================================================================
    // GHOST BUTTON - Transparent, text only
    // ==========================================================================
    pub GhostButton = <ButtonBase> {
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0
            instance disabled: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_800);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let final_bg = mix((TRANSPARENT), hover_bg, self.hover);

                sdf.fill(final_bg);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance disabled: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }
    }

    pub GhostButtonSmall = <GhostButton> {
        padding: {left: 12, right: 12, top: 6, bottom: 6}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
    }

    // ==========================================================================
    // DESTRUCTIVE BUTTON - Red background for dangerous actions
    // ==========================================================================
    pub DestructiveButton = <ButtonBase> {
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0
            instance disabled: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let normal = (ACCENT_RED);
                let hovered = vec4(0.86, 0.15, 0.15, 1.0); // darker red
                let pressed_color = vec4(0.73, 0.11, 0.11, 1.0);
                let disabled_color = mix((SLATE_300), (SLATE_600), self.dark_mode);

                let base = mix(normal, hovered, self.hover);
                let active = mix(base, pressed_color, self.pressed);
                let final_color = mix(active, disabled_color, self.disabled);

                sdf.fill(final_color);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance disabled: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_400), self.disabled);
            }
        }
    }

    pub DestructiveButtonSmall = <DestructiveButton> {
        padding: {left: 12, right: 12, top: 6, bottom: 6}
        draw_text: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
    }

    // ==========================================================================
    // ICON BUTTON - Square button with icon
    // ==========================================================================
    pub IconButton = <ButtonBase> {
        width: 36, height: 36
        padding: 0
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x - 2.0);

                let light_hover = (SLATE_100);
                let dark_hover = (SLATE_700);
                let hover_bg = mix(light_hover, dark_hover, self.dark_mode);
                let final_bg = mix((TRANSPARENT), hover_bg, self.hover);

                sdf.fill(final_bg);
                return sdf.result;
            }
        }
        draw_icon: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_500), (SLATE_400), self.dark_mode);
            }
        }
    }

    pub IconButtonSmall = <IconButton> {
        width: 28, height: 28
    }

    pub IconButtonLarge = <IconButton> {
        width: 44, height: 44
    }
}
