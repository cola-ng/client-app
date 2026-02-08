//! # Toast Component
//!
//! Toast notification widget with multiple types and auto-dismiss.
//!
//! ## Types
//! - Success: Green, checkmark icon
//! - Error: Red, X icon
//! - Warning: Amber, exclamation icon
//! - Info: Blue, info icon
//!
//! ## Features
//! - Auto-dismiss with configurable duration
//! - Slide-in/out animations
//! - Stacking support

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // TOAST BASE - Foundation for all toast types
    // ==========================================================================
    pub ToastBase = <RoundedView> {
        width: 360, height: Fit
        flow: Right
        padding: {left: 16, right: 16, top: 14, bottom: 14}
        align: {y: 0.5}
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance slide: 1.0  // 0 = hidden, 1 = visible
            instance border_radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow
                sdf.box(2., 4., self.rect_size.x, self.rect_size.y, 8.0);
                let shadow = mix(vec4(0., 0., 0., 0.1), vec4(0., 0., 0., 0.25), self.dark_mode);
                sdf.fill(shadow);

                // Main body
                sdf.box(0., 0., self.rect_size.x - 2., self.rect_size.y - 2., 8.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);

                // Border
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(border, 1.0);

                return sdf.result;
            }
        }

        toast_icon = <View> {
            width: 24, height: 24
            show_bg: true
        }

        toast_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            toast_title = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            toast_message = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        close_btn = <Button> {
            width: 28, height: 28
            padding: 0
            align: {x: 0.5, y: 0.5}
            text: ""
            draw_bg: {
                instance hover: 0.0
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y, 12.0);
                    let hover_bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(mix((TRANSPARENT), hover_bg, self.hover));

                    // X icon
                    let icon_color = mix((SLATE_400), (SLATE_500), self.dark_mode);
                    sdf.move_to(c.x - 4., c.y - 4.);
                    sdf.line_to(c.x + 4., c.y + 4.);
                    sdf.stroke(icon_color, 1.5);
                    sdf.move_to(c.x + 4., c.y - 4.);
                    sdf.line_to(c.x - 4., c.y + 4.);
                    sdf.stroke(icon_color, 1.5);

                    return sdf.result;
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

    // ==========================================================================
    // SUCCESS TOAST
    // ==========================================================================
    pub ToastSuccess = <ToastBase> {
        toast_icon = {
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let green = (ACCENT_GREEN);

                    // Circle background
                    sdf.circle(c.x, c.y, 10.);
                    sdf.fill(green);

                    // Checkmark
                    sdf.move_to(c.x - 4., c.y);
                    sdf.line_to(c.x - 1., c.y + 3.);
                    sdf.line_to(c.x + 5., c.y - 3.);
                    sdf.stroke((WHITE), 2.0);

                    return sdf.result;
                }
            }
        }
    }

    // ==========================================================================
    // ERROR TOAST
    // ==========================================================================
    pub ToastError = <ToastBase> {
        toast_icon = {
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let red = (ACCENT_RED);

                    // Circle background
                    sdf.circle(c.x, c.y, 10.);
                    sdf.fill(red);

                    // X mark
                    sdf.move_to(c.x - 3., c.y - 3.);
                    sdf.line_to(c.x + 3., c.y + 3.);
                    sdf.stroke((WHITE), 2.0);
                    sdf.move_to(c.x + 3., c.y - 3.);
                    sdf.line_to(c.x - 3., c.y + 3.);
                    sdf.stroke((WHITE), 2.0);

                    return sdf.result;
                }
            }
        }
    }

    // ==========================================================================
    // WARNING TOAST
    // ==========================================================================
    pub ToastWarning = <ToastBase> {
        toast_icon = {
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let amber = (ACCENT_YELLOW);

                    // Triangle background
                    sdf.move_to(c.x, c.y - 9.);
                    sdf.line_to(c.x + 10., c.y + 8.);
                    sdf.line_to(c.x - 10., c.y + 8.);
                    sdf.close_path();
                    sdf.fill(amber);

                    // Exclamation mark
                    sdf.move_to(c.x, c.y - 4.);
                    sdf.line_to(c.x, c.y + 2.);
                    sdf.stroke((WHITE), 2.0);
                    sdf.circle(c.x, c.y + 5., 1.5);
                    sdf.fill((WHITE));

                    return sdf.result;
                }
            }
        }
    }

    // ==========================================================================
    // INFO TOAST
    // ==========================================================================
    pub ToastInfo = <ToastBase> {
        toast_icon = {
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let blue = (ACCENT_BLUE);

                    // Circle background
                    sdf.circle(c.x, c.y, 10.);
                    sdf.fill(blue);

                    // Info icon (i)
                    sdf.circle(c.x, c.y - 4., 1.5);
                    sdf.fill((WHITE));
                    sdf.move_to(c.x, c.y - 1.);
                    sdf.line_to(c.x, c.y + 5.);
                    sdf.stroke((WHITE), 2.0);

                    return sdf.result;
                }
            }
        }
    }

    // ==========================================================================
    // TOAST CONTAINER - Stack toasts from top-right
    // ==========================================================================
    pub ToastContainer = <View> {
        width: Fill, height: Fill
        flow: Overlay
        align: {x: 1.0, y: 0.0}
        padding: {top: 20, right: 20}

        toasts = <View> {
            width: Fit, height: Fit
            flow: Down
            spacing: 12
        }
    }

    // Toast container at bottom
    pub ToastContainerBottom = <View> {
        width: Fill, height: Fill
        flow: Overlay
        align: {x: 0.5, y: 1.0}
        padding: {bottom: 20}

        toasts = <View> {
            width: Fit, height: Fit
            flow: Down
            spacing: 12
        }
    }

    // ==========================================================================
    // SIMPLE TOAST - Compact single-line toast
    // ==========================================================================
    pub ToastSimple = <RoundedView> {
        width: Fit, height: Fit
        flow: Right
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        align: {y: 0.5}
        spacing: 10
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(2., 4., self.rect_size.x, self.rect_size.y, 8.0);
                let shadow = mix(vec4(0., 0., 0., 0.1), vec4(0., 0., 0., 0.25), self.dark_mode);
                sdf.fill(shadow);
                sdf.box(0., 0., self.rect_size.x - 2., self.rect_size.y - 2., 8.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        toast_message = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Success variant
    pub ToastSimpleSuccess = <ToastSimple> {
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let light_bg = #f0fdf4;  // green-50
                let dark_bg = (SLATE_800);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));
                let border = mix((ACCENT_GREEN), (ACCENT_GREEN_DARK), self.dark_mode);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        toast_message = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return mix(#166534, (ACCENT_GREEN_DARK), self.dark_mode);
                }
            }
        }
    }

    // Error variant
    pub ToastSimpleError = <ToastSimple> {
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let light_bg = #fef2f2;  // red-50
                let dark_bg = (SLATE_800);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));
                let border = mix((ACCENT_RED), (ACCENT_RED_DARK), self.dark_mode);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        toast_message = {
            draw_text: {
                fn get_color(self) -> vec4 {
                    return mix(#991b1b, (ACCENT_RED_DARK), self.dark_mode);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug)]
pub struct ToastMessage {
    pub toast_type: ToastType,
    pub title: String,
    pub message: String,
    pub duration_ms: u64,
}

impl Default for ToastMessage {
    fn default() -> Self {
        Self {
            toast_type: ToastType::Info,
            title: String::new(),
            message: String::new(),
            duration_ms: 5000,
        }
    }
}

impl ToastMessage {
    pub fn success(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Success,
            title: title.into(),
            message: message.into(),
            duration_ms: 5000,
        }
    }

    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Error,
            title: title.into(),
            message: message.into(),
            duration_ms: 5000,
        }
    }

    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Warning,
            title: title.into(),
            message: message.into(),
            duration_ms: 5000,
        }
    }

    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            toast_type: ToastType::Info,
            title: title.into(),
            message: message.into(),
            duration_ms: 5000,
        }
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }
}
