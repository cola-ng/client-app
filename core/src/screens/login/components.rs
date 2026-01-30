//! # Login Components
//!
//! Shared components for the login screen.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // OAuth button component
    pub OAuthButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 20, right: 20, top: 14, bottom: 14}
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_bg = mix((WHITE), (SLATE_50), self.hover);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.hover);
                let bg = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg);

                let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
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

    // Primary form button
    pub FormButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 20, right: 20, top: 14, bottom: 14}
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance hover: 0.0
            instance disabled: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let enabled = mix((ACCENT_PRIMARY), (ACCENT_PRIMARY_HOVER), self.hover);
                let disabled_bg = mix((SLATE_300), (SLATE_600), self.dark_mode);
                let bg = mix(enabled, disabled_bg, self.disabled);
                sdf.fill(bg);

                return sdf.result;
            }
        }

        draw_text: {
            instance disabled: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((WHITE), vec4(1., 1., 1., 0.6), self.disabled);
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

    // Form text input
    pub FormInput = <TextInput> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 14, bottom: 14}

        draw_bg: {
            instance dark_mode: 0.0
            instance error: 0.0
            instance focus: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);

                let normal_border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                let focus_border = (ACCENT_PRIMARY);
                let error_border = (ACCENT_RED);
                let border = mix(mix(normal_border, focus_border, self.focus), error_border, self.error);
                sdf.stroke(border, mix(1.0, 2.0, max(self.focus, self.error)));

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

    // Form label
    pub FormLabel = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Form error text
    pub FormError = <Label> {
        draw_text: {
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            color: (ACCENT_RED)
        }
    }

    // Divider with text
    pub DividerWithText = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 12

        line_left = <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
                }
            }
        }

        text = <Label> {
            text: "或"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }

        line_right = <View> {
            width: Fill, height: 1
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
                }
            }
        }
    }
}
