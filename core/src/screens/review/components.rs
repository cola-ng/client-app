use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_component::*;

    use colang_widgets::theme::*;

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
                let active = mix((ACCENT_INDIGO), (ACCENT_INDIGO), self.dark_mode);
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
                let active = mix((ACCENT_INDIGO), (ACCENT_INDIGO), self.dark_mode);
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
                let fill = mix((ACCENT_GREEN), (ACCENT_INDIGO), self.dark_mode);
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
                sdf.fill((ACCENT_INDIGO));
                return sdf.result;
            }
        }
        draw_text: {
            text_style: <FONT_BOLD>{ font_size: 11.0 }
            color: (WHITE)
        }
    }
}
