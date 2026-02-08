//! # Stats Chart Component
//!
//! Chart widgets for data visualization.
//!
//! ## Variants
//! - Bar chart: Vertical bars
//! - Horizontal bar chart: Horizontal bars
//! - Mini chart: Compact sparkline-style

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // BAR CHART - Vertical bars with labels
    // ==========================================================================
    pub BarChart = <View> {
        width: Fill, height: 200
        flow: Right
        align: {y: 1.0}
        spacing: 8
        padding: {bottom: 24}
    }

    // Single bar in chart
    pub ChartBar = <View> {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5, y: 1.0}
        spacing: 8

        bar = <RoundedView> {
            width: Fill, height: 100  // Set dynamically
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance value: 0.5  // 0 to 1
                instance border_radius: 4.0

                fn get_color(self) -> vec4 {
                    return mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                }
            }
        }

        bar_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Bar with value label on top
    pub ChartBarLabeled = <View> {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5, y: 1.0}
        spacing: 4

        value_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        bar = <RoundedView> {
            width: Fill, height: 100
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance border_radius: 4.0
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                }
            }
        }

        bar_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // HORIZONTAL BAR CHART
    // ==========================================================================
    pub HorizontalBarChart = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 12
    }

    // Single horizontal bar
    pub HorizontalChartBar = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 6

        label_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            bar_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <View> { width: Fill }

            value_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        bar_container = <View> {
            width: Fill, height: 8
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance progress: 0.5

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
                        let fill = mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                        sdf.fill(fill);
                    }

                    return sdf.result;
                }
            }
        }
    }

    // ==========================================================================
    // STAT CARD - Number with label
    // ==========================================================================
    pub StatCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        align: {x: 0.5}
        spacing: 4
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance tint_r: 1.0
            instance tint_g: 0.969
            instance tint_b: 0.929
            instance border_radius: 8.0

            fn get_color(self) -> vec4 {
                let light_bg = vec4(self.tint_r, self.tint_g, self.tint_b, 1.0);
                return mix(light_bg, (SLATE_700), self.dark_mode);
            }
        }

        stat_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                instance color_r: 0.976
                instance color_g: 0.451
                instance color_b: 0.086
                text_style: <FONT_BOLD>{ font_size: 24.0 }
                fn get_color(self) -> vec4 {
                    return vec4(self.color_r, self.color_g, self.color_b, 1.0);
                }
            }
        }

        stat_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Stat card with icon
    pub StatCardIcon = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Right
        align: {y: 0.5}
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        stat_icon = <View> {
            width: 48, height: 48
            align: {x: 0.5, y: 0.5}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                    let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(bg);
                    return sdf.result;
                }
            }

            icon = <Icon> {
                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return (ACCENT_PRIMARY);
                    }
                }
                icon_walk: {width: 24, height: 24}
            }
        }

        stat_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            stat_value = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 20.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            stat_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }

    // ==========================================================================
    // MINI SPARKLINE CHART
    // ==========================================================================
    pub Sparkline = <View> {
        width: 100, height: 32
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            // Data points would be passed as instance variables
            instance v0: 0.3
            instance v1: 0.5
            instance v2: 0.4
            instance v3: 0.8
            instance v4: 0.6
            instance v5: 0.9
            instance v6: 0.7

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let w = self.rect_size.x;
                let h = self.rect_size.y;
                let step_x = w / 6.;

                // Draw line connecting points
                let line_color = mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);

                let y0 = h * (1. - self.v0);
                let y1 = h * (1. - self.v1);
                let y2 = h * (1. - self.v2);
                let y3 = h * (1. - self.v3);
                let y4 = h * (1. - self.v4);
                let y5 = h * (1. - self.v5);
                let y6 = h * (1. - self.v6);

                sdf.move_to(0., y0);
                sdf.line_to(step_x * 1., y1);
                sdf.line_to(step_x * 2., y2);
                sdf.line_to(step_x * 3., y3);
                sdf.line_to(step_x * 4., y4);
                sdf.line_to(step_x * 5., y5);
                sdf.line_to(step_x * 6., y6);
                sdf.stroke(line_color, 2.0);

                return sdf.result;
            }
        }
    }

    // ==========================================================================
    // STATS ROW - Multiple stat cards in a row
    // ==========================================================================
    pub StatsRow = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 12
    }

    // Stats grid (2 columns)
    pub StatsGrid = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 12

        row1 = <StatsRow> {}
        row2 = <StatsRow> {}
    }

    // ==========================================================================
    // LEGEND ITEM
    // ==========================================================================
    pub LegendItem = <View> {
        width: Fit, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 8

        legend_dot = <View> {
            width: 12, height: 12
            show_bg: true
            draw_bg: {
                instance color_r: 0.976
                instance color_g: 0.451
                instance color_b: 0.086

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y, c.x - 1.);
                    let color = vec4(self.color_r, self.color_g, self.color_b, 1.0);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
        }

        legend_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Legend container
    pub ChartLegend = <View> {
        width: Fit, height: Fit
        flow: Right
        spacing: 16
    }
}
