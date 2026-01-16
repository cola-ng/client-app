//! MoFA Studio App - Main application shell
//!
//! This file contains the main App struct and all UI definitions.
//! Organized into sections:
//! - UI Definitions (live_design! macro)
//! - Widget Structs (MainBody, App)
//! - Event Handling (AppMain impl)
//! - Helper Methods (organized by responsibility)

// App plugin system imports
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

use colang_core::asset_api::init_asset_api;
use colang_core::learn_api::{init_learn_api, set_learn_api_token};
use colang_core::models::Preferences;
use colang_core::screens::conversation::conversation_screen::ConversationScreenWidgetRefExt;
use colang_core::screens::settings::settings_screen::SettingsScreenWidgetRefExt;
use colang_core::screens::settings::{SettingsScreenAction, ThemeMode};
use colang_shell::widgets::sidebar::SidebarWidgetRefExt;
use makepad_widgets::*;
use makepad_component::*;
use makepad_component::widgets::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use ::widgets::StateChangeListener;
use ::widgets::debug_panel::DebugPanelWidgetRefExt;

use crate::config::Config;

// ============================================================================
// TAB IDENTIFIER
// ============================================================================

/// Type-safe tab identifiers (replaces magic strings)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabId {
    Profile,
    Settings,
}

enum DesktopAuthResult {
    Success(String),
    Error(String),
}

// ============================================================================
// UI DEFINITIONS
// ============================================================================

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    use makepad_component::*;
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

    // Import fonts and colors from shared theme (single source of truth)
    use widgets::theme::FONT_REGULAR;
    use widgets::theme::FONT_MEDIUM;
    use widgets::theme::FONT_SEMIBOLD;
    use widgets::theme::FONT_BOLD;
    // Semantic colors
    use widgets::theme::DARK_BG;
    use widgets::theme::PANEL_BG;
    use widgets::theme::ACCENT_BLUE;
    use widgets::theme::ACCENT_GREEN;
    use widgets::theme::ACCENT_INDIGO;
    use widgets::theme::TEXT_PRIMARY;
    use widgets::theme::TEXT_SECONDARY;
    use widgets::theme::TEXT_MUTED;
    use widgets::theme::DIVIDER;
    use widgets::theme::BORDER;
    use widgets::theme::HOVER_BG;
    use widgets::theme::WHITE;
    use widgets::theme::TRANSPARENT;
    // Palette colors
    use widgets::theme::SLATE_50;
    use widgets::theme::SLATE_200;
    use widgets::theme::SLATE_400;
    use widgets::theme::SLATE_500;
    use widgets::theme::SLATE_600;
    use widgets::theme::SLATE_700;
    use widgets::theme::SLATE_800;
    use widgets::theme::GRAY_300;
    use widgets::theme::GRAY_600;
    use widgets::theme::GRAY_700;
    use widgets::theme::INDIGO_100;

    use colang_shell::widgets::sidebar::Sidebar;
    use colang_shell::widgets::main_body::MainBody;

    // Dark theme colors (imported for shader use)
    use widgets::theme::DARK_BG_DARK;
    use widgets::theme::PANEL_BG_DARK;
    use widgets::theme::TEXT_PRIMARY_DARK;
    use widgets::theme::TEXT_SECONDARY_DARK;
    use widgets::theme::BORDER_DARK;
    use widgets::theme::HOVER_BG_DARK;
    use widgets::theme::DIVIDER_DARK;

    // ------------------------------------------------------------------------
    // App Window
    // ------------------------------------------------------------------------
    App = {{App}} {
        ui: <Window> {
            window: { title: "开朗英语", inner_size: vec2(1400, 900) }
            pass: { clear_color: (DARK_BG) }
            flow: Overlay

            body = <MainBody> {}

            sidebar_trigger_overlay = <View> {
                width: 28, height: 28
                abs_pos: vec2(18.0, 16.0)
                cursor: Hand
            }

            sidebar_menu_overlay = <View> {
                width: 250, height: Fit
                abs_pos: vec2(0.0, 52.0)
                visible: false
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        let bg = mix((SLATE_50), (SLATE_800), self.dark_mode);
                        sdf.fill(bg);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        let border = mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
                        sdf.stroke(border, 1.0);
                        return sdf.result;
                    }
                }

                sidebar_content = <Sidebar> {}
            }


            user_menu = <View> {
                width: 140, height: Fit
                abs_pos: vec2(1250.0, 55.0)
                visible: false
                padding: 6
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        let bg = mix((SLATE_50), (SLATE_800), self.dark_mode);
                        sdf.fill(bg);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                        let border = mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
                        sdf.stroke(border, 1.0);
                        return sdf.result;
                    }
                }
                flow: Down
                spacing: 2

                menu_profile_btn = <Button> {
                    width: Fill, height: Fit
                    padding: {top: 10, bottom: 10, left: 10, right: 10}
                    align: {x: 0.0, y: 0.5}
                    text: "Profile"
                    icon_walk: {width: 14, height: 14, margin: {right: 8}}
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/user.svg")
                        fn get_color(self) -> vec4 { return (SLATE_500); }
                    }
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: { font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((GRAY_700), (SLATE_200), self.dark_mode);
                        }
                    }
                    draw_bg: {
                        instance hover: 0.0
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let light_normal = (SLATE_50);
                            let light_hover = (SLATE_200);
                            let dark_normal = (SLATE_800);
                            let dark_hover = (SLATE_700);
                            let normal = mix(light_normal, dark_normal, self.dark_mode);
                            let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                            let color = mix(normal, hover_color, self.hover);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                            sdf.fill(color);
                            return sdf.result;
                        }
                    }
                }

                menu_divider = <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((BORDER), (BORDER_DARK), self.dark_mode);
                        }
                    }
                }

                menu_settings_btn = <Button> {
                    width: Fill, height: Fit
                    padding: {top: 10, bottom: 10, left: 10, right: 10}
                    align: {x: 0.0, y: 0.5}
                    text: "Settings"
                    icon_walk: {width: 14, height: 14, margin: {right: 8}}
                    draw_icon: {
                        svg_file: dep("crate://self/resources/icons/settings.svg")
                        fn get_color(self) -> vec4 { return (SLATE_500); }
                    }
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: { font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((GRAY_700), (SLATE_200), self.dark_mode);
                        }
                    }
                    draw_bg: {
                        instance hover: 0.0
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            let light_normal = (SLATE_50);
                            let light_hover = (SLATE_200);
                            let dark_normal = (SLATE_800);
                            let dark_hover = (SLATE_700);
                            let normal = mix(light_normal, dark_normal, self.dark_mode);
                            let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                            let color = mix(normal, hover_color, self.hover);
                            sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                            sdf.fill(color);
                            return sdf.result;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    user_menu_open: bool,
    #[rust]
    sidebar_menu_open: bool,
    #[rust]
    open_tabs: Vec<TabId>,
    #[rust]
    active_tab: Option<TabId>,
    #[rust]
    last_window_size: DVec2,
    #[rust]
    sidebar_animating: bool,
    #[rust]
    sidebar_animation_start: f64,
    #[rust]
    sidebar_slide_in: bool,
    // /// Registry of installed apps (populated on init)
    // #[rust]
    // app_registry: AppRegistry,
    /// Dark mode state
    #[rust]
    dark_mode: bool,
    /// Dark mode animation progress (0.0 = light, 1.0 = dark)
    #[rust]
    dark_mode_anim: f64,
    /// Whether dark mode animation is in progress
    #[rust]
    dark_mode_animating: bool,
    /// Animation start time
    #[rust]
    dark_mode_anim_start: f64,
    /// Whether initial theme has been applied (on first draw)
    #[rust]
    theme_initialized: bool,
    #[rust]
    auth_token: Option<String>,
    #[rust]
    desktop_auth_in_progress: bool,
    #[rust]
    desktop_auth_state: Option<String>,
    #[rust]
    desktop_auth_redirect_uri: Option<String>,
    #[rust]
    desktop_auth_rx: Option<mpsc::Receiver<DesktopAuthResult>>,
    #[rust]
    website_url: String,
    #[rust]
    debug_panel_width: f64,
    #[rust]
    debug_panel_dragging: bool,
    #[rust]
    debug_panel_drag_start_x: f64,
    #[rust]
    debug_panel_drag_start_width: f64,
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Initialize configuration directory and load config
        // This ensures ~/.colang directory exists on first run
        let config = Config::load().unwrap_or_default();
        self.website_url = config.website_url;

        // Initialize the app registry with all installed apps
        // self.app_registry.register(ConversationScreen::info());
        // self.app_registry.register(SettingsScreen::info());

        // Load user preferences and restore dark mode
        let prefs = Preferences::load();
        self.dark_mode = prefs.dark_mode;
        self.dark_mode_anim = if prefs.dark_mode { 1.0 } else { 0.0 };
        self.auth_token = prefs.auth_token.clone();
        self.desktop_auth_in_progress = false;
        self.desktop_auth_state = None;
        self.desktop_auth_redirect_uri = None;
        self.desktop_auth_rx = None;
        self.debug_panel_width = 400.0;
        self.debug_panel_dragging = false;
        self.debug_panel_drag_start_x = 0.0;
        self.debug_panel_drag_start_width = 400.0;

        // Initialize API clients with backend URL
        init_asset_api(&config.api_url);
        init_learn_api(&config.api_url, prefs.auth_token);
    }
}

// ============================================================================
// WIDGET REGISTRATION
// ============================================================================

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        widgets::live_design(cx);
        makepad_component::live_design(cx);
        colang_core::screens::live_design(cx);
        colang_shell::widgets::sidebar::live_design(cx);
        colang_shell::widgets::tabs::live_design(cx);
        colang_shell::widgets::main_body::live_design(cx);
    }
}

// ============================================================================
// EVENT HANDLING
// ============================================================================

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());

        // Initialize theme on first draw (widgets are ready)
        if !self.theme_initialized {
            if let Event::Draw(_) = event {
                self.theme_initialized = true;
                // Apply initial dark mode from preferences (full update)
                self.apply_dark_mode_panels(cx);
                self.apply_dark_mode_screens(cx);
                // Update header theme toggle icon
                self.update_theme_toggle_icon(cx);
                self.update_login_button_label(cx);
                // Set website URL for settings screen links
                self.ui
                    .settings_screen(ids!(
                        body.base.content_area.main_content.content.settings_screen
                    ))
                    .set_website_url(self.website_url.clone());
            }
        }

        // Window resize handling
        self.handle_window_resize(cx, event);

        // Sidebar animation
        if self.sidebar_animating {
            self.update_sidebar_animation(cx);
        }

        // Dark mode animation
        if self.dark_mode_animating {
            self.update_dark_mode_animation(cx);
        }

        // Extract actions
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => &[],
        };

        self.poll_desktop_auth(cx);

        // Handle hover events
        self.handle_sidebar_hover(cx, event);
        self.handle_theme_toggle(cx, event);
        self.handle_debug_button(cx, event);
        self.handle_debug_panel_resize(cx, event);

        // Handle click events
        self.handle_sidebar_clicks(cx, &actions);
        self.handle_login_clicks(cx, &actions);
        self.handle_close_app_clicks(cx, &actions);
        self.handle_mofa_hero_buttons(cx, event);
        self.handle_conversation_screen_buttons(cx, &actions);
        self.handle_review_screen_buttons(cx, &actions);
        self.handle_home_screen_buttons(cx, &actions);
        self.handle_tab_clicks(cx, &actions);
        self.handle_tab_close_clicks(cx, event);
        self.handle_settings_actions(cx, &actions);
    }
}

// ============================================================================
// WINDOW & LAYOUT METHODS
// ============================================================================

impl App {
    /// Handle window resize events
    fn handle_window_resize(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::WindowGeomChange(wg) = event {
            let new_size = wg.new_geom.inner_size;
            if new_size != self.last_window_size {
                self.last_window_size = new_size;
                self.update_overlay_positions(cx);
            }
        }

        if let Event::Draw(_) = event {
            let window_rect = self.ui.area().rect(cx);
            if window_rect.size.x > 0.0 && window_rect.size != self.last_window_size {
                self.last_window_size = window_rect.size;
                self.update_overlay_positions(cx);
            }
        }
    }

    /// Update overlay positions based on window size
    fn update_overlay_positions(&mut self, cx: &mut Cx) {
        let window_width = self.last_window_size.x;

        if window_width <= 0.0 {
            return;
        }

        let user_menu_x = window_width - 150.0;
        self.ui.view(ids!(user_menu)).apply_over(
            cx,
            live! {
                abs_pos: (dvec2(user_menu_x, 55.0))
            },
        );

        self.ui.redraw(cx);
    }
}

// ============================================================================
// USER MENU METHODS
// ============================================================================

impl App {
    /// Handle user menu hover
    fn handle_user_menu_hover(&mut self, cx: &mut Cx, event: &Event) {
        let user_btn = self.ui.view(ids!(body.base.header.user_profile_container));
        let user_menu = self.ui.view(ids!(user_menu));

        match event.hits(cx, user_btn.area()) {
            Hit::FingerHoverIn(_) => {
                if !self.user_menu_open {
                    self.user_menu_open = true;
                    user_menu.set_visible(cx, true);
                    self.ui.redraw(cx);
                }
            }
            _ => {}
        }

        if self.user_menu_open {
            if let Event::MouseMove(mm) = event {
                let btn_rect = user_btn.area().rect(cx);
                let menu_rect = user_menu.area().rect(cx);

                let in_btn = mm.abs.x >= btn_rect.pos.x - 5.0
                    && mm.abs.x <= btn_rect.pos.x + btn_rect.size.x + 5.0
                    && mm.abs.y >= btn_rect.pos.y - 5.0
                    && mm.abs.y <= btn_rect.pos.y + btn_rect.size.y + 10.0;

                let in_menu = mm.abs.x >= menu_rect.pos.x - 5.0
                    && mm.abs.x <= menu_rect.pos.x + menu_rect.size.x + 5.0
                    && mm.abs.y >= menu_rect.pos.y - 5.0
                    && mm.abs.y <= menu_rect.pos.y + menu_rect.size.y + 5.0;

                if !in_btn && !in_menu {
                    self.user_menu_open = false;
                    user_menu.set_visible(cx, false);
                    self.ui.redraw(cx);
                }
            }
        }
    }

    /// Handle user menu button clicks
    fn handle_user_menu_clicks(&mut self, cx: &mut Cx, actions: &[Action]) {
        if self
            .ui
            .button(ids!(user_menu.menu_profile_btn))
            .clicked(actions)
        {
            self.user_menu_open = false;
            self.ui.view(ids!(user_menu)).set_visible(cx, false);
            self.open_or_switch_tab(cx, TabId::Profile);
        }

        if self
            .ui
            .button(ids!(user_menu.menu_settings_btn))
            .clicked(actions)
        {
            self.user_menu_open = false;
            self.ui.view(ids!(user_menu)).set_visible(cx, false);
            self.open_or_switch_tab(cx, TabId::Settings);
        }
    }

    fn handle_login_clicks(&mut self, cx: &mut Cx, actions: &[Action]) {
        if self
            .ui
            .button(ids!(body.base.header.user_profile_container.login_btn))
            .clicked(actions)
        {
            if self.auth_token.is_some() {
                // Open user profile page when already logged in
                let config = Config::load().unwrap_or_default();
                let _ = webbrowser::open(&config.profile_url());
            } else {
                // Start desktop login flow when not logged in
                self.start_desktop_login(cx);
            }
        }
    }

    fn handle_close_app_clicks(&mut self, cx: &mut Cx, actions: &[Action]) {
        if self
            .ui
            .button(ids!(body.base.header.close_app_btn))
            .clicked(actions)
        {
            cx.quit();
        }
    }

    fn poll_desktop_auth(&mut self, cx: &mut Cx) {
        let rx = match &self.desktop_auth_rx {
            Some(rx) => rx,
            None => return,
        };

        let result = match rx.try_recv() {
            Ok(v) => v,
            Err(std::sync::mpsc::TryRecvError::Empty) => return,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.desktop_auth_rx = None;
                self.desktop_auth_in_progress = false;
                self.desktop_auth_state = None;
                self.desktop_auth_redirect_uri = None;
                self.update_login_button_label(cx);
                return;
            }
        };

        self.desktop_auth_rx = None;
        self.desktop_auth_in_progress = false;

        match result {
            DesktopAuthResult::Success(token) => {
                self.auth_token = Some(token.clone());
                println!("Desktop login succeeded, token: {}", token);
                // Update learn API client with new token
                set_learn_api_token(Some(token.clone()));
                let mut prefs = Preferences::load();
                prefs.auth_token = Some(token);
                let _ = prefs.save();
            }
            DesktopAuthResult::Error(message) => {
                eprintln!("Desktop login failed: {}", message);
            }
        }

        self.desktop_auth_state = None;
        self.desktop_auth_redirect_uri = None;
        self.update_login_button_label(cx);
    }

    fn update_login_button_label(&mut self, cx: &mut Cx) {
        let id = ids!(body.base.header.user_profile_container.login_btn);
        if self.desktop_auth_in_progress {
            self.ui
                .button(id)
                .apply_over(cx, live! { text: "登录中..." });
        } else if self.auth_token.is_some() {
            self.ui.button(id).apply_over(cx, live! { text: "已登录" });
        } else {
            self.ui.button(id).apply_over(cx, live! { text: "登录" });
        }
        self.ui.redraw(cx);
    }

    fn start_desktop_login(&mut self, cx: &mut Cx) {
        if self.desktop_auth_in_progress {
            return;
        }

        let state = Uuid::new_v4().to_string();
        let listener = match TcpListener::bind("127.0.0.1:0") {
            Ok(l) => l,
            Err(_) => return,
        };

        let _ = listener.set_nonblocking(true);
        let port = match listener.local_addr() {
            Ok(addr) => addr.port(),
            Err(_) => return,
        };

        let redirect_uri = format!("http://127.0.0.1:{}/auth/callback", port);

        // Load configuration
        let config = Config::load().unwrap_or_default();
        let website_url = config.website_url;
        let api_url = config.api_url;

        let authorize_url = format!(
            "{}/auth?redirect_uri={}&state={}",
            website_url.trim_end_matches('/'),
            encode_query_component(&redirect_uri),
            encode_query_component(&state)
        );

        println!("Opening browser for URL: {}", authorize_url);

        self.desktop_auth_in_progress = true;
        self.desktop_auth_state = Some(state.clone());
        self.desktop_auth_redirect_uri = Some(redirect_uri.clone());
        self.update_login_button_label(cx);

        let (tx, rx) = mpsc::channel::<DesktopAuthResult>();
        self.desktop_auth_rx = Some(rx);

        thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5 * 60);
            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let uri = {
                            let mut first_line = String::new();
                            let mut reader = BufReader::new(&stream);
                            let _ = reader.read_line(&mut first_line);
                            first_line
                                .split_whitespace()
                                .nth(1)
                                .unwrap_or("/")
                                .to_string()
                        };
                        let params = parse_query_params(&uri);
                        let code = params.get("code").cloned().unwrap_or_default();
                        let returned_state = params.get("state").cloned().unwrap_or_default();

                        let body = if code.is_empty() || returned_state.is_empty() {
                            "<html><body>Invalid request.</body></html>"
                        } else if returned_state != state {
                            "<html><body>Invalid state.</body></html>"
                        } else {
                            "<html><body>登录成功，可以关闭此页面。</body></html>"
                        };

                        let _ = write!(
                            stream,
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            body.as_bytes().len(),
                            body
                        );
                        let _ = stream.flush();

                        if code.is_empty() || returned_state.is_empty() {
                            let _ = tx.send(DesktopAuthResult::Error("missing code/state".into()));
                            break;
                        }
                        if returned_state != state {
                            let _ = tx.send(DesktopAuthResult::Error("invalid state".into()));
                            break;
                        }

                        let result = exchange_desktop_code(&api_url, &code, &redirect_uri)
                            .map(DesktopAuthResult::Success)
                            .unwrap_or_else(DesktopAuthResult::Error);
                        let _ = tx.send(result);
                        break;
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if Instant::now() > deadline {
                            let _ = tx.send(DesktopAuthResult::Error("timeout".into()));
                            break;
                        }
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(_) => {
                        let _ = tx.send(DesktopAuthResult::Error("listener failed".into()));
                        break;
                    }
                }
            }
        });

        let _ = webbrowser::open(&authorize_url);
    }

    /// Handle header theme toggle button
    fn handle_theme_toggle(&mut self, cx: &mut Cx, event: &Event) {
        let theme_btn = self.ui.view(ids!(body.base.header.theme_toggle));

        match event.hits(cx, theme_btn.area()) {
            Hit::FingerHoverIn(_) => {
                self.ui
                    .view(ids!(body.base.header.theme_toggle))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { hover: 1.0 }
                        },
                    );
                self.ui.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.ui
                    .view(ids!(body.base.header.theme_toggle))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { hover: 0.0 }
                        },
                    );
                self.ui.redraw(cx);
            }
            Hit::FingerUp(_) => {
                self.toggle_dark_mode(cx);
                self.update_theme_toggle_icon(cx);

                // Save preference to disk
                let mut prefs = Preferences::load();
                prefs.dark_mode = self.dark_mode;
                if let Err(e) = prefs.save() {
                    eprintln!("Failed to save dark mode preference: {}", e);
                }
            }
            _ => {}
        }
    }

    /// Update the theme toggle icon based on current mode
    fn update_theme_toggle_icon(&mut self, cx: &mut Cx) {
        let is_dark = self.dark_mode;
        self.ui
            .view(ids!(body.base.header.theme_toggle.sun_icon))
            .set_visible(cx, !is_dark);
        self.ui
            .view(ids!(body.base.header.theme_toggle.moon_icon))
            .set_visible(cx, is_dark);
        self.ui.redraw(cx);
    }

    /// Handle header debug button
    fn handle_debug_button(&mut self, cx: &mut Cx, event: &Event) {
        let debug_btn = self.ui.view(ids!(body.base.header.debug_btn));

        match event.hits(cx, debug_btn.area()) {
            Hit::FingerHoverIn(_) => {
                self.ui.view(ids!(body.base.header.debug_btn)).apply_over(
                    cx,
                    live! {
                        draw_bg: { hover: 1.0 }
                    },
                );
                self.ui.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.ui.view(ids!(body.base.header.debug_btn)).apply_over(
                    cx,
                    live! {
                        draw_bg: { hover: 0.0 }
                    },
                );
                self.ui.redraw(cx);
            }
            Hit::FingerUp(_) => {
                let panel_path = ids!(body.base.content_area.debug_panel);
                let splitter_path = ids!(body.base.content_area.debug_splitter);
                let debug_panel = self.ui.debug_panel(panel_path);
                let is_visible = debug_panel.is_visible();
                let new_visible = !is_visible;

                // Toggle visibility
                debug_panel.apply_over(
                    cx,
                    live! {
                        visible: (new_visible),
                        width: (self.debug_panel_width)
                    },
                );
                debug_panel.update_dark_mode(cx, self.dark_mode_anim);
                self.ui
                    .view(splitter_path)
                    .apply_over(cx, live! { visible: (new_visible) });
                self.ui.redraw(cx);
            }
            _ => {}
        }
    }

    fn handle_debug_panel_resize(&mut self, cx: &mut Cx, event: &Event) {
        let panel_path = ids!(body.base.content_area.debug_panel);
        let splitter_path = ids!(body.base.content_area.debug_splitter);

        if !self.ui.debug_panel(panel_path).is_visible() {
            self.debug_panel_dragging = false;
            self.ui.view(splitter_path).apply_over(
                cx,
                live! {
                    draw_bg: { hover: 0.0 }
                },
            );
            return;
        }

        let splitter = self.ui.view(splitter_path);
        match event.hits(cx, splitter.area()) {
            Hit::FingerHoverIn(_) => {
                self.ui.view(splitter_path).apply_over(
                    cx,
                    live! {
                        draw_bg: { hover: 1.0 }
                    },
                );
                self.ui.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                if !self.debug_panel_dragging {
                    self.ui.view(splitter_path).apply_over(
                        cx,
                        live! {
                            draw_bg: { hover: 0.0 }
                        },
                    );
                    self.ui.redraw(cx);
                }
            }
            Hit::FingerDown(fe) => {
                self.debug_panel_dragging = true;
                self.debug_panel_drag_start_x = fe.abs.x;
                self.debug_panel_drag_start_width = self.debug_panel_width;
            }
            Hit::FingerUp(_) => {
                self.debug_panel_dragging = false;
            }
            _ => {}
        }

        if self.debug_panel_dragging {
            if let Event::MouseMove(mm) = event {
                let delta = self.debug_panel_drag_start_x - mm.abs.x;
                let next = (self.debug_panel_drag_start_width + delta).clamp(240.0, 720.0);
                if (next - self.debug_panel_width).abs() > 0.5 {
                    self.debug_panel_width = next;
                    self.ui.debug_panel(panel_path).apply_over(
                        cx,
                        live! {
                            width: (next)
                        },
                    );
                    self.ui.redraw(cx);
                }
            }
        }
    }
}

// ============================================================================
// SIDEBAR METHODS
// ============================================================================

impl App {
    /// Handle sidebar hover
    fn handle_sidebar_hover(&mut self, cx: &mut Cx, event: &Event) {
        let sidebar_trigger = self.ui.view(ids!(sidebar_trigger_overlay));
        let sidebar_menu = self.ui.view(ids!(sidebar_menu_overlay));

        match event.hits(cx, sidebar_trigger.area()) {
            Hit::FingerHoverIn(_) => {
                if !self.sidebar_menu_open && !self.sidebar_animating {
                    self.sidebar_menu_open = true;
                    self.start_sidebar_slide_in(cx);
                }
            }
            _ => {}
        }

        if self.sidebar_menu_open && !self.sidebar_animating {
            if let Event::MouseMove(mm) = event {
                let trigger_rect = sidebar_trigger.area().rect(cx);
                let sidebar_rect = sidebar_menu.area().rect(cx);

                let in_trigger = mm.abs.x >= trigger_rect.pos.x - 5.0
                    && mm.abs.x <= trigger_rect.pos.x + trigger_rect.size.x + 5.0
                    && mm.abs.y >= trigger_rect.pos.y - 5.0
                    && mm.abs.y <= trigger_rect.pos.y + trigger_rect.size.y + 5.0;

                let in_sidebar = mm.abs.x >= sidebar_rect.pos.x - 5.0
                    && mm.abs.x <= sidebar_rect.pos.x + sidebar_rect.size.x + 10.0
                    && mm.abs.y >= sidebar_rect.pos.y - 5.0
                    && mm.abs.y <= sidebar_rect.pos.y + sidebar_rect.size.y + 5.0;

                if !in_trigger && !in_sidebar {
                    self.sidebar_menu_open = false;
                    self.start_sidebar_slide_out(cx);
                }
            }
        }
    }

    /// Handle sidebar menu item clicks
    fn handle_sidebar_clicks(&mut self, cx: &mut Cx, actions: &[Action]) {
        // Home tab
        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.home_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);
            // Stop any running timers
            self.ui
                .conversation_screen(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .stop_timers(cx);
            // Show home, hide others
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.reading_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.set_header_page_title(cx, "🏠", "首页");
            self.ui.redraw(cx);
        }

        // Colang tab
        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.dialog_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.reading_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .conversation_screen(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .start_timers(cx);
            self.set_header_page_title(cx, "💬", "交流对话");
            self.ui.redraw(cx);
        }
        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.review_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.reading_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.set_header_page_title(cx, "📚", "复习中心");
            self.ui.redraw(cx);
        }

        // Scene Center tab
        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.scenes_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);
            self.ui
                .conversation_screen(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .stop_timers(cx);
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.reading_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.set_header_page_title(cx, "🎭", "场景中心");
            self.ui.redraw(cx);
        }

        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.reading_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.navigate_to_reading_practice(cx);
        }

        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.dictionary_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.navigate_to_dictionary(cx);
        }

        // Settings tab
        if self
            .ui
            .button(ids!(sidebar_menu_overlay.sidebar_content.settings_tab))
            .clicked(actions)
        {
            self.sidebar_menu_open = false;
            self.start_sidebar_slide_out(cx);
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);
            self.ui
                .conversation_screen(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .stop_timers(cx);
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.reading_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.set_header_page_title(cx, "⚙️", "设置");
            self.ui.redraw(cx);
        }
    }
}

// ============================================================================
// ANIMATION METHODS
// ============================================================================

impl App {
    fn set_header_page_title(&mut self, cx: &mut Cx, icon: &str, title: &str) {
        self.ui
            .label(ids!(body.base.header.page_title_container.page_icon))
            .set_text(cx, icon);
        self.ui
            .label(ids!(body.base.header.page_title_container.page_title))
            .set_text(cx, title);
    }

    /// Update sidebar slide animation
    fn update_sidebar_animation(&mut self, cx: &mut Cx) {
        const ANIMATION_DURATION: f64 = 0.2;
        const SIDEBAR_WIDTH: f64 = 250.0;

        let elapsed = Cx::time_now() - self.sidebar_animation_start;
        let progress = (elapsed / ANIMATION_DURATION).min(1.0);
        let eased = 1.0 - (1.0 - progress).powi(3);

        let x = if self.sidebar_slide_in {
            -SIDEBAR_WIDTH * (1.0 - eased)
        } else {
            -SIDEBAR_WIDTH * eased
        };

        self.ui.view(ids!(sidebar_menu_overlay)).apply_over(
            cx,
            live! {
                abs_pos: (dvec2(x, 52.0))
            },
        );

        if progress >= 1.0 {
            self.sidebar_animating = false;
            if !self.sidebar_slide_in {
                self.ui
                    .view(ids!(sidebar_menu_overlay))
                    .set_visible(cx, false);
            }
        }

        self.ui.redraw(cx);
    }

    /// Start sidebar slide-in animation
    fn start_sidebar_slide_in(&mut self, cx: &mut Cx) {
        self.sidebar_animating = true;
        self.sidebar_animation_start = Cx::time_now();
        self.sidebar_slide_in = true;
        self.ui.view(ids!(sidebar_menu_overlay)).apply_over(
            cx,
            live! {
                abs_pos: (dvec2(-250.0, 52.0))
            },
        );
        self.ui
            .view(ids!(sidebar_menu_overlay))
            .set_visible(cx, true);
        self.ui
            .sidebar(ids!(sidebar_menu_overlay.sidebar_content))
            .restore_selection_state(cx);
        self.ui.redraw(cx);
    }

    /// Start sidebar slide-out animation
    fn start_sidebar_slide_out(&mut self, cx: &mut Cx) {
        self.sidebar_animating = true;
        self.sidebar_animation_start = Cx::time_now();
        self.sidebar_slide_in = false;
        self.ui.redraw(cx);
    }

    /// Toggle dark mode with animation
    pub fn toggle_dark_mode(&mut self, cx: &mut Cx) {
        self.dark_mode = !self.dark_mode;
        self.dark_mode_animating = true;
        self.dark_mode_anim_start = Cx::time_now();

        // Apply screens immediately at target value (snap, not animated)
        // This avoids calling update_dark_mode on every frame
        let target = if self.dark_mode { 1.0 } else { 0.0 };
        self.apply_dark_mode_screens_with_value(cx, target);

        self.ui.redraw(cx);
    }

    /// Set dark mode to a specific value with animation
    fn set_dark_mode(&mut self, cx: &mut Cx, is_dark: bool) {
        if self.dark_mode != is_dark {
            self.toggle_dark_mode(cx);
            self.update_theme_toggle_icon(cx);

            // Save preference to disk
            let mut prefs = Preferences::load();
            prefs.dark_mode = self.dark_mode;
            if let Err(e) = prefs.save() {
                eprintln!("Failed to save dark mode preference: {}", e);
            }
        }
    }

    /// Handle settings screen actions
    fn handle_settings_actions(&mut self, cx: &mut Cx, actions: &[Action]) {
        for action in actions {
            match action.as_widget_action().cast() {
                SettingsScreenAction::ThemeModeChanged(mode) => {
                    match mode {
                        ThemeMode::Light => {
                            self.set_dark_mode(cx, false);
                        }
                        ThemeMode::Dark => {
                            self.set_dark_mode(cx, true);
                        }
                        ThemeMode::System => {
                            // For now, default to light mode when "Follow System" is selected
                            // TODO: Implement actual system theme detection
                            self.set_dark_mode(cx, false);
                        }
                    }
                }
                SettingsScreenAction::OpenUrl(url) => {
                    let _ = webbrowser::open(&url);
                }
                _ => {}
            }
        }
    }

    /// Update dark mode animation
    fn update_dark_mode_animation(&mut self, cx: &mut Cx) {
        let elapsed = Cx::time_now() - self.dark_mode_anim_start;
        let duration = 0.3; // 300ms animation

        // Ease-out cubic
        let t = (elapsed / duration).min(1.0);
        let eased = 1.0 - (1.0 - t).powi(3);

        // Animate from current to target
        let target = if self.dark_mode { 1.0 } else { 0.0 };
        let start = if self.dark_mode { 0.0 } else { 1.0 };
        self.dark_mode_anim = start + (target - start) * eased;

        // During animation: only update main panels (no errors)
        // Full update with screens happens only at the end
        self.apply_dark_mode_panels(cx);

        if t >= 1.0 {
            self.dark_mode_animating = false;
            self.dark_mode_anim = target;
            // Apply to ALL widgets including screens at animation end
            self.apply_dark_mode_screens(cx);
        }

        self.ui.redraw(cx);
    }

    /// Apply dark mode to main panels only (safe for animation frames, no errors)
    fn apply_dark_mode_panels(&mut self, cx: &mut Cx) {
        let dm = self.dark_mode_anim;

        // Apply to main app background (MainBody)
        self.ui.view(ids!(body)).apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dm) }
            },
        );

        // Apply to header
        self.ui.view(ids!(body.base.header)).apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dm) }
            },
        );

        // Apply to sidebar menu overlay
        self.ui.view(ids!(sidebar_menu_overlay)).apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dm) }
            },
        );

        // Apply to sidebar content (this is safe, sidebar widget handles it internally)
        self.ui
            .sidebar(ids!(sidebar_menu_overlay.sidebar_content))
            .update_dark_mode(cx, dm);

        // Apply to user menu
        self.ui.view(ids!(user_menu)).apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dm) }
            },
        );

        // Apply to user menu buttons
        self.ui.button(ids!(user_menu.menu_profile_btn)).apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dm) }
                draw_text: { dark_mode: (dm) }
            },
        );
        self.ui
            .button(ids!(user_menu.menu_settings_btn))
            .apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dm) }
                    draw_text: { dark_mode: (dm) }
                },
            );
        self.ui.view(ids!(user_menu.menu_divider)).apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dm) }
            },
        );

        // Apply to close app button
        self.ui
            .view(ids!(body.base.header.close_app_btn))
            .apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dm) }
                },
            );

        // Apply to tab overlay - only when tabs are open
        if !self.open_tabs.is_empty() {
            self.ui.view(ids!(body.tab_overlay)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dm) }
                },
            );

            // Apply to tab bar
            self.ui.view(ids!(body.tab_overlay.tab_bar)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dm) }
                },
            );

            // Apply to tab widgets
            if self.open_tabs.contains(&TabId::Profile) {
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.profile_tab))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .label(ids!(body.tab_overlay.tab_bar.profile_tab.tab_label))
                    .apply_over(
                        cx,
                        live! {
                            draw_text: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.profile_tab.close_btn))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
            }

            if self.open_tabs.contains(&TabId::Settings) {
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.settings_tab))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .label(ids!(body.tab_overlay.tab_bar.settings_tab.tab_label))
                    .apply_over(
                        cx,
                        live! {
                            draw_text: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.settings_tab.close_btn))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
            }

            // Tab content backgrounds
            if self.open_tabs.contains(&TabId::Profile) {
                // Profile page background
                self.ui
                    .view(ids!(body.tab_overlay.tab_content.profile_page))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
                // Profile page internal widgets
                self.ui
                    .label(ids!(
                        body.tab_overlay.tab_content.profile_page.profile_title
                    ))
                    .apply_over(
                        cx,
                        live! {
                            draw_text: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .view(ids!(
                        body.tab_overlay.tab_content.profile_page.profile_divider
                    ))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .view(ids!(
                        body.tab_overlay
                            .tab_content
                            .profile_page
                            .profile_row
                            .profile_avatar
                    ))
                    .apply_over(
                        cx,
                        live! {
                            draw_bg: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .label(ids!(
                        body.tab_overlay
                            .tab_content
                            .profile_page
                            .profile_row
                            .profile_info
                            .profile_name
                    ))
                    .apply_over(
                        cx,
                        live! {
                            draw_text: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .label(ids!(
                        body.tab_overlay
                            .tab_content
                            .profile_page
                            .profile_row
                            .profile_info
                            .profile_email
                    ))
                    .apply_over(
                        cx,
                        live! {
                            draw_text: { dark_mode: (dm) }
                        },
                    );
                self.ui
                    .label(ids!(
                        body.tab_overlay
                            .tab_content
                            .profile_page
                            .profile_coming_soon
                    ))
                    .apply_over(
                        cx,
                        live! {
                            draw_text: { dark_mode: (dm) }
                        },
                    );
            }
        }
    }

    /// Apply dark mode to screens (may produce errors, called once at start/end only)
    fn apply_dark_mode_screens(&mut self, cx: &mut Cx) {
        self.apply_dark_mode_screens_with_value(cx, self.dark_mode_anim);
    }

    /// Apply dark mode to screens with a specific value
    fn apply_dark_mode_screens_with_value(&mut self, cx: &mut Cx, dm: f64) {
        // Apply to Colang screen (main content)
        self.ui
            .conversation_screen(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .conversation_screen
            ))
            .on_dark_mode_change(cx, dm);

        // Apply to Settings screen in main content
        self.ui
            .settings_screen(ids!(
                body.base.content_area.main_content.content.settings_screen
            ))
            .update_dark_mode(cx, dm);

        // Apply to tab overlay content - only when tabs are open
        if !self.open_tabs.is_empty() {
            if self.open_tabs.contains(&TabId::Settings) {
                self.ui
                    .settings_screen(ids!(body.tab_overlay.tab_content.settings_tab_page))
                    .update_dark_mode(cx, dm);
            }
        }
    }
}

// ============================================================================
// TAB MANAGEMENT METHODS
// ============================================================================

impl App {
    /// Open a tab or switch to it if already open
    fn open_or_switch_tab(&mut self, cx: &mut Cx, tab_id: TabId) {
        if !self.open_tabs.contains(&tab_id) {
            self.open_tabs.push(tab_id);
        }

        self.active_tab = Some(tab_id);
        self.update_tab_ui(cx);
    }

    /// Close a tab
    fn close_tab(&mut self, cx: &mut Cx, tab_id: TabId) {
        self.open_tabs.retain(|t| *t != tab_id);

        if self.active_tab == Some(tab_id) {
            self.active_tab = self.open_tabs.last().copied();
        }

        self.update_tab_ui(cx);
    }

    /// Handle tab widget clicks
    fn handle_tab_clicks(&mut self, cx: &mut Cx, actions: &[Action]) {
        if self
            .ui
            .view(ids!(body.tab_overlay.tab_bar.profile_tab))
            .finger_up(actions)
            .is_some()
        {
            if self.open_tabs.contains(&TabId::Profile) {
                self.active_tab = Some(TabId::Profile);
                self.update_tab_ui(cx);
            }
        }

        if self
            .ui
            .view(ids!(body.tab_overlay.tab_bar.settings_tab))
            .finger_up(actions)
            .is_some()
        {
            if self.open_tabs.contains(&TabId::Settings) {
                self.active_tab = Some(TabId::Settings);
                self.update_tab_ui(cx);
            }
        }
    }

    /// Handle tab close button clicks
    fn handle_tab_close_clicks(&mut self, cx: &mut Cx, event: &Event) {
        let profile_close = self
            .ui
            .view(ids!(body.tab_overlay.tab_bar.profile_tab.close_btn));
        match event.hits(cx, profile_close.area()) {
            Hit::FingerUp(_) => {
                self.close_tab(cx, TabId::Profile);
                return;
            }
            Hit::FingerHoverIn(_) => {
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.profile_tab.close_btn))
                    .apply_over(cx, live! { draw_bg: { hover: 1.0 } });
                self.ui.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.profile_tab.close_btn))
                    .apply_over(cx, live! { draw_bg: { hover: 0.0 } });
                self.ui.redraw(cx);
            }
            _ => {}
        }

        let settings_close = self
            .ui
            .view(ids!(body.tab_overlay.tab_bar.settings_tab.close_btn));
        match event.hits(cx, settings_close.area()) {
            Hit::FingerUp(_) => {
                self.close_tab(cx, TabId::Settings);
                return;
            }
            Hit::FingerHoverIn(_) => {
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.settings_tab.close_btn))
                    .apply_over(cx, live! { draw_bg: { hover: 1.0 } });
                self.ui.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.ui
                    .view(ids!(body.tab_overlay.tab_bar.settings_tab.close_btn))
                    .apply_over(cx, live! { draw_bg: { hover: 0.0 } });
                self.ui.redraw(cx);
            }
            _ => {}
        }
    }

    /// Update tab bar and content visibility
    fn update_tab_ui(&mut self, cx: &mut Cx) {
        let profile_open = self.open_tabs.contains(&TabId::Profile);
        let settings_open = self.open_tabs.contains(&TabId::Settings);
        let any_tabs_open = !self.open_tabs.is_empty();

        let profile_active = self.active_tab == Some(TabId::Profile);
        let settings_active = self.active_tab == Some(TabId::Settings);

        let _was_overlay_visible = self.ui.view(ids!(body.tab_overlay)).visible();

        self.ui
            .view(ids!(body.tab_overlay))
            .set_visible(cx, any_tabs_open);

        // Update tab visibility
        self.ui
            .view(ids!(body.tab_overlay.tab_bar.profile_tab))
            .set_visible(cx, profile_open);
        self.ui
            .view(ids!(body.tab_overlay.tab_bar.settings_tab))
            .set_visible(cx, settings_open);

        // Update profile tab active state
        let profile_active_val = if profile_active { 1.0 } else { 0.0 };
        self.ui
            .view(ids!(body.tab_overlay.tab_bar.profile_tab))
            .apply_over(cx, live! { draw_bg: { active: (profile_active_val) } });
        self.ui
            .label(ids!(body.tab_overlay.tab_bar.profile_tab.tab_label))
            .apply_over(cx, live! { draw_text: { active: (profile_active_val) } });

        // Update settings tab active state
        let settings_active_val = if settings_active { 1.0 } else { 0.0 };
        self.ui
            .view(ids!(body.tab_overlay.tab_bar.settings_tab))
            .apply_over(cx, live! { draw_bg: { active: (settings_active_val) } });
        self.ui
            .label(ids!(body.tab_overlay.tab_bar.settings_tab.tab_label))
            .apply_over(cx, live! { draw_text: { active: (settings_active_val) } });

        // Hide all content pages first
        self.ui
            .view(ids!(body.tab_overlay.tab_content.profile_page))
            .set_visible(cx, false);
        self.ui
            .view(ids!(body.tab_overlay.tab_content.settings_tab_page))
            .set_visible(cx, false);

        // Show active tab content
        match self.active_tab {
            Some(TabId::Profile) => {
                self.ui
                    .view(ids!(body.tab_overlay.tab_content.profile_page))
                    .set_visible(cx, true);
            }
            Some(TabId::Settings) => {
                self.ui
                    .view(ids!(body.tab_overlay.tab_content.settings_tab_page))
                    .set_visible(cx, true);
            }
            None => {
                if profile_open {
                    self.ui
                        .view(ids!(body.tab_overlay.tab_content.profile_page))
                        .set_visible(cx, true);
                } else if settings_open {
                    self.ui
                        .view(ids!(body.tab_overlay.tab_content.settings_tab_page))
                        .set_visible(cx, true);
                }
            }
        }

        self.ui.redraw(cx);
    }
}

// ============================================================================
// MOFA HERO METHODS
// ============================================================================

impl App {
    /// Handle MofaHero start/stop button clicks
    fn handle_mofa_hero_buttons(&mut self, cx: &mut Cx, event: &Event) {
        let start_view = self.ui.view(ids!(
            body.base
                .content_area
                .main_content
                .content
                .conversation_screen
                .mofa_hero
                .action_section
                .start_view
        ));
        match event.hits(cx, start_view.area()) {
            Hit::FingerUp(_) => {
                self.ui
                    .view(ids!(
                        body.base
                            .content_area
                            .main_content
                            .content
                            .conversation_screen
                            .mofa_hero
                            .action_section
                            .start_view
                    ))
                    .set_visible(cx, false);
                self.ui
                    .view(ids!(
                        body.base
                            .content_area
                            .main_content
                            .content
                            .conversation_screen
                            .mofa_hero
                            .action_section
                            .stop_view
                    ))
                    .set_visible(cx, true);
                self.ui.redraw(cx);
            }
            _ => {}
        }
        let stop_view = self.ui.view(ids!(
            body.base
                .content_area
                .main_content
                .content
                .conversation_screen
                .mofa_hero
                .action_section
                .stop_view
        ));
        match event.hits(cx, stop_view.area()) {
            Hit::FingerUp(_) => {
                self.ui
                    .view(ids!(
                        body.base
                            .content_area
                            .main_content
                            .content
                            .conversation_screen
                            .mofa_hero
                            .action_section
                            .start_view
                    ))
                    .set_visible(cx, true);
                self.ui
                    .view(ids!(
                        body.base
                            .content_area
                            .main_content
                            .content
                            .conversation_screen
                            .mofa_hero
                            .action_section
                            .stop_view
                    ))
                    .set_visible(cx, false);
                self.ui.redraw(cx);
            }
            _ => {}
        }
    }
}

fn encode_query_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        let ok = matches!(
            b,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~'
        );
        if ok {
            out.push(b as char);
        } else {
            out.push('%');
            out.push_str(&format!("{:02X}", b));
        }
    }
    out
}

fn decode_query_component(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let h1 = bytes[i + 1];
                let h2 = bytes[i + 2];
                let v1 = (h1 as char).to_digit(16);
                let v2 = (h2 as char).to_digit(16);
                if let (Some(v1), Some(v2)) = (v1, v2) {
                    out.push(((v1 << 4) | v2) as u8);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn parse_query_params(uri: &str) -> HashMap<String, String> {
    let query = match uri.split_once('?') {
        Some((_, q)) => q,
        None => return HashMap::new(),
    };
    let mut out = HashMap::new();
    for part in query.split('&') {
        if part.is_empty() {
            continue;
        }
        let (k, v) = match part.split_once('=') {
            Some((k, v)) => (k, v),
            None => (part, ""),
        };
        out.insert(decode_query_component(k), decode_query_component(v));
    }
    out
}

#[derive(Serialize)]
struct ConsumeDesktopCodeRequest {
    code: String,
    redirect_uri: String,
}

#[derive(Deserialize)]
struct ConsumeDesktopCodeResponse {
    access_token: String,
}

fn exchange_desktop_code(api_url: &str, code: &str, redirect_uri: &str) -> Result<String, String> {
    let endpoint = format!("{}/auth/consume", api_url.trim_end_matches('/'));

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
            headers
        })
        .build()
        .map_err(|e| e.to_string())?;

    let res = client
        .post(endpoint)
        .json(&ConsumeDesktopCodeRequest {
            code: code.to_string(),
            redirect_uri: redirect_uri.to_string(),
        })
        .send()
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().unwrap_or_default();
        return Err(if body.is_empty() {
            format!("HTTP {}", status.as_u16())
        } else {
            body
        });
    }

    let data: ConsumeDesktopCodeResponse = res.json().map_err(|e| e.to_string())?;
    Ok(data.access_token)
}

// ============================================================================
// DIALOG SCENE BUTTON HANDLERS
// ============================================================================

impl App {
    fn handle_conversation_screen_buttons(&mut self, cx: &mut Cx, actions: &[Action]) {
        // Handle "选择场景" button click
        if self
            .ui
            .button(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .conversation_screen
                    .main_layout
                    .left_column
                    .chat_container
                    .chat_info
                    .chat_actions
                    .select_scene_btn
            ))
            .clicked(actions)
        {
            // Navigate to scene center
            self.sidebar_menu_open = false;
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);

            // Stop dialog scene timers
            self.ui
                .conversation_screen(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .stop_timers(cx);

            // Hide dialog scene, show scene center
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.set_header_page_title(cx, "🎭", "场景中心");

            // Hide other scenes
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });

            self.ui.redraw(cx);
        }
    }

    fn handle_review_screen_buttons(&mut self, cx: &mut Cx, actions: &[Action]) {
        // Handle "选择场景" button click
        if self
            .ui
            .button(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .review_screen
                    .content_scroll
                    .content
                    .header_row
                    .select_scene_btn
            ))
            .clicked(actions)
        {
            // Navigate to scene center
            self.sidebar_menu_open = false;
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);

            // Hide review scene, show scene center
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.set_header_page_title(cx, "🎭", "场景中心");

            // Hide other scenes
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });

            self.ui.redraw(cx);
        }
    }

    fn handle_home_screen_buttons(&mut self, cx: &mut Cx, actions: &[Action]) {
        // Handle "选择场景" button click
        if self
            .ui
            .button(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .left_column
                    .welcome_card
                    .select_scene_btn
            ))
            .clicked(actions)
        {
            // Navigate to scene center
            self.sidebar_menu_open = false;
            self.open_tabs.clear();
            self.active_tab = None;
            self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);

            // Hide home scene, show scene center
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.home_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.scenes_screen
                ))
                .apply_over(cx, live! { visible: true });
            self.set_header_page_title(cx, "🎭", "场景中心");

            // Hide other scenes
            self.ui
                .view(ids!(
                    body.base
                        .content_area
                        .main_content
                        .content
                        .conversation_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.review_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.reading_screen
                ))
                .apply_over(cx, live! { visible: false });
            self.ui
                .view(ids!(
                    body.base.content_area.main_content.content.settings_screen
                ))
                .apply_over(cx, live! { visible: false });

            self.ui.redraw(cx);
        }

        // Handle "开始对话" button - navigate to dialog screen
        if self
            .ui
            .button(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .left_column
                    .welcome_card
                    .welcome_content
                    .start_button
            ))
            .clicked(actions)
        {
            self.navigate_to_conversation_screen(cx);
        }

        // Handle quick action: 场景模拟 - navigate to scene center
        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .left_column
                    .actions_card
                    .actions_row
                    .action_scenario
                    .action_panel
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_scenario(cx);
        }

        // Handle quick action: 经典对白 - navigate to classic dialogues
        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .left_column
                    .actions_card
                    .actions_row
                    .action_dialogue
                    .action_panel
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_scenario(cx);
        }

        // Handle quick action: 跟读练习 - navigate to reading practice
        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .left_column
                    .actions_card
                    .actions_row
                    .action_reading
                    .action_panel
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_reading_practice(cx);
        }

        // Handle quick action: 实时助手 - navigate to assistant
        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .left_column
                    .actions_card
                    .actions_row
                    .action_assistant
                    .action_panel
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_conversation_screen(cx);
        }

        // Handle scenario cards
        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .right_column
                    .scenes_card
                    .scenes_row
                    .scenehotel
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_scenario(cx);
        }

        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .right_column
                    .scenes_card
                    .scenes_row
                    .scenerestaurant
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_scenario(cx);
        }

        if self
            .ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .home_screen
                    .content_scroll
                    .content
                    .right_column
                    .scenes_card
                    .scenes_row
                    .sceneinterview
            ))
            .finger_up(actions)
            .is_some()
        {
            self.navigate_to_scenario(cx);
        }
    }

    // Helper navigation methods
    fn navigate_to_conversation_screen(&mut self, cx: &mut Cx) {
        self.hide_all_screens(cx);
        self.ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .conversation_screen
            ))
            .apply_over(cx, live! { visible: true });
        self.ui
            .conversation_screen(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .conversation_screen
            ))
            .start_timers(cx);
        self.set_header_page_title(cx, "💬", "交流对话");
        self.ui.redraw(cx);
    }

    fn navigate_to_scenario(&mut self, cx: &mut Cx) {
        self.hide_all_screens(cx);
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.scenes_screen
            ))
            .apply_over(cx, live! { visible: true });
        self.set_header_page_title(cx, "🎭", "场景中心");
        self.ui.redraw(cx);
    }

    fn navigate_to_reading_practice(&mut self, cx: &mut Cx) {
        self.hide_all_screens(cx);
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.reading_screen
            ))
            .apply_over(cx, live! { visible: true });
        self.set_header_page_title(cx, "🎤", "跟读练习");
        self.ui.redraw(cx);
    }

    fn navigate_to_dictionary(&mut self, cx: &mut Cx) {
        self.hide_all_screens(cx);
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.dictionary_screen
            ))
            .apply_over(cx, live! { visible: true });
        self.set_header_page_title(cx, "📖", "词典查询");
        self.ui.redraw(cx);
    }

    fn hide_all_screens(&mut self, cx: &mut Cx) {
        self.sidebar_menu_open = false;
        self.open_tabs.clear();
        self.active_tab = None;
        self.ui.view(ids!(body.tab_overlay)).set_visible(cx, false);

        // Stop dialog timers if running
        self.ui
            .conversation_screen(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .conversation_screen
            ))
            .stop_timers(cx);

        // Hide all screens
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.home_screen
            ))
            .apply_over(cx, live! { visible: false });
        self.ui
            .view(ids!(
                body.base
                    .content_area
                    .main_content
                    .content
                    .conversation_screen
            ))
            .apply_over(cx, live! { visible: false });
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.review_screen
            ))
            .apply_over(cx, live! { visible: false });
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.scenes_screen
            ))
            .apply_over(cx, live! { visible: false });
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.reading_screen
            ))
            .apply_over(cx, live! { visible: false });
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.dictionary_screen
            ))
            .apply_over(cx, live! { visible: false });
        self.ui
            .view(ids!(
                body.base.content_area.main_content.content.settings_screen
            ))
            .apply_over(cx, live! { visible: false });
    }
}

// ============================================================================
// APP ENTRY POINT
// ============================================================================

app_main!(App);
