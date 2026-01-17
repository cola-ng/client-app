//! General settings panel - startup, appearance, and storage options

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;

    use colang_widgets::theme::*;
    use makepad_component::widgets::radio::*;

    // Reusable components for settings panels
    pub SectionTitle = <Label> {
        margin: {top: 16, bottom: 8}
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_BOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    pub SettingsRow = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 12
        padding: {top: 8, bottom: 8}
    }

    pub SettingsLabel = <Label> {
        width: Fit
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    pub HDivider = <View> {
        width: Fill, height: 1
        margin: {top: 16, bottom: 8}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((BORDER), (SLATE_700), self.dark_mode);
            }
        }
    }

    // Standard button style
    pub SettingsButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 16, right: 16, top: 8, bottom: 8}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let light_normal = (SLATE_100);
                let light_hover = (SLATE_200);
                let dark_normal = (SLATE_700);
                let dark_hover = (SLATE_600);

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let bg = mix(normal, hover_color, self.hover);

                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
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

    // Simple checkbox style for settings
    pub SettingsCheckBox = <CheckBox> {
        width: Fit, height: Fit
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Simple radio button style for settings
    pub SettingsRadioButton = <RadioButton> {
        width: Fit, height: Fit
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Language dropdown
    pub LanguageDropdown = <DropDown> {
        width: 200, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        popup_menu_position: BelowInput
        labels: ["Auto", "简体中文", "English"]
        values: [auto, zh_cn, en]
        selected_item: 2
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
            width: 200
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
    // General Tab Content
    // ========================================================================

    pub GeneralTab = <View> {
        width: Fill, height: Fill
        flow: Down
        padding: 24

        <Label> {
            text: "General"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        <SettingsRow> {
            startup_checkbox = <SettingsCheckBox> {
                text: "Launch at system startup"
            }
        }

        <SettingsRow> {
            exit_checkbox = <SettingsCheckBox> {
                text: "Exit when close main panel"
            }
        }

        <SettingsRow> {
            <SettingsLabel> { text: "Language" }
            <View> { width: Fill, height: Fit }
            language_dropdown = <LanguageDropdown> {}
        }

        <HDivider> {}

        // Appearance section
        appearance_section = <View> {
            width: Fill, height: Fit
            flow: Down

            <SectionTitle> { text: "Appearance" }

            appearance_radios = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 24

                light_radio = <MpRadio> {
                    text: "Light Mode"
                }

                dark_radio = <MpRadio> {
                    text: "Dark Mode"
                }

                auto_radio = <MpRadio> {
                    text: "Follow System"
                    checked: true
                }
            }
        }

        <HDivider> {}

        // Storage section
        storage_section = <View> {
            width: Fill, height: Fit
            flow: Down

            <SectionTitle> { text: "Storage" }

            <SettingsRow> {
                <SettingsLabel> { text: "Data Location" }
                <View> { width: Fill, height: Fit }
                storage_path = <Label> {
                    text: "~/Documents/colang"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                        }
                    }
                }
                browse_btn = <SettingsButton> { text: "Browse..." }
                default_path_btn = <SettingsButton> { text: "Default" }
                open_path_btn = <SettingsButton> { text: "Open" }
            }

            <SettingsRow> {
                <SettingsLabel> { text: "Cache" }
                <View> { width: Fill, height: Fit }
                cache_size = <Label> {
                    text: "256 MB"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                        }
                    }
                }
                clear_cache_btn = <SettingsButton> { text: "Clear Cache" }
            }
        }

        <View> { width: Fill, height: Fill }
    }
}
