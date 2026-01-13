//! Settings Screen - Main settings interface with tabs
//!
//! Tabs: General, Audio, AI Providers, About

use crate::data::{Preferences, Provider, ProviderId};
use crate::provider_view::ProviderViewWidgetExt;
use crate::providers_panel::{ProvidersPanelAction, ProvidersPanelWidgetExt};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use widgets::theme::*;

    use crate::providers_panel::ProvidersPanel;
    use crate::provider_view::ProviderView;
    use crate::add_provider_modal::AddProviderModal;

    // ========================================================================
    // Reusable Components
    // ========================================================================

    // Settings section title
    SectionTitle = <Label> {
        width: Fill, height: Fit
        margin: {bottom: 8, top: 16}
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_300), self.dark_mode);
            }
        }
    }

    // Settings item row with label
    SettingsRow = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: {top: 8, bottom: 8}
        align: {y: 0.5}
        spacing: 12
    }

    // Settings label
    SettingsLabel = <Label> {
        width: Fit, height: Fit
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Settings description text
    SettingsDesc = <Label> {
        width: Fill, height: Fit
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
            }
        }
    }

    // Horizontal divider
    HDivider = <View> {
        width: Fill, height: 1
        margin: {top: 12, bottom: 12}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((BORDER), (BORDER_DARK), self.dark_mode);
            }
        }
    }

    // Vertical divider
    VerticalDivider = <View> {
        width: 1, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((BORDER), (BORDER_DARK), self.dark_mode);
            }
        }
    }

    // Tab button for sidebar
    TabButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background colors
                let light_normal = (WHITE);
                let light_hover = (SLATE_100);
                let light_selected = (BLUE_50);

                let dark_normal = (SLATE_800);
                let dark_hover = (SLATE_700);
                let dark_selected = (SLATE_700);

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let selected_color = mix(light_selected, dark_selected, self.dark_mode);

                let bg = mix(normal, hover_color, self.hover);
                let bg = mix(bg, selected_color, self.selected);

                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(bg);

                // Left indicator when selected
                if self.selected > 0.5 {
                    let indicator_color = mix((ACCENT_BLUE), (ACCENT_BLUE_DARK), self.dark_mode);
                    sdf.box(0., 4., 3., self.rect_size.y - 8., 1.5);
                    sdf.fill(indicator_color);
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance selected: 0.0
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }

            fn get_color(self) -> vec4 {
                let light_normal = (TEXT_SECONDARY);
                let light_selected = (ACCENT_BLUE);
                let dark_normal = (TEXT_SECONDARY_DARK);
                let dark_selected = (ACCENT_BLUE_DARK);

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let selected_color = mix(light_selected, dark_selected, self.dark_mode);
                return mix(normal, selected_color, self.selected);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
            pressed = {
                default: off
                off = { from: {all: Forward {duration: 0.05}} apply: {draw_bg: {pressed: 0.0}} }
                on = { from: {all: Forward {duration: 0.02}} apply: {draw_bg: {pressed: 1.0}} }
            }
        }
    }

    // Standard button style
    SettingsButton = <Button> {
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

    // Primary action button
    PrimaryButton = <Button> {
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

    // Simple checkbox style for settings
    SettingsCheckBox = <CheckBox> {
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
    SettingsRadioButton = <RadioButton> {
        width: Fit, height: Fit
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Device selector (using RoundedView instead of Dropdown)
    DeviceSelector = <RoundedView> {
        width: Fill, height: Fit
        flow: Right
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        align: {y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 6.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        <Label> {
            width: Fill, height: Fit
            text: "Default Device"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        // Dropdown arrow indicator
        <Label> {
            text: "▾"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                }
            }
        }
    }

    // ========================================================================
    // General Tab Content
    // ========================================================================

    GeneralTab = <View> {
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

        // Startup section
        startup_section = <View> {
            width: Fill, height: Fit
            flow: Down
            margin: {top: 20}

            <SectionTitle> { text: "Startup" }

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
        }

        <HDivider> {}

        // Appearance section
        appearance_section = <View> {
            width: Fill, height: Fit
            flow: Down

            <SectionTitle> { text: "Appearance" }

            appearance_radios = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                auto_radio = <SettingsRadioButton> {
                    text: "Auto (follow system)"
                    animator: { selected = { default: on } }
                }

                light_radio = <SettingsRadioButton> {
                    text: "Light Mode"
                }

                dark_radio = <SettingsRadioButton> {
                    text: "Dark Mode"
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
                    text: "~/Documents/ColangEnglish"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                        }
                    }
                }
                browse_btn = <SettingsButton> { text: "Browse..." }
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

    // ========================================================================
    // Audio Tab Content
    // ========================================================================

    AudioTab = <View> {
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
                speaker_device = <DeviceSelector> {}
                speaker_test_btn = <SettingsButton> { text: "Test" }
            }

            <SettingsRow> {
                <SettingsLabel> { text: "Volume" width: 120 }
                speaker_volume = <Slider> {
                    width: Fill, height: Fit
                    min: 0.0, max: 100.0
                    step: 1.0
                    text: ""
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
                mic_device = <DeviceSelector> {}
                mic_test_btn = <SettingsButton> { text: "Test" }
            }

            <SettingsRow> {
                <SettingsLabel> { text: "Input Volume" width: 120 }
                mic_volume = <Slider> {
                    width: Fill, height: Fit
                    min: 0.0, max: 100.0
                    step: 1.0
                    text: ""
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

    // ========================================================================
    // AI Providers Tab Content
    // ========================================================================

    ProvidersTab = <View> {
        width: Fill, height: Fill
        flow: Right

        // Left panel - provider list
        providers_panel = <ProvidersPanel> {}

        // Divider
        <VerticalDivider> {}

        // Right panel - provider details
        provider_view = <ProviderView> {}
    }

    // ========================================================================
    // About Tab Content
    // ========================================================================

    AboutTab = <View> {
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

    // ========================================================================
    // Main Settings Screen
    // ========================================================================

    pub SettingsScreen = {{SettingsScreen}} {
        width: Fill, height: Fill
        flow: Overlay
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Main content
        content = <View> {
            width: Fill, height: Fill
            flow: Right

            // Left sidebar with tabs
            sidebar = <View> {
                width: 200, height: Fill
                flow: Down
                padding: {top: 16, bottom: 16, left: 8, right: 8}
                spacing: 4
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix((WHITE), (SLATE_800), self.dark_mode);
                    }
                }

                <Label> {
                    text: "Settings"
                    margin: {left: 8, bottom: 16}
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 16.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                general_tab_btn = <TabButton> { text: "General" }
                audio_tab_btn = <TabButton> { text: "Audio" }
                providers_tab_btn = <TabButton> { text: "AI Providers" }
                about_tab_btn = <TabButton> { text: "About" }

                <View> { width: Fill, height: Fill }
            }

            // Divider
            sidebar_divider = <VerticalDivider> {}

            // Right content area with PageFlip
            pages = <PageFlip> {
                width: Fill, height: Fill
                active_page: general_page

                general_page = <GeneralTab> {}
                audio_page = <AudioTab> {}
                providers_page = <ProvidersTab> {}
                about_page = <AboutTab> {}
            }
        }

        // Modal overlay (hidden by default)
        add_provider_modal = <AddProviderModal> {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SettingsTab {
    General,
    Audio,
    Providers,
    About,
}

#[derive(Live, LiveHook, Widget)]
pub struct SettingsScreen {
    #[deref]
    view: View,

    #[rust]
    preferences: Option<Preferences>,

    #[rust]
    selected_provider_id: Option<ProviderId>,

    #[rust]
    current_tab: SettingsTab,

    #[rust]
    dark_mode: f64,
}

impl Widget for SettingsScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Initialize default tab on first run
        if self.current_tab == SettingsTab::General {
            self.update_tab_selection(cx);
        }

        // Initialize with default provider on first event
        if self.selected_provider_id.is_none() {
            let default_id = ProviderId::from("openai");
            self.selected_provider_id = Some(default_id.clone());
            self.load_provider_to_view(cx, &default_id);
        }

        // Extract actions for button clicks
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle tab button clicks
        if self.view.button(ids!(content.sidebar.general_tab_btn)).clicked(actions) {
            self.current_tab = SettingsTab::General;
            self.view.page_flip(ids!(content.pages)).set_active_page(cx, live_id!(general_page));
            self.update_tab_selection(cx);
        }
        if self.view.button(ids!(content.sidebar.audio_tab_btn)).clicked(actions) {
            self.current_tab = SettingsTab::Audio;
            self.view.page_flip(ids!(content.pages)).set_active_page(cx, live_id!(audio_page));
            self.update_tab_selection(cx);
        }
        if self.view.button(ids!(content.sidebar.providers_tab_btn)).clicked(actions) {
            self.current_tab = SettingsTab::Providers;
            self.view.page_flip(ids!(content.pages)).set_active_page(cx, live_id!(providers_page));
            self.update_tab_selection(cx);
        }
        if self.view.button(ids!(content.sidebar.about_tab_btn)).clicked(actions) {
            self.current_tab = SettingsTab::About;
            self.view.page_flip(ids!(content.pages)).set_active_page(cx, live_id!(about_page));
            self.update_tab_selection(cx);
        }

        // Handle provider selection from ProvidersPanel action
        for action in actions {
            if let ProvidersPanelAction::Selected(id) = action.as_widget_action().cast() {
                if self.selected_provider_id.as_ref() != Some(&id) {
                    self.selected_provider_id = Some(id.clone());
                    self.load_provider_to_view(cx, &id);
                }
            }
        }

        // Handle add provider button
        if self.view.button(ids!(content.pages.providers_page.providers_panel.add_button)).clicked(actions) {
            self.view.view(ids!(add_provider_modal)).set_visible(cx, true);
            self.view.redraw(cx);
        }

        // Handle modal cancel button
        if self.view.button(ids!(add_provider_modal.cancel_button)).clicked(actions) {
            self.view.view(ids!(add_provider_modal)).set_visible(cx, false);
            self.view.redraw(cx);
        }

        // Handle modal add button
        if self.view.button(ids!(add_provider_modal.add_button)).clicked(actions) {
            let name = self.view.text_input(ids!(add_provider_modal.name_input)).text();
            let host = self.view.text_input(ids!(add_provider_modal.host_input)).text();
            let key = self.view.text_input(ids!(add_provider_modal.key_input)).text();

            if !name.is_empty() && !host.is_empty() {
                let id = ProviderId::from(name.to_lowercase().replace(" ", "_").replace("-", "_"));
                let provider = Provider {
                    id: id.clone(),
                    name,
                    url: host,
                    api_key: if key.is_empty() { None } else { Some(key) },
                    provider_type: crate::data::ProviderType::Custom,
                    enabled: true,
                    models: vec![],
                    is_custom: true,
                    connection_status: crate::data::ProviderConnectionStatus::Disconnected,
                };

                self.add_custom_provider(cx, provider);
                self.view.view(ids!(add_provider_modal)).set_visible(cx, false);
                self.view.redraw(cx);
            }
        }

        // Handle save button in provider view
        if self.view.button(ids!(content.pages.providers_page.provider_view.save_button)).clicked(actions) {
            self.save_current_provider(cx);
        }

        // Handle remove button in provider view
        if self.view.button(ids!(content.pages.providers_page.provider_view.remove_button)).clicked(actions) {
            self.remove_current_provider(cx);
        }

        // Handle sync button in provider view
        if self.view.button(ids!(content.pages.providers_page.provider_view.sync_button)).clicked(actions) {
            self.sync_models(cx);
        }

        // Handle appearance radio buttons
        self.view.radio_button_set(ids_array!(
            content.pages.general_page.appearance_section.appearance_radios.auto_radio,
            content.pages.general_page.appearance_section.appearance_radios.light_radio,
            content.pages.general_page.appearance_section.appearance_radios.dark_radio
        )).selected(cx, actions);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Default for SettingsTab {
    fn default() -> Self {
        SettingsTab::General
    }
}

impl SettingsScreen {
    fn update_tab_selection(&mut self, cx: &mut Cx) {
        let tabs = [
            (ids!(content.sidebar.general_tab_btn), SettingsTab::General),
            (ids!(content.sidebar.audio_tab_btn), SettingsTab::Audio),
            (ids!(content.sidebar.providers_tab_btn), SettingsTab::Providers),
            (ids!(content.sidebar.about_tab_btn), SettingsTab::About),
        ];

        for (tab_id, tab) in tabs {
            let selected = if self.current_tab == tab { 1.0 } else { 0.0 };
            self.view.button(tab_id).apply_over(cx, live! {
                draw_bg: { selected: (selected), dark_mode: (self.dark_mode) }
                draw_text: { selected: (selected), dark_mode: (self.dark_mode) }
            });
        }
        self.view.redraw(cx);
    }

    fn load_provider_to_view(&mut self, cx: &mut Cx, provider_id: &ProviderId) {
        // Load preferences if needed
        if self.preferences.is_none() {
            self.preferences = Some(Preferences::load());
        }

        // Find the provider and clone the data we need
        let (provider_name, provider_url, api_key, saved_models, has_api_key, is_custom) = {
            if let Some(prefs) = &self.preferences {
                if let Some(provider) = prefs.providers.iter().find(|p| &p.id == provider_id) {
                    (
                        provider.name.clone(),
                        provider.url.clone(),
                        provider.api_key.as_deref().unwrap_or("").to_string(),
                        provider.models.clone(),
                        provider.api_key.as_ref().map(|k| !k.is_empty()).unwrap_or(false),
                        provider.is_custom,
                    )
                } else {
                    let name = match provider_id.as_str() {
                        "openai" => "OpenAI",
                        "deepseek" => "DeepSeek",
                        "alibaba_cloud" => "Alibaba Cloud (Qwen)",
                        _ => provider_id.as_str(),
                    };
                    let url = match provider_id.as_str() {
                        "openai" => "https://api.openai.com/v1",
                        "deepseek" => "https://api.deepseek.com",
                        "alibaba_cloud" => "https://dashscope.aliyuncs.com/compatible-mode/v1",
                        _ => "",
                    };
                    (name.to_string(), url.to_string(), "".to_string(), Vec::new(), false, false)
                }
            } else {
                let name = match provider_id.as_str() {
                    "openai" => "OpenAI",
                    "deepseek" => "DeepSeek",
                    "alibaba_cloud" => "Alibaba Cloud (Qwen)",
                    _ => provider_id.as_str(),
                };
                let url = match provider_id.as_str() {
                    "openai" => "https://api.openai.com/v1",
                    "deepseek" => "https://api.deepseek.com",
                    "alibaba_cloud" => "https://dashscope.aliyuncs.com/compatible-mode/v1",
                    _ => "",
                };
                (name.to_string(), url.to_string(), "".to_string(), Vec::new(), false, false)
            }
        };

        // Update the provider view
        self.view.label(ids!(content.pages.providers_page.provider_view.provider_name))
            .set_text(cx, &provider_name);
        self.view.text_input(ids!(content.pages.providers_page.provider_view.api_host_input))
            .set_text(cx, &provider_url);
        self.view.text_input(ids!(content.pages.providers_page.provider_view.api_key_input))
            .set_text(cx, &api_key);

        // Update status labels
        self.view.label(ids!(content.pages.providers_page.provider_view.no_models_label))
            .set_visible(cx, saved_models.is_empty());

        if !saved_models.is_empty() {
            self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, &format!("{} models", saved_models.len()));
        } else if has_api_key {
            self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "");
        } else {
            self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "Enter API key to sync models");
        }

        // Update sync button state
        if has_api_key {
            self.view.button(ids!(content.pages.providers_page.provider_view.sync_button))
                .apply_over(cx, live! { draw_bg: { disabled: 0.0 } });
        } else {
            self.view.button(ids!(content.pages.providers_page.provider_view.sync_button))
                .apply_over(cx, live! { draw_bg: { disabled: 1.0 } });
        }

        // Show content, hide empty state
        self.view.view(ids!(content.pages.providers_page.provider_view.content))
            .set_visible(cx, true);
        self.view.view(ids!(content.pages.providers_page.provider_view.empty_state))
            .set_visible(cx, false);
        self.view.button(ids!(content.pages.providers_page.provider_view.remove_button))
            .set_visible(cx, is_custom);

        self.view.redraw(cx);
    }

    fn save_current_provider(&mut self, _cx: &mut Cx) {
        if let Some(provider_id) = &self.selected_provider_id {
            let api_host = self.view
                .text_input(ids!(content.pages.providers_page.provider_view.api_host_input))
                .text();
            let api_key = {
                let key = self.view
                    .text_input(ids!(content.pages.providers_page.provider_view.api_key_input))
                    .text();
                if key.is_empty() { None } else { Some(key) }
            };

            if self.preferences.is_none() {
                self.preferences = Some(Preferences::load());
            }
            if let Some(prefs) = &mut self.preferences {
                if let Some(provider) = prefs.providers.iter_mut().find(|p| &p.id == provider_id) {
                    provider.url = api_host;
                    provider.api_key = api_key;
                    if let Err(e) = prefs.save() {
                        eprintln!("Failed to save preferences: {}", e);
                    } else {
                        ::log::info!("Saved provider settings for {}", provider_id.as_str());
                    }
                }
            }
        }
    }

    fn remove_current_provider(&mut self, cx: &mut Cx) {
        if let Some(provider_id) = self.selected_provider_id.clone() {
            if self.preferences.is_none() {
                self.preferences = Some(Preferences::load());
            }
            if let Some(prefs) = &mut self.preferences {
                if let Some(idx) = prefs.providers.iter().position(|p| p.id == provider_id && p.is_custom) {
                    prefs.providers.remove(idx);
                    if let Err(e) = prefs.save() {
                        eprintln!("Failed to save preferences: {}", e);
                    }
                    self.selected_provider_id = None;
                    self.view.view(ids!(content.pages.providers_page.provider_view.content))
                        .set_visible(cx, false);
                    self.view.view(ids!(content.pages.providers_page.provider_view.empty_state))
                        .set_visible(cx, true);
                    self.view.label(ids!(content.pages.providers_page.provider_view.provider_name))
                        .set_text(cx, "Select a Provider");
                    self.view.redraw(cx);
                }
            }
        }
    }

    fn add_custom_provider(&mut self, cx: &mut Cx, provider: Provider) {
        if self.preferences.is_none() {
            self.preferences = Some(Preferences::load());
        }
        if let Some(prefs) = &mut self.preferences {
            let id = provider.id.clone();
            prefs.providers.push(provider);
            if let Err(e) = prefs.save() {
                eprintln!("Failed to save preferences: {}", e);
            }
            self.selected_provider_id = Some(id.clone());
            self.load_provider_to_view(cx, &id);
        }
    }

    fn sync_models(&mut self, cx: &mut Cx) {
        let api_key = self.view
            .text_input(ids!(content.pages.providers_page.provider_view.api_key_input))
            .text();
        if api_key.is_empty() {
            self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "Enter API key to sync models");
            self.view.redraw(cx);
            return;
        }

        self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
            .set_text(cx, "Syncing models...");
        self.view.button(ids!(content.pages.providers_page.provider_view.sync_button))
            .apply_over(cx, live! { draw_bg: { disabled: 1.0 } });
        self.view.redraw(cx);

        let models = if let Some(provider_id) = &self.selected_provider_id {
            match provider_id.as_str() {
                "openai" => vec![
                    "gpt-4o".to_string(),
                    "gpt-4o-mini".to_string(),
                    "gpt-4-turbo".to_string(),
                    "gpt-4".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ],
                "deepseek" => vec!["deepseek-chat".to_string(), "deepseek-coder".to_string()],
                "alibaba_cloud" => vec![
                    "qwen-turbo".to_string(),
                    "qwen-plus".to_string(),
                    "qwen-max".to_string(),
                ],
                _ => vec!["custom-model".to_string()],
            }
        } else {
            vec![]
        };

        if let Some(provider_id) = &self.selected_provider_id {
            if self.preferences.is_none() {
                self.preferences = Some(Preferences::load());
            }
            if let Some(prefs) = &mut self.preferences {
                if let Some(provider) = prefs.providers.iter_mut().find(|p| &p.id == provider_id) {
                    provider.models = models.clone();
                    if let Err(e) = prefs.save() {
                        eprintln!("Failed to save models: {}", e);
                    }
                }
            }
        }

        self.view.label(ids!(content.pages.providers_page.provider_view.no_models_label))
            .set_visible(cx, models.is_empty());

        if models.is_empty() {
            self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "No models found");
        } else {
            self.view.label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, &format!("Found {} models", models.len()));
        }

        self.view.button(ids!(content.pages.providers_page.provider_view.sync_button))
            .apply_over(cx, live! { draw_bg: { disabled: 0.0 } });

        if let Some(provider_id) = self.selected_provider_id.clone() {
            self.load_provider_to_view(cx, &provider_id);
        }
    }
}

impl SettingsScreenRef {
    /// Initialize the settings screen with preferences
    pub fn init(&self, cx: &mut Cx, preferences: Preferences) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.preferences = Some(preferences);
            inner.view.redraw(cx);
        }
    }

    /// Reload preferences from disk
    pub fn reload_preferences(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.preferences = Some(Preferences::load());
            inner.view.redraw(cx);
        }
    }

    /// Update dark mode for this screen
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.dark_mode = dark_mode;

            // Apply dark mode to screen background
            inner.view.apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply dark mode to sidebar
            inner.view.view(ids!(content.sidebar)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply dark mode to sidebar divider
            inner.view.view(ids!(content.sidebar_divider)).apply_over(cx, live! {
                draw_bg: { dark_mode: (dark_mode) }
            });

            // Apply dark mode to providers panel and provider view
            inner.view.providers_panel(ids!(content.pages.providers_page.providers_panel))
                .update_dark_mode(cx, dark_mode);
            inner.view.provider_view(ids!(content.pages.providers_page.provider_view))
                .update_dark_mode(cx, dark_mode);

            // Update tab selection with new dark mode
            inner.update_tab_selection(cx);

            inner.view.redraw(cx);
        }
    }
}
