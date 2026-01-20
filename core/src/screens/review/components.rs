use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Orange theme accent colors
    ACCENT_ORANGE = #f97316
    ACCENT_ORANGE_HOVER = #ea580c

    pub CardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }
    }

    pub PanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }
    }

    pub MutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    pub SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    pub ReviewTabButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 14, right: 14, top: 8, bottom: 8}
        margin: {right: 8}
        draw_bg: {
            instance dark_mode: 0.0
            instance selected: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                let normal = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let active = mix((ACCENT_ORANGE), (ACCENT_ORANGE), self.dark_mode);
                sdf.fill(mix(normal, active, self.selected));
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance selected: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                let normal = mix((SLATE_600), (SLATE_200), self.dark_mode);
                return mix(normal, (WHITE), self.selected);
            }
        }
    }

    pub ReviewActionButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 12, bottom: 12}
        draw_bg: {
            instance dark_mode: 0.0
            instance tint: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                let base = mix((SLATE_50), (SLATE_800), self.dark_mode);
                let tint = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(mix(base, tint, self.tint));
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }
    }

    pub MiniStat = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        align: {x: 0.5}
        stat_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
        stat_label = <MutedText> {}
    }

    pub CalendarCell = <RoundedView> {
        width: 14, height: 14
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0
            instance done: 0.0
            border_radius: 3.0
            fn get_color(self) -> vec4 {
                let normal = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let done = mix((ACCENT_GREEN), (ACCENT_GREEN), self.dark_mode);
                let active = mix((ACCENT_ORANGE), (ACCENT_ORANGE), self.dark_mode);
                return mix(mix(normal, done, self.done), active, self.active);
            }
        }
    }

    pub ProgressBar = <RoundedView> {
        width: 200, height: 8
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.7
            border_radius: 4.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                let bg = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(bg);
                let fill_w = self.rect_size.x * self.progress;
                sdf.box(0., 0., fill_w, self.rect_size.y, 4.0);
                let fill = mix((ACCENT_GREEN), (ACCENT_ORANGE), self.dark_mode);
                sdf.fill(fill);
                return sdf.result;
            }
        }
    }

    pub PillButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 12, right: 12, top: 8, bottom: 8}
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 16.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_600), (SLATE_200), self.dark_mode);
            }
        }
    }

    pub SecondaryButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 12, right: 12, top: 8, bottom: 8}
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 10.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                let bg = mix((SLATE_50), (SLATE_800), self.dark_mode);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }
    }

    pub PrimaryButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 14, right: 14, top: 9, bottom: 9}
        draw_bg: {
            border_radius: 10.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                sdf.fill((ACCENT_ORANGE));
                return sdf.result;
            }
        }
        draw_text: {
            text_style: <FONT_BOLD>{ font_size: 11.0 }
            color: (WHITE)
        }
    }

    // Stats panel for top of review page
    pub StatCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        align: {x: 0.5}
        flow: Down
        spacing: 4
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance tint: #fff5eb  // default orange-50
            border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix(self.tint, (SLATE_700), self.dark_mode);
            }
        }
        stat_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                instance tint: #f97316  // default orange-600
                text_style: <FONT_BOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    return mix(self.tint, self.tint, self.dark_mode);
                }
            }
        }
        stat_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Color constants for stats
    STAT_ORANGE_BG = #fff5eb
    STAT_ORANGE_TEXT = #f97316
    STAT_GREEN_BG = #f0fdf4
    STAT_GREEN_TEXT = #16a34a
    STAT_RED_BG = #fef2f2
    STAT_RED_TEXT = #dc2626
    STAT_BLUE_BG = #eff6ff
    STAT_BLUE_TEXT = #2563DD

    pub StatCardOrange = <StatCard> {
        draw_bg: { tint: (STAT_ORANGE_BG) }
        stat_value = { draw_text: { tint: (STAT_ORANGE_TEXT) } }
    }

    pub StatCardGreen = <StatCard> {
        draw_bg: { tint: (STAT_GREEN_BG) }
        stat_value = { draw_text: { tint: (STAT_GREEN_TEXT) } }
    }

    pub StatCardRed = <StatCard> {
        draw_bg: { tint: (STAT_RED_BG) }
        stat_value = { draw_text: { tint: (STAT_RED_TEXT) } }
    }

    pub StatCardBlue = <StatCard> {
        draw_bg: { tint: (STAT_BLUE_BG) }
        stat_value = { draw_text: { tint: (STAT_BLUE_TEXT) } }
    }

    pub StatsPanel = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 12
        stat_due = <StatCardOrange> {
            stat_value = { text: "23" }
            stat_label = { text: "待复习" }
        }
        stat_mastered = <StatCardGreen> {
            stat_value = { text: "156" }
            stat_label = { text: "已掌握" }
        }
        stat_mistakes = <StatCardRed> {
            stat_value = { text: "8" }
            stat_label = { text: "易错点" }
        }
        stat_accuracy = <StatCardBlue> {
            stat_value = { text: "85%" }
            stat_label = { text: "正确率" }
        }
    }

    // Word card component matching website design
    pub WordCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 8
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        header_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            word_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }
            <View> { width: Fill }
            due_badge = <RoundedView> {
                visible: false
                width: Fit, height: Fit
                padding: {left: 8, right: 8, top: 3, bottom: 3}
                show_bg: true
                draw_bg: {
                    instance urgent: 1.0
                    border_radius: 10.0
                    fn get_color(self) -> vec4 {
                        let urgent_bg = #fef2f2;
                        let normal_bg = #fffbeb;
                        return mix(normal_bg, urgent_bg, self.urgent);
                    }
                }
                due_text = <Label> {
                    draw_text: {
                        instance urgent: 1.0
                        text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                        fn get_color(self) -> vec4 {
                            let urgent_color = #dc2626;
                            let normal_color = #d97706;
                            return mix(normal_color, urgent_color, self.urgent);
                        }
                    }
                }
            }
        }
        hint_label = <Label> {
            text: "点击查看释义"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
        progress_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            spacing: 8
            margin: {top: 4}
            progress_bar = <RoundedView> {
                width: Fill, height: 8
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    instance progress: 0.6
                    border_radius: 4.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let bg = mix((SLATE_200), (SLATE_700), self.dark_mode);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        sdf.fill(bg);
                        let fill_w = self.rect_size.x * self.progress;
                        sdf.box(0., 0., fill_w, self.rect_size.y, 4.0);
                        let fill_color = mix(#ef4444, mix(#f59e0b, #22c55e, step(0.5, self.progress)), step(0.3, self.progress));
                        sdf.fill(fill_color);
                        return sdf.result;
                    }
                }
            }
            progress_label = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }

    // Color constants for tip banners
    pub TIP_AMBER_BG = #fffbDD
    pub TIP_AMBER_TEXT = #924000
    pub TIP_GREEN_BG = #f0fdf4
    pub TIP_GREEN_TEXT = #166534

    // Tip banner for mistakes/mastered tabs
    pub TipBanner = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Right
        align: {y: 0.5}
        spacing: 8
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance tint: (TIP_AMBER_BG)
            border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix(self.tint, (SLATE_700), self.dark_mode);
            }
        }
        tip_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                instance tint: (TIP_AMBER_TEXT)
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix(self.tint, (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Bar chart component for stats
    pub WeekBarChart = <View> {
        width: Fill, height: 128
        flow: Right
        align: {y: 1.0}
        spacing: 8
    }

    pub ChartBar = <View> {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5, y: 1.0}
        spacing: 4
        bar = <RoundedView> {
            width: Fill, height: 60
            show_bg: true
            draw_bg: {
                border_radius: 4.0
                fn get_color(self) -> vec4 {
                    return #fb923c;  // orange-400
                }
            }
        }
        day_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Stat overview item for stats tab
    pub StatOverviewItem = <PanelBase> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 4
        item_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
        item_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }
}
