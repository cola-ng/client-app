//! # Form Components
//!
//! Form widgets including text inputs, textareas, labels, and error messages.
//!
//! ## Components
//! - FormTextInput: Single-line text input with label
//! - FormTextarea: Multi-line text input
//! - FormLabel: Field label
//! - FormError: Error message display
//! - FormGroup: Container for form fields

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // FORM LABEL
    // ==========================================================================
    pub FormLabel = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            instance required: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }

            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_300), self.dark_mode);
            }
        }
    }

    // Label with required indicator
    pub FormLabelRequired = <View> {
        width: Fit, height: Fit
        flow: Right
        spacing: 2

        label_text = <FormLabel> {}

        required_indicator = <Label> {
            text: "*"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (ACCENT_RED)
            }
        }
    }

    // ==========================================================================
    // FORM TEXT INPUT
    // ==========================================================================
    pub FormTextInput = <TextInput> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance error: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                // Background
                let light_bg = (WHITE);
                let dark_bg = (SLATE_800);
                let bg = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg);

                // Border
                let light_border = (SLATE_300);
                let dark_border = (SLATE_600);
                let focus_border = (ACCENT_PRIMARY);
                let error_border = (ACCENT_RED);

                let normal_border = mix(light_border, dark_border, self.dark_mode);
                let active_border = mix(normal_border, focus_border, self.focus);
                let final_border = mix(active_border, error_border, self.error);

                sdf.stroke(final_border, 1.0);

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

        draw_selection: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix(vec4(0.25, 0.51, 0.96, 0.3), vec4(0.4, 0.6, 1.0, 0.4), self.dark_mode);
            }
        }

        draw_cursor: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Text input with icon on left
    pub FormTextInputIcon = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance error: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_bg = (WHITE);
                let dark_bg = (SLATE_800);
                let bg = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg);

                let light_border = (SLATE_300);
                let dark_border = (SLATE_600);
                let focus_border = (ACCENT_PRIMARY);
                let error_border = (ACCENT_RED);

                let normal_border = mix(light_border, dark_border, self.dark_mode);
                let active_border = mix(normal_border, focus_border, self.focus);
                let final_border = mix(active_border, error_border, self.error);

                sdf.stroke(final_border, 1.0);
                return sdf.result;
            }
        }

        input_icon = <Icon> {
            draw_icon: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix((SLATE_400), (SLATE_500), self.dark_mode);
                }
            }
            icon_walk: {width: 18, height: 18, margin: {right: 10}}
        }

        text_input = <TextInput> {
            width: Fill, height: Fit
            padding: 0
            draw_bg: { fn pixel(self) -> vec4 { return (TRANSPARENT); } }
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // FORM TEXTAREA
    // ==========================================================================
    pub FormTextarea = <TextInput> {
        width: Fill, height: 120
        padding: {left: 12, right: 12, top: 10, bottom: 10}

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance error: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_bg = (WHITE);
                let dark_bg = (SLATE_800);
                let bg = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg);

                let light_border = (SLATE_300);
                let dark_border = (SLATE_600);
                let focus_border = (ACCENT_PRIMARY);
                let error_border = (ACCENT_RED);

                let normal_border = mix(light_border, dark_border, self.dark_mode);
                let active_border = mix(normal_border, focus_border, self.focus);
                let final_border = mix(active_border, error_border, self.error);

                sdf.stroke(final_border, 1.0);
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

    // ==========================================================================
    // FORM HELPER / ERROR TEXT
    // ==========================================================================
    pub FormHelperText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }

            fn get_color(self) -> vec4 {
                return mix((SLATE_500), (SLATE_400), self.dark_mode);
            }
        }
    }

    pub FormErrorText = <Label> {
        draw_text: {
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            color: (ACCENT_RED)
        }
    }

    // Error text with icon
    pub FormErrorWithIcon = <View> {
        width: Fit, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 6

        error_icon = <View> {
            width: 14, height: 14
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y, c.x - 1.);
                    sdf.fill((ACCENT_RED));
                    // Exclamation mark
                    sdf.move_to(c.x, c.y - 3.);
                    sdf.line_to(c.x, c.y + 1.);
                    sdf.stroke((WHITE), 1.5);
                    sdf.circle(c.x, c.y + 4., 1.);
                    sdf.fill((WHITE));
                    return sdf.result;
                }
            }
        }

        error_text = <FormErrorText> {}
    }

    // ==========================================================================
    // FORM GROUP - Container for a field
    // ==========================================================================
    pub FormGroup = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 8

        label = <FormLabel> {}
        input = <FormTextInput> {}
    }

    // Form group with required indicator
    pub FormGroupRequired = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 8

        label = <FormLabelRequired> {}
        input = <FormTextInput> {}
    }

    // Form group with error state
    pub FormGroupError = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 8

        label = <FormLabel> {}
        input = <FormTextInput> { draw_bg: { error: 1.0 } }
        error = <FormErrorText> {}
    }

    // ==========================================================================
    // FORM SELECT (Dropdown style)
    // ==========================================================================
    pub FormSelect = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        align: {y: 0.5}
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_bg = (WHITE);
                let dark_bg = (SLATE_800);
                let bg = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg);

                let light_border = (SLATE_300);
                let dark_border = (SLATE_600);
                let focus_border = (ACCENT_PRIMARY);

                let normal_border = mix(light_border, dark_border, self.dark_mode);
                let final_border = mix(normal_border, focus_border, self.focus);

                sdf.stroke(final_border, 1.0);
                return sdf.result;
            }
        }

        select_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        chevron = <View> {
            width: 18, height: 18
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let icon_color = mix((SLATE_400), (SLATE_500), self.dark_mode);
                    // Chevron down
                    sdf.move_to(c.x - 4., c.y - 2.);
                    sdf.line_to(c.x, c.y + 2.);
                    sdf.line_to(c.x + 4., c.y - 2.);
                    sdf.stroke(icon_color, 1.5);
                    return sdf.result;
                }
            }
        }
    }

    // ==========================================================================
    // CHECKBOX
    // ==========================================================================
    pub FormCheckbox = <View> {
        width: Fit, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 10
        cursor: Hand

        checkbox = <View> {
            width: 20, height: 20
            show_bg: true
            draw_bg: {
                instance hover: 0.0
                instance checked: 0.0
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);

                    let unchecked_bg = mix((WHITE), (SLATE_700), self.dark_mode);
                    let checked_bg = (ACCENT_PRIMARY);
                    let bg = mix(unchecked_bg, checked_bg, self.checked);

                    let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                    let checked_border = (ACCENT_PRIMARY);
                    let final_border = mix(border, checked_border, self.checked);

                    sdf.fill(bg);
                    sdf.stroke(final_border, 1.0);

                    // Checkmark
                    if self.checked > 0.5 {
                        sdf.move_to(c.x - 4., c.y);
                        sdf.line_to(c.x - 1., c.y + 3.);
                        sdf.line_to(c.x + 5., c.y - 3.);
                        sdf.stroke((WHITE), 2.0);
                    }

                    return sdf.result;
                }
            }
        }

        checkbox_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // RADIO BUTTON
    // ==========================================================================
    pub FormRadio = <View> {
        width: Fit, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 10
        cursor: Hand

        radio = <View> {
            width: 20, height: 20
            show_bg: true
            draw_bg: {
                instance hover: 0.0
                instance selected: 0.0
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Outer circle
                    sdf.circle(c.x, c.y, c.x - 1.);

                    let unchecked_bg = mix((WHITE), (SLATE_700), self.dark_mode);
                    let checked_border = (ACCENT_PRIMARY);
                    let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                    let final_border = mix(border, checked_border, self.selected);

                    sdf.fill(unchecked_bg);
                    sdf.stroke(final_border, 1.5);

                    // Inner dot
                    if self.selected > 0.5 {
                        sdf.circle(c.x, c.y, 5.);
                        sdf.fill((ACCENT_PRIMARY));
                    }

                    return sdf.result;
                }
            }
        }

        radio_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // SWITCH / TOGGLE
    // ==========================================================================
    pub FormSwitch = <View> {
        width: 44, height: 24
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance on: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Track
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                let off_track = mix((SLATE_300), (SLATE_600), self.dark_mode);
                let on_track = (ACCENT_PRIMARY);
                let track_color = mix(off_track, on_track, self.on);
                sdf.fill(track_color);

                // Thumb
                let thumb_x = mix(12., self.rect_size.x - 12., self.on);
                sdf.circle(thumb_x, 12., 9.);
                sdf.fill((WHITE));

                return sdf.result;
            }
        }
    }
}
