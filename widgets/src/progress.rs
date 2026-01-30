//! # Progress Component
//!
//! Progress bar widgets with linear and circular variants.
//!
//! ## Variants
//! - Linear: Horizontal progress bar
//! - Circular: Ring/donut style progress

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // LINEAR PROGRESS BAR
    // ==========================================================================
    pub ProgressBar = <RoundedView> {
        width: Fill, height: 8
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.0  // 0.0 to 1.0
            instance color_r: 0.976  // ACCENT_PRIMARY default
            instance color_g: 0.451
            instance color_b: 0.086
            border_radius: 4.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background track
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                let track = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(track);

                // Progress fill
                let fill_width = self.rect_size.x * self.progress;
                if fill_width > 0. {
                    sdf.box(0., 0., fill_width, self.rect_size.y, 4.0);
                    let fill_color = vec4(self.color_r, self.color_g, self.color_b, 1.0);
                    sdf.fill(fill_color);
                }

                return sdf.result;
            }
        }
    }

    // Smaller progress bar
    pub ProgressBarSmall = <ProgressBar> {
        height: 4
        draw_bg: { border_radius: 2.0 }
    }

    // Larger progress bar
    pub ProgressBarLarge = <ProgressBar> {
        height: 12
        draw_bg: { border_radius: 6.0 }
    }

    // Progress bar with label
    pub ProgressBarLabeled = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 8

        label_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            progress_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <View> { width: Fill }

            progress_value = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        bar = <ProgressBar> {}
    }

    // ==========================================================================
    // CIRCULAR PROGRESS
    // ==========================================================================
    pub ProgressCircle = <View> {
        width: 64, height: 64
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.0  // 0.0 to 1.0
            instance track_width: 6.0
            instance color_r: 0.976
            instance color_g: 0.451
            instance color_b: 0.086

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let radius = c.x - self.track_width * 0.5 - 2.;

                // Background track
                sdf.circle(c.x, c.y, radius);
                let track = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(track, self.track_width);

                // Progress arc
                if self.progress > 0. {
                    let angle = self.progress * 6.28318530718;  // 2 * PI
                    let start_angle = -1.5707963267;  // -PI/2 (start from top)

                    // Draw arc segments
                    let segments = 64.;
                    let step = angle / segments;
                    let fill_color = vec4(self.color_r, self.color_g, self.color_b, 1.0);

                    // Simplified arc drawing using circle approximation
                    let end_angle = start_angle + angle;
                    let ex = c.x + cos(end_angle) * radius;
                    let ey = c.y + sin(end_angle) * radius;

                    // Arc visualization using stroke
                    sdf.move_to(c.x, c.y - radius);
                    for i in 0..64 {
                        let t = (i as f32) / 64.;
                        if t <= self.progress {
                            let a = start_angle + t * 6.28318530718;
                            let x = c.x + cos(a) * radius;
                            let y = c.y + sin(a) * radius;
                            sdf.line_to(x, y);
                        }
                    }
                    sdf.stroke(fill_color, self.track_width);
                }

                return sdf.result;
            }
        }
    }

    // Small circular progress
    pub ProgressCircleSmall = <ProgressCircle> {
        width: 40, height: 40
        draw_bg: { track_width: 4.0 }
    }

    // Large circular progress
    pub ProgressCircleLarge = <ProgressCircle> {
        width: 96, height: 96
        draw_bg: { track_width: 8.0 }
    }

    // Circular progress with label in center
    pub ProgressCircleLabeled = <View> {
        width: 80, height: 80
        flow: Overlay
        align: {x: 0.5, y: 0.5}

        circle = <ProgressCircle> {
            width: Fill, height: Fill
        }

        progress_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 16.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // INDETERMINATE PROGRESS (Spinner)
    // ==========================================================================
    pub ProgressSpinner = <View> {
        width: 32, height: 32
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance rotation: 0.0  // Animated rotation value
            instance color_r: 0.976
            instance color_g: 0.451
            instance color_b: 0.086

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let radius = c.x - 4.;

                // Draw spinning arc
                let fill_color = vec4(self.color_r, self.color_g, self.color_b, 1.0);
                let track = mix((SLATE_200), (SLATE_700), self.dark_mode);

                // Background track
                sdf.circle(c.x, c.y, radius);
                sdf.stroke(track, 3.0);

                // Animated arc (quarter circle)
                let start = self.rotation;
                let arc_len = 1.5707963267;  // PI/2

                sdf.move_to(c.x + cos(start) * radius, c.y + sin(start) * radius);
                for i in 0..16 {
                    let t = (i as f32) / 16.;
                    let a = start + t * arc_len;
                    sdf.line_to(c.x + cos(a) * radius, c.y + sin(a) * radius);
                }
                sdf.stroke(fill_color, 3.0);

                return sdf.result;
            }
        }
    }

    pub ProgressSpinnerSmall = <ProgressSpinner> {
        width: 20, height: 20
    }

    pub ProgressSpinnerLarge = <ProgressSpinner> {
        width: 48, height: 48
    }

    // ==========================================================================
    // STEP PROGRESS (Multi-step indicator)
    // ==========================================================================
    pub ProgressStep = <View> {
        width: 32, height: 32
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance state: 0.0  // 0 = pending, 1 = active, 2 = completed

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Circle
                sdf.circle(c.x, c.y, 14.);

                let pending = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let active = (ACCENT_PRIMARY);
                let completed = (ACCENT_GREEN);

                let is_active = step(0.5, self.state) * (1. - step(1.5, self.state));
                let is_completed = step(1.5, self.state);

                let color = mix(pending, active, is_active);
                let color = mix(color, completed, is_completed);

                sdf.fill(color);

                // Checkmark for completed
                if self.state > 1.5 {
                    sdf.move_to(c.x - 5., c.y);
                    sdf.line_to(c.x - 1., c.y + 4.);
                    sdf.line_to(c.x + 6., c.y - 4.);
                    sdf.stroke((WHITE), 2.0);
                }

                return sdf.result;
            }
        }
    }

    // Step connector line
    pub ProgressStepConnector = <View> {
        width: 40, height: 4
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance completed: 0.0

            fn pixel(self) -> vec4 {
                let pending = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let done = (ACCENT_GREEN);
                return mix(pending, done, self.completed);
            }
        }
    }

    // Complete step progress row
    pub ProgressSteps = <View> {
        width: Fit, height: Fit
        flow: Right
        align: {y: 0.5}
    }
}
