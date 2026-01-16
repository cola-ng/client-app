//! Settings screen - main entry point with tab navigation

use makepad_component::widgets::*;
use makepad_component::*;
use makepad_widgets::*;

use super::provider_view::ProviderViewWidgetExt;
use super::providers_panel::{ProvidersPanelAction, ProvidersPanelWidgetExt};
use super::release_notes_modal::ReleaseNotesModalWidgetExt;
use crate::models::{Preferences, Provider, ProviderId};

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;
    use link::shaders::*;

    use makepad_component::theme::colors::*;
    use makepad_component::widgets::button::*;
    use makepad_component::widgets::checkbox::*;
    use makepad_component::widgets::switch::*;
    use makepad_component::widgets::divider::*;
    use makepad_component::widgets::radio::*;
    use makepad_component::widgets::progress::*;
    use makepad_component::widgets::slider::*;
    use makepad_component::widgets::input::*;
    use makepad_component::widgets::badge::*;
    use makepad_component::widgets::tooltip::*;

    use widgets::theme::*;

    use crate::screens::settings::providers_panel::ProvidersPanel;
    use crate::screens::settings::provider_view::ProviderView;
    use crate::screens::settings::add_provider_modal::AddProviderModal;
    use crate::screens::settings::release_notes_modal::ReleaseNotesModal;
    use crate::screens::settings::general_panel::GeneralTab;
    use crate::screens::settings::*;
    use crate::screens::settings::audio_panel::AudioTab;
    use crate::screens::settings::about_panel::AboutTab;

    // Vertical divider for sidebar
    VerticalDivider = <View> {
        width: 1, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((BORDER), (SLATE_700), self.dark_mode);
            }
        }
    }

    // Tab button in sidebar
    TabButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        align: {x: 0.0, y: 0.5}

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
        release_notes_modal = <ReleaseNotesModal> {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SettingsTab {
    General,
    Audio,
    Providers,
    About,
}

/// Theme mode for appearance settings
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

/// Actions emitted by the SettingsScreen
#[derive(Clone, Debug, DefaultNone)]
pub enum SettingsScreenAction {
    None,
    ThemeModeChanged(ThemeMode),
    OpenUrl(String),
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

    #[rust]
    data_location: String,

    #[rust]
    input_devices: Vec<String>,

    #[rust]
    output_devices: Vec<String>,

    #[rust]
    audio_initialized: bool,

    #[rust]
    website_url: String,
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

        // Initialize data location from preferences
        if self.data_location.is_empty() {
            if self.preferences.is_none() {
                self.preferences = Some(Preferences::load());
            }
            if let Some(prefs) = &self.preferences {
                self.data_location = prefs
                    .data_location
                    .clone()
                    .unwrap_or_else(|| super::get_default_data_location());
                self.view
                    .label(ids!(
                        content.pages.general_page.storage_section.storage_path
                    ))
                    .set_text(cx, &self.data_location);
            }
        }

        // Initialize audio devices
        if !self.audio_initialized {
            self.init_audio_devices(cx);
            self.audio_initialized = true;
        }

        // Extract actions for button clicks
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle tab button clicks
        if self
            .view
            .button(ids!(content.sidebar.general_tab_btn))
            .clicked(actions)
        {
            self.current_tab = SettingsTab::General;
            self.view
                .page_flip(ids!(content.pages))
                .set_active_page(cx, live_id!(general_page));
            self.update_tab_selection(cx);
        }
        if self
            .view
            .button(ids!(content.sidebar.audio_tab_btn))
            .clicked(actions)
        {
            self.current_tab = SettingsTab::Audio;
            self.view
                .page_flip(ids!(content.pages))
                .set_active_page(cx, live_id!(audio_page));
            self.update_tab_selection(cx);
        }
        if self
            .view
            .button(ids!(content.sidebar.providers_tab_btn))
            .clicked(actions)
        {
            self.current_tab = SettingsTab::Providers;
            self.view
                .page_flip(ids!(content.pages))
                .set_active_page(cx, live_id!(providers_page));
            self.update_tab_selection(cx);
        }
        if self
            .view
            .button(ids!(content.sidebar.about_tab_btn))
            .clicked(actions)
        {
            self.current_tab = SettingsTab::About;
            self.view
                .page_flip(ids!(content.pages))
                .set_active_page(cx, live_id!(about_page));
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
        if self
            .view
            .button(ids!(
                content.pages.providers_page.providers_panel.add_button
            ))
            .clicked(actions)
        {
            self.view
                .view(ids!(add_provider_modal))
                .set_visible(cx, true);
            self.view.redraw(cx);
        }

        // Handle modal cancel button
        if self
            .view
            .button(ids!(add_provider_modal.cancel_button))
            .clicked(actions)
        {
            self.view
                .view(ids!(add_provider_modal))
                .set_visible(cx, false);
            self.view.redraw(cx);
        }

        // Handle modal add button
        if self
            .view
            .button(ids!(add_provider_modal.add_button))
            .clicked(actions)
        {
            let name = self
                .view
                .text_input(ids!(add_provider_modal.name_input))
                .text();
            let host = self
                .view
                .text_input(ids!(add_provider_modal.host_input))
                .text();
            let key = self
                .view
                .text_input(ids!(add_provider_modal.key_input))
                .text();

            if !name.is_empty() && !host.is_empty() {
                let id = ProviderId::from(name.to_lowercase().replace(" ", "_").replace("-", "_"));
                let provider = Provider {
                    id: id.clone(),
                    name,
                    url: host,
                    api_key: if key.is_empty() { None } else { Some(key) },
                    provider_type: crate::models::ProviderType::Custom,
                    enabled: true,
                    models: vec![],
                    is_custom: true,
                    connection_status: crate::models::ProviderConnectionStatus::Disconnected,
                };

                self.add_custom_provider(cx, provider);
                self.view
                    .view(ids!(add_provider_modal))
                    .set_visible(cx, false);
                self.view.redraw(cx);
            }
        }

        // Handle save button in provider view
        if self
            .view
            .button(ids!(content.pages.providers_page.provider_view.save_button))
            .clicked(actions)
        {
            self.save_current_provider(cx);
        }

        // Handle remove button in provider view
        if self
            .view
            .button(ids!(
                content.pages.providers_page.provider_view.remove_button
            ))
            .clicked(actions)
        {
            self.remove_current_provider(cx);
        }

        // Handle sync button in provider view
        if self
            .view
            .button(ids!(content.pages.providers_page.provider_view.sync_button))
            .clicked(actions)
        {
            self.sync_models(cx);
        }

        // Handle browse button for data location
        if self
            .view
            .button(ids!(content.pages.general_page.storage_section.browse_btn))
            .clicked(actions)
        {
            self.browse_data_location(cx);
        }

        // Handle default path button
        if self
            .view
            .button(ids!(
                content.pages.general_page.storage_section.default_path_btn
            ))
            .clicked(actions)
        {
            self.reset_to_default_location(cx);
        }

        // Handle open path button
        if self
            .view
            .button(ids!(
                content.pages.general_page.storage_section.open_path_btn
            ))
            .clicked(actions)
        {
            super::open_data_location(&self.data_location);
        }

        // Handle clear cache button
        if self
            .view
            .button(ids!(
                content.pages.general_page.storage_section.clear_cache_btn
            ))
            .clicked(actions)
        {
            self.clear_cache(cx);
        }

        // Handle appearance radio buttons using MpRadio
        if self
            .view
            .mp_radio(ids!(
                content
                    .pages
                    .general_page
                    .appearance_section
                    .appearance_radios
                    .light_radio
            ))
            .changed(actions)
            .is_some()
        {
            self.view
                .mp_radio(ids!(
                    content
                        .pages
                        .general_page
                        .appearance_section
                        .appearance_radios
                        .dark_radio
                ))
                .set_checked(cx, false);
            self.view
                .mp_radio(ids!(
                    content
                        .pages
                        .general_page
                        .appearance_section
                        .appearance_radios
                        .auto_radio
                ))
                .set_checked(cx, false);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SettingsScreenAction::ThemeModeChanged(ThemeMode::Light),
            );
        }

        if self
            .view
            .mp_radio(ids!(
                content
                    .pages
                    .general_page
                    .appearance_section
                    .appearance_radios
                    .dark_radio
            ))
            .changed(actions)
            .is_some()
        {
            self.view
                .mp_radio(ids!(
                    content
                        .pages
                        .general_page
                        .appearance_section
                        .appearance_radios
                        .light_radio
                ))
                .set_checked(cx, false);
            self.view
                .mp_radio(ids!(
                    content
                        .pages
                        .general_page
                        .appearance_section
                        .appearance_radios
                        .auto_radio
                ))
                .set_checked(cx, false);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SettingsScreenAction::ThemeModeChanged(ThemeMode::Dark),
            );
        }

        if self
            .view
            .mp_radio(ids!(
                content
                    .pages
                    .general_page
                    .appearance_section
                    .appearance_radios
                    .auto_radio
            ))
            .changed(actions)
            .is_some()
        {
            self.view
                .mp_radio(ids!(
                    content
                        .pages
                        .general_page
                        .appearance_section
                        .appearance_radios
                        .light_radio
                ))
                .set_checked(cx, false);
            self.view
                .mp_radio(ids!(
                    content
                        .pages
                        .general_page
                        .appearance_section
                        .appearance_radios
                        .dark_radio
                ))
                .set_checked(cx, false);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SettingsScreenAction::ThemeModeChanged(ThemeMode::System),
            );
        }

        // Handle Release Notes button click
        if self
            .view
            .button(ids!(content.pages.about_page.release_notes_btn))
            .clicked(actions)
        {
            self.view
                .release_notes_modal(ids!(release_notes_modal))
                .show(cx);
        }

        // Handle Release Notes modal close button
        if self
            .view
            .button(ids!(
                release_notes_modal
                    .dialog_container
                    .dialog
                    .header
                    .close_button
            ))
            .clicked(actions)
        {
            self.view
                .release_notes_modal(ids!(release_notes_modal))
                .hide(cx);
        }

        // Handle Service Agreement link click
        if self
            .view
            .button(ids!(content.pages.about_page.footer.links_row.terms_link))
            .clicked(actions)
        {
            let url = format!("{}/terms", self.website_url.trim_end_matches('/'));
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SettingsScreenAction::OpenUrl(url),
            );
        }

        // Handle Privacy Policy link click
        if self
            .view
            .button(ids!(content.pages.about_page.footer.links_row.privacy_link))
            .clicked(actions)
        {
            let url = format!("{}/privacy", self.website_url.trim_end_matches('/'));
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SettingsScreenAction::OpenUrl(url),
            );
        }

        // Handle Send Feedback button click
        if self
            .view
            .button(ids!(content.pages.about_page.feedback_btn))
            .clicked(actions)
        {
            let url = format!("{}/feedback", self.website_url.trim_end_matches('/'));
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SettingsScreenAction::OpenUrl(url),
            );
        }
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
            (
                ids!(content.sidebar.providers_tab_btn),
                SettingsTab::Providers,
            ),
            (ids!(content.sidebar.about_tab_btn), SettingsTab::About),
        ];

        for (tab_id, tab) in tabs {
            let selected = if self.current_tab == tab { 1.0 } else { 0.0 };
            self.view.button(tab_id).apply_over(
                cx,
                live! {
                    draw_bg: { selected: (selected), dark_mode: (self.dark_mode) }
                    draw_text: { selected: (selected), dark_mode: (self.dark_mode) }
                },
            );
        }
        self.view.redraw(cx);
    }

    fn init_audio_devices(&mut self, cx: &mut Cx) {
        let devices = super::init_audio_devices();

        self.input_devices = devices.input_devices;
        self.output_devices = devices.output_devices;

        // Populate input dropdown
        if !devices.input_labels.is_empty() {
            let dropdown = self
                .view
                .drop_down(ids!(content.pages.audio_page.mic_section.mic_device));
            dropdown.set_labels(cx, devices.input_labels);
            dropdown.set_selected_item(cx, 0);
        }

        // Populate output dropdown
        if !devices.output_labels.is_empty() {
            let dropdown = self.view.drop_down(ids!(
                content.pages.audio_page.speaker_section.speaker_device
            ));
            dropdown.set_labels(cx, devices.output_labels);
            dropdown.set_selected_item(cx, 0);
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
                        provider
                            .api_key
                            .as_ref()
                            .map(|k| !k.is_empty())
                            .unwrap_or(false),
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
                    (
                        name.to_string(),
                        url.to_string(),
                        "".to_string(),
                        Vec::new(),
                        false,
                        false,
                    )
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
                (
                    name.to_string(),
                    url.to_string(),
                    "".to_string(),
                    Vec::new(),
                    false,
                    false,
                )
            }
        };

        // Update the provider view
        self.view
            .label(ids!(
                content.pages.providers_page.provider_view.provider_name
            ))
            .set_text(cx, &provider_name);
        self.view
            .text_input(ids!(
                content.pages.providers_page.provider_view.api_host_input
            ))
            .set_text(cx, &provider_url);
        self.view
            .text_input(ids!(
                content.pages.providers_page.provider_view.api_key_input
            ))
            .set_text(cx, &api_key);

        // Update status labels
        self.view
            .label(ids!(
                content.pages.providers_page.provider_view.no_models_label
            ))
            .set_visible(cx, saved_models.is_empty());

        if !saved_models.is_empty() {
            self.view
                .label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, &format!("{} models", saved_models.len()));
        } else if has_api_key {
            self.view
                .label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "");
        } else {
            self.view
                .label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "Enter API key to sync models");
        }

        // Update sync button state
        if has_api_key {
            self.view
                .button(ids!(content.pages.providers_page.provider_view.sync_button))
                .apply_over(cx, live! { draw_bg: { disabled: 0.0 } });
        } else {
            self.view
                .button(ids!(content.pages.providers_page.provider_view.sync_button))
                .apply_over(cx, live! { draw_bg: { disabled: 1.0 } });
        }

        // Show content, hide empty state
        self.view
            .view(ids!(content.pages.providers_page.provider_view.content))
            .set_visible(cx, true);
        self.view
            .view(ids!(content.pages.providers_page.provider_view.empty_state))
            .set_visible(cx, false);
        self.view
            .button(ids!(
                content.pages.providers_page.provider_view.remove_button
            ))
            .set_visible(cx, is_custom);

        self.view.redraw(cx);
    }

    fn save_current_provider(&mut self, _cx: &mut Cx) {
        if let Some(provider_id) = &self.selected_provider_id {
            let api_host = self
                .view
                .text_input(ids!(
                    content.pages.providers_page.provider_view.api_host_input
                ))
                .text();
            let api_key = {
                let key = self
                    .view
                    .text_input(ids!(
                        content.pages.providers_page.provider_view.api_key_input
                    ))
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
                if let Some(idx) = prefs
                    .providers
                    .iter()
                    .position(|p| p.id == provider_id && p.is_custom)
                {
                    prefs.providers.remove(idx);
                    if let Err(e) = prefs.save() {
                        eprintln!("Failed to save preferences: {}", e);
                    }
                    self.selected_provider_id = None;
                    self.view
                        .view(ids!(content.pages.providers_page.provider_view.content))
                        .set_visible(cx, false);
                    self.view
                        .view(ids!(content.pages.providers_page.provider_view.empty_state))
                        .set_visible(cx, true);
                    self.view
                        .label(ids!(
                            content.pages.providers_page.provider_view.provider_name
                        ))
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
        let api_key = self
            .view
            .text_input(ids!(
                content.pages.providers_page.provider_view.api_key_input
            ))
            .text();
        if api_key.is_empty() {
            self.view
                .label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "Enter API key to sync models");
            self.view.redraw(cx);
            return;
        }

        self.view
            .label(ids!(content.pages.providers_page.provider_view.sync_status))
            .set_text(cx, "Syncing models...");
        self.view
            .button(ids!(content.pages.providers_page.provider_view.sync_button))
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

        self.view
            .label(ids!(
                content.pages.providers_page.provider_view.no_models_label
            ))
            .set_visible(cx, models.is_empty());

        if models.is_empty() {
            self.view
                .label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, "No models found");
        } else {
            self.view
                .label(ids!(content.pages.providers_page.provider_view.sync_status))
                .set_text(cx, &format!("Found {} models", models.len()));
        }

        self.view
            .button(ids!(content.pages.providers_page.provider_view.sync_button))
            .apply_over(cx, live! { draw_bg: { disabled: 0.0 } });

        if let Some(provider_id) = self.selected_provider_id.clone() {
            self.load_provider_to_view(cx, &provider_id);
        }
    }

    fn browse_data_location(&mut self, cx: &mut Cx) {
        if let Some(folder) = super::browse_data_location(&self.data_location) {
            self.data_location = folder.clone();

            // Update the label in the UI
            self.view
                .label(ids!(
                    content.pages.general_page.storage_section.storage_path
                ))
                .set_text(cx, &folder);

            // Save to preferences
            if self.preferences.is_none() {
                self.preferences = Some(Preferences::load());
            }
            if let Some(prefs) = &mut self.preferences {
                prefs.data_location = Some(folder);
                if let Err(e) = prefs.save() {
                    eprintln!("Failed to save data location: {}", e);
                }
            }

            self.view.redraw(cx);
        }
    }

    fn clear_cache(&mut self, cx: &mut Cx) {
        if super::clear_cache().is_ok() {
            // Update cache size label
            self.view
                .label(ids!(content.pages.general_page.storage_section.cache_size))
                .set_text(cx, "0 MB");
            self.view.redraw(cx);
        }
    }

    fn reset_to_default_location(&mut self, cx: &mut Cx) {
        let default_path = super::get_default_data_location();
        self.data_location = default_path.clone();

        // Update the label in the UI
        self.view
            .label(ids!(
                content.pages.general_page.storage_section.storage_path
            ))
            .set_text(cx, &default_path);

        // Save to preferences (None means use default)
        if self.preferences.is_none() {
            self.preferences = Some(Preferences::load());
        }
        if let Some(prefs) = &mut self.preferences {
            prefs.data_location = None; // None means default
            if let Err(e) = prefs.save() {
                eprintln!("Failed to save data location: {}", e);
            }
        }

        self.view.redraw(cx);
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
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Apply dark mode to sidebar
            inner.view.view(ids!(content.sidebar)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Apply dark mode to sidebar divider
            inner.view.view(ids!(content.sidebar_divider)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Apply dark mode to providers panel and provider view
            inner
                .view
                .providers_panel(ids!(content.pages.providers_page.providers_panel))
                .update_dark_mode(cx, dark_mode);
            inner
                .view
                .provider_view(ids!(content.pages.providers_page.provider_view))
                .update_dark_mode(cx, dark_mode);

            // Update tab selection with new dark mode
            inner.update_tab_selection(cx);

            // Update release notes modal dark mode
            inner
                .view
                .release_notes_modal(ids!(release_notes_modal))
                .update_dark_mode(cx, dark_mode);

            inner.view.redraw(cx);
        }
    }

    /// Set the website URL for links
    pub fn set_website_url(&self, website_url: String) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.website_url = website_url;
        }
    }
}
