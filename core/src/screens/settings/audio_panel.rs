//! Audio settings panel - microphone and speaker configuration

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;
    use link::shaders::*;

    use ::widgets::theme::*;
    use makepad_component::widgets::*;
    use makepad_component::theme::colors::*;

    use crate::screens::settings::general_panel::SectionTitle;
    use crate::screens::settings::general_panel::SettingsRow;
    use crate::screens::settings::general_panel::SettingsLabel;
    use crate::screens::settings::general_panel::SettingsButton;
    use crate::screens::settings::general_panel::HDivider;

    // Audio device dropdown
    pub AudioDeviceDropdown = <DropDown> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        popup_menu_position: BelowInput
        labels: ["Default Device"]
        values: []
        selected_item: 0
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
        popup_menu: {
            width: 300
            draw_bg: {
                instance dark_mode: 0.0
                border_size: 1.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                    let border = mix((BORDER), (SLATE_600), self.dark_mode);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                    sdf.fill(bg);
                    sdf.stroke(border, self.border_size);
                    return sdf.result;
                }
            }
            menu_item: {
                width: Fill
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let base = mix((WHITE), (SLATE_800), self.dark_mode);
                        let hover_color = mix((GRAY_100), (SLATE_700), self.dark_mode);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(base, hover_color, self.hover));
                        return sdf.result;
                    }
                }
                draw_text: {
                    instance dark_mode: 0.0
                    fn get_color(self) -> vec4 {
                        let base = mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        return base;
                    }
                }
            }
        }
    }

    // ========================================================================
    // Audio Tab Content
    // ========================================================================

    pub AudioTab = <View> {
        width: Fill, height: Fill
        flow: Down
        padding: 24

        <Label> {
            text: "Audio"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        // Speaker section
        speaker_section = <View> {
            width: Fill, height: Fit
            flow: Down
            margin: {top: 20}

            <SectionTitle> { text: "Speaker" }

            <SettingsRow> {
                <SettingsLabel> { text: "Output Device" width: 120 }
                speaker_device = <AudioDeviceDropdown> {}
                speaker_test_btn = <SettingsButton> { text: "Test" }
            }

            <SettingsRow> {
                <SettingsLabel> { text: "Volume" width: 120 }
                speaker_volume = <MpSlider> {
                    width: Fill, height: Fit
                    min: 0.0, max: 100.0
                    step: 1.0
                }
                speaker_volume_label = <Label> {
                    width: 40
                    text: "80%"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                        }
                    }
                }
            }
        }

        <HDivider> {}

        // Microphone section
        mic_section = <View> {
            width: Fill, height: Fit
            flow: Down

            <SectionTitle> { text: "Microphone" }

            <SettingsRow> {
                <SettingsLabel> { text: "Input Device" width: 120 }
                mic_device = <AudioDeviceDropdown> {}
                mic_test_btn = <SettingsButton> { text: "Test" }
            }

            <SettingsRow> {
                <SettingsLabel> { text: "Input Volume" width: 120 }
                mic_volume = <MpSlider> {
                    width: Fill, height: Fit
                    min: 0.0, max: 100.0
                    step: 1.0
                }
                mic_volume_label = <Label> {
                    width: 40
                    text: "75%"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                        }
                    }
                }
            }

            // Mic level meter
            <SettingsRow> {
                <SettingsLabel> { text: "Input Level" width: 120 }
                mic_level_meter = <View> {
                    width: Fill, height: 8
                    show_bg: true
                    draw_bg: {
                        instance level: 0.3
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            // Background
                            let bg = mix((SLATE_200), (SLATE_700), self.dark_mode);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                            sdf.fill(bg);
                            // Level indicator
                            let level_width = self.rect_size.x * self.level;
                            sdf.box(0., 0., level_width, self.rect_size.y, 4.0);
                            sdf.fill((GREEN_500));
                            return sdf.result;
                        }
                    }
                }
                <View> { width: 40, height: Fit }
            }
        }

        <View> { width: Fill, height: Fill }
    }
}

/// Audio device information
pub struct AudioDevices {
    pub input_devices: Vec<String>,
    pub output_devices: Vec<String>,
    pub input_labels: Vec<String>,
    pub output_labels: Vec<String>,
}
