//! # Dialog Component
//!
//! Modal dialog widget with animations and backdrop.
//!
//! ## Features
//! - Centered modal with backdrop
//! - Open/close animations
//! - Header, content, and footer sections
//! - Configurable sizes

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // DIALOG BACKDROP - Semi-transparent overlay
    // ==========================================================================
    pub DialogBackdrop = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance opacity: 0.0
            fn pixel(self) -> vec4 {
                return vec4(0., 0., 0., 0.5 * self.opacity);
            }
        }
    }

    // ==========================================================================
    // DIALOG CONTAINER - Main dialog box
    // ==========================================================================
    pub DialogContainer = <RoundedView> {
        width: 480, height: Fit
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance scale: 1.0
            instance border_radius: 16.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow
                sdf.box(4., 6., self.rect_size.x - 4., self.rect_size.y - 4., 16.0);
                let shadow = mix(vec4(0., 0., 0., 0.15), vec4(0., 0., 0., 0.3), self.dark_mode);
                sdf.fill(shadow);

                // Main dialog
                sdf.box(0., 0., self.rect_size.x - 4., self.rect_size.y - 4., 16.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);

                return sdf.result;
            }
        }
    }

    // Small dialog (320px)
    pub DialogContainerSmall = <DialogContainer> {
        width: 320
    }

    // Large dialog (640px)
    pub DialogContainerLarge = <DialogContainer> {
        width: 640
    }

    // Full-width dialog (with margins)
    pub DialogContainerFull = <DialogContainer> {
        width: Fill
        margin: {left: 40, right: 40}
    }

    // ==========================================================================
    // DIALOG HEADER - Title and close button
    // ==========================================================================
    pub DialogHeader = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {left: 24, right: 16, top: 20, bottom: 16}
        spacing: 12

        dialog_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        close_btn = <Button> {
            width: 32, height: 32
            padding: 0
            align: {x: 0.5, y: 0.5}
            text: ""
            draw_bg: {
                instance hover: 0.0
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y, 14.0);
                    let hover_bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(mix((TRANSPARENT), hover_bg, self.hover));

                    // X icon
                    let icon_color = mix((SLATE_500), (SLATE_400), self.dark_mode);
                    sdf.move_to(c.x - 5., c.y - 5.);
                    sdf.line_to(c.x + 5., c.y + 5.);
                    sdf.stroke(icon_color, 1.5);
                    sdf.move_to(c.x + 5., c.y - 5.);
                    sdf.line_to(c.x - 5., c.y + 5.);
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

    // Header without close button
    pub DialogHeaderSimple = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: {left: 24, right: 24, top: 24, bottom: 16}
        spacing: 8

        dialog_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        dialog_description = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // DIALOG CONTENT - Main content area
    // ==========================================================================
    pub DialogContent = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: {left: 24, right: 24, top: 0, bottom: 24}
    }

    // Scrollable content for long dialogs
    pub DialogContentScroll = <ScrollYView> {
        width: Fill, height: 300
        padding: {left: 24, right: 24, top: 0, bottom: 24}
    }

    // ==========================================================================
    // DIALOG FOOTER - Action buttons
    // ==========================================================================
    pub DialogFooter = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {x: 1.0, y: 0.5}
        padding: {left: 24, right: 24, top: 16, bottom: 24}
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Top border
                sdf.rect(0., 0., self.rect_size.x, 1.0);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(border);
                return sdf.result;
            }
        }
    }

    // Footer with full-width buttons stacked
    pub DialogFooterStacked = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: {left: 24, right: 24, top: 16, bottom: 24}
        spacing: 8
    }

    // ==========================================================================
    // COMPLETE DIALOG WRAPPER
    // ==========================================================================
    pub Dialog = {{Dialog}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false
        align: {x: 0.5, y: 0.5}

        backdrop = <DialogBackdrop> {}

        container = <DialogContainer> {
            header = <DialogHeader> {}
            content = <DialogContent> {}
            footer = <DialogFooter> {}
        }
    }

    // Alert dialog (simple confirmation)
    pub AlertDialog = <Dialog> {
        container = <DialogContainerSmall> {
            header = <DialogHeaderSimple> {
                dialog_title = { text: "Alert" }
                dialog_description = { text: "Are you sure you want to proceed?" }
            }
            footer = <DialogFooter> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Dialog {
    #[deref]
    view: View,
    #[rust]
    is_open: bool,
    #[rust]
    animation_progress: f64,
    #[rust]
    animating: bool,
    #[rust]
    animation_start: f64,
}

impl Widget for Dialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Handle animation
        if self.animating {
            if let Event::NextFrame(_) = event {
                self.update_animation(cx);
            }
        }

        // Handle backdrop click to close
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Close button click
        if self.view.button(ids!(container.header.close_btn)).clicked(actions) {
            self.close(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Dialog {
    pub fn open(&mut self, cx: &mut Cx) {
        if self.is_open {
            return;
        }
        self.is_open = true;
        self.animating = true;
        self.animation_start = Cx::time_now();
        self.view.set_visible(cx, true);
        self.view.redraw(cx);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        if !self.is_open {
            return;
        }
        self.is_open = false;
        self.animating = true;
        self.animation_start = Cx::time_now();
        self.view.redraw(cx);
    }

    fn update_animation(&mut self, cx: &mut Cx) {
        const DURATION: f64 = 0.2;
        let elapsed = Cx::time_now() - self.animation_start;
        let t = (elapsed / DURATION).min(1.0);

        // Ease-out cubic
        let eased = 1.0 - (1.0 - t).powi(3);

        if self.is_open {
            self.animation_progress = eased;
        } else {
            self.animation_progress = 1.0 - eased;
        }

        // Apply animation values
        self.view.view(ids!(backdrop)).apply_over(
            cx,
            live! { draw_bg: { opacity: (self.animation_progress) } },
        );

        let scale = 0.95 + 0.05 * self.animation_progress;
        self.view.view(ids!(container)).apply_over(
            cx,
            live! { draw_bg: { scale: (scale) } },
        );

        if t >= 1.0 {
            self.animating = false;
            if !self.is_open {
                self.view.set_visible(cx, false);
            }
        }

        self.view.redraw(cx);
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl DialogRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open()
        } else {
            false
        }
    }
}
