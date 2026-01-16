//! About panel - app info, version, and links

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;
    use link::shaders::*;

    use widgets::theme::*;

    use crate::screens::settings::general_panel::SettingsButton;

    // Link-style clickable label
    pub LinkLabel = <Button> {
        width: Fit, height: Fit
        padding: 0

        draw_bg: {
            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }

        draw_text: {
            instance hover: 0.0
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }

            fn get_color(self) -> vec4 {
                let normal = mix((ACCENT_BLUE), (ACCENT_BLUE_DARK), self.dark_mode);
                let hover_color = mix((BLUE_600), (BLUE_400), self.dark_mode);
                return mix(normal, hover_color, self.hover);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_text: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_text: {hover: 1.0}} }
            }
        }
    }

    // Primary action button
    pub PrimaryButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 8, bottom: 8}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let normal = (ACCENT_BLUE);
                let hover_color = (BLUE_600);
                let bg = mix(normal, hover_color, self.hover);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            fn get_color(self) -> vec4 { return (WHITE); }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    // ========================================================================
    // About Tab Content
    // ========================================================================

    pub AboutTab = <View> {
        width: Fill, height: Fill
        flow: Down
        padding: 24
        align: {x: 0.5}

        <View> { width: Fill, height: Fill }

        // Logo
        logo_container = <View> {
            width: Fit, height: Fit
            align: {x: 0.5, y: 0.5}

            <RoundedView> {
                width: 80, height: 80
                show_bg: true
                draw_bg: {
                    color: (ACCENT_BLUE)
                    border_radius: 16.0
                }
                align: {x: 0.5, y: 0.5}

                <Label> {
                    text: "开"
                    draw_text: {
                        text_style: <FONT_BOLD>{ font_size: 36.0 }
                        fn get_color(self) -> vec4 { return (WHITE); }
                    }
                }
            }
        }

        // App name and version
        <View> {
            width: Fit, height: Fit
            flow: Down
            margin: {top: 16}
            align: {x: 0.5}

            <Label> {
                text: "开朗英语"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 20.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            version_label = <Label> {
                text: "Version 1.0.0"
                margin: {top: 4}
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Action buttons
        <View> {
            width: Fit, height: Fit
            flow: Down
            margin: {top: 24}
            spacing: 12
            align: {x: 0.5}

            check_update_btn = <PrimaryButton> {
                width: 200
                text: "Check for Updates"
            }

            release_notes_btn = <SettingsButton> {
                width: 200
                text: "Release Notes"
            }

            feedback_btn = <SettingsButton> {
                width: 200
                text: "Send Feedback"
            }
        }

        <View> { width: Fill, height: Fill }

        // Footer links
        footer = <View> {
            width: Fill, height: Fit
            flow: Down
            align: {x: 0.5}

            links_row = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: 8
                align: {x: 0.5}

                terms_link = <LinkLabel> {
                    text: "Service Agreement"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((ACCENT_BLUE), (ACCENT_BLUE_DARK), self.dark_mode);
                        }
                    }
                }

                <Label> {
                    text: "|"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                        }
                    }
                }

                privacy_link = <LinkLabel> {
                    text: "Privacy Policy"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((ACCENT_BLUE), (ACCENT_BLUE_DARK), self.dark_mode);
                        }
                    }
                }
            }

            copyright = <Label> {
                text: "© 2026 Colang English. All rights reserved."
                margin: {top: 8}
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                    }
                }
            }
        }
    }
}
