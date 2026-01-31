use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Import fonts and colors from shared theme (single source of truth)
    use colang_widgets::theme::FONT_REGULAR;
    use colang_widgets::theme::FONT_MEDIUM;
    use colang_widgets::theme::FONT_BOLD;
    use colang_widgets::theme::SLATE_50;
    use colang_widgets::theme::SLATE_200;
    use colang_widgets::theme::SLATE_400;
    use colang_widgets::theme::SLATE_500;
    use colang_widgets::theme::SLATE_600;
    use colang_widgets::theme::SLATE_700;
    use colang_widgets::theme::SLATE_800;
    use colang_widgets::theme::SLATE_900;

    // Orange accent colors matching website (from-orange-500 to-amber-500)
    ORANGE_50 = #fff7ed
    ORANGE_100 = #ffedd5
    ORANGE_400 = #fb923c
    ORANGE_500 = #f97316
    ORANGE_600 = #ea580c
    ORANGE_900 = #7c2d12
    AMBER_500 = #f59e0b
    AMBER_400 = #fbbf24

    // Additional slate colors for dark mode readability
    SLATE_200 = #e2e8f0
    SLATE_300 = #cbd5e1
    use colang_widgets::theme::DIVIDER;
    use colang_widgets::theme::DIVIDER_DARK;
    use colang_widgets::theme::TEXT_PRIMARY_DARK;
    use colang_widgets::theme::TEXT_SECONDARY_DARK;

    // Custom sidebar button matching website Header.tsx navigation style
    // - Active: orange gradient background with white text
    // - Inactive: gray text with hover:bg-orange-50
    pub SidebarMenuButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 10, bottom: 10, left: 14, right: 14}
        margin: 0
        align: {x: 0.0, y: 0.5}
        icon_walk: {width: 18, height: 18, margin: {right: 10}}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // When selected: orange gradient (from-orange-500 to-amber-500)
                // When hover: orange-50 tint
                // Normal: transparent/slate background
                if self.selected > 0.5 {
                    // Gradient from orange-500 to amber-500 (left to right)
                    let orange_500 = vec4(0.976, 0.451, 0.086, 1.0); // #f97316
                    let amber_500 = vec4(0.961, 0.620, 0.043, 1.0);  // #f59e0b
                    let gradient_color = mix(orange_500, amber_500, self.pos.x);
                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill(gradient_color);
                } else {
                    // Light mode: transparent -> orange-50 (hover)
                    // Dark mode: slate-800 -> slate-700 (hover)
                    let light_normal = vec4(0.0, 0.0, 0.0, 0.0);
                    let light_hover = vec4(1.0, 0.969, 0.929, 1.0); // orange-50
                    let dark_normal = (SLATE_800);
                    let dark_hover = (SLATE_700);

                    let normal = mix(light_normal, dark_normal, self.dark_mode);
                    let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                    let color = mix(normal, hover_color, self.hover);

                    sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 8.0);
                    sdf.fill(color);
                }
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance selected: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }

            fn get_color(self) -> vec4 {
                // Selected: white text
                // Normal light: gray-600
                // Normal dark: slate-300
                if self.selected > 0.5 {
                    return vec4(1.0, 1.0, 1.0, 1.0); // white
                }
                let light_color = vec4(0.298, 0.333, 0.388, 1.0); // gray-600 (#4b5563)
                let dark_color = (SLATE_300);
                return mix(light_color, dark_color, self.dark_mode);
            }
        }

        draw_icon: {
            instance dark_mode: 0.0
            instance selected: 0.0
            fn get_color(self) -> vec4 {
                // Selected: white icons
                // Normal: orange-500 icons (matching website)
                if self.selected > 0.5 {
                    return vec4(1.0, 1.0, 1.0, 1.0); // white
                }
                let light_color = vec4(0.976, 0.451, 0.086, 1.0); // orange-500
                let dark_color = (ORANGE_400);
                return mix(light_color, dark_color, self.dark_mode);
            }
        }
    }


    // Main sidebar container - matches website Header.tsx nav style
    // Clean white/slate background with proper padding
    pub Sidebar = {{Sidebar}} {
        width: Fill, height: Fit
        flow: Down
        spacing: 2.0
        padding: {top: 16, bottom: 16, left: 12, right: 12}
        margin: 0

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // White background in light mode, slate-800 in dark mode
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0); // white
                let dark_bg = (SLATE_800);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill(mix(light_bg, dark_bg, self.dark_mode));
                return sdf.result;
            }
        }

        // Logo area spacer
        logo_area = <View> {
            width: Fill, height: 8
        }

        // Navigation buttons - labels match website Header.tsx navItems
        home_tab = <SidebarMenuButton> {
            text: "首页"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/home.svg")
            }
        }

        dialog_tab = <SidebarMenuButton> {
            text: "天天唠嗑"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/chat.svg")
            }
        }

        review_tab = <SidebarMenuButton> {
            text: "温故知新"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/reset.svg")
            }
        }

        scenes_tab = <SidebarMenuButton> {
            text: "角色扮演"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/radio.svg")
            }
        }

        reading_tab = <SidebarMenuButton> {
            text: "大声跟读"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/mic.svg")
            }
        }

        dictionary_tab = <SidebarMenuButton> {
            text: "词典翻译"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/book.svg")
            }
        }

        // Divider before settings
        <View> {
            width: Fill, height: 1
            margin: {top: 12, bottom: 12, left: 8, right: 8}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let light = vec4(0.898, 0.906, 0.922, 1.0); // gray-200
                    let dark = (SLATE_700);
                    return mix(light, dark, self.dark_mode);
                }
            }
        }

        settings_tab = <SidebarMenuButton> {
            text: "设置"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/settings.svg")
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum SidebarSelection {
    Home,
    Dialog,
    Review,
    Scenes,
    Reading,
    Dictionary,
    Settings,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum SidebarAction {
    None,
    ToggleTheme,
}

#[derive(Live, LiveHook, Widget)]
pub struct Sidebar {
    #[deref]
    view: View,

    #[rust]
    selection: Option<SidebarSelection>, // Track current selection
}

impl Widget for Sidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        if self.view.button(ids!(home_tab)).clicked(actions) {
            println!("Home tab clicked");
            self.handle_selection(cx, SidebarSelection::Home);
        }

        if self.view.button(ids!(dialog_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Dialog);
        }

        if self.view.button(ids!(review_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Review);
        }

        if self.view.button(ids!(scenes_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Scenes);
        }

        if self.view.button(ids!(reading_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Reading);
        }

        if self.view.button(ids!(dictionary_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Dictionary);
        }

        // Handle Settings tab click
        if self.view.button(ids!(settings_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Settings);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Sidebar {
    fn handle_selection(&mut self, cx: &mut Cx, selection: SidebarSelection) {
        self.selection = Some(selection.clone());

        // Clear all selections first
        self.clear_all_selections(cx);

        // Apply selected state based on what was clicked (bg, icon, and text)
        match &selection {
            SidebarSelection::Home => {
                self.view
                    .button(ids!(home_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
            SidebarSelection::Dialog => {
                self.view
                    .button(ids!(dialog_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
            SidebarSelection::Review => {
                self.view
                    .button(ids!(review_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
            SidebarSelection::Scenes => {
                self.view
                    .button(ids!(scenes_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
            SidebarSelection::Reading => {
                self.view
                    .button(ids!(reading_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
            SidebarSelection::Dictionary => {
                self.view
                    .button(ids!(dictionary_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
            SidebarSelection::Settings => {
                self.view
                    .button(ids!(settings_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
            }
        }

        self.view.redraw(cx);
    }

    fn clear_all_selections(&mut self, cx: &mut Cx) {
        // Macro to clear selection on multiple buttons (bg, icon, and text)
        macro_rules! clear_selection {
            ($self:expr, $cx:expr, $($path:expr),+ $(,)?) => {
                $( $self.view.button($path).apply_over($cx, live!{ draw_bg: { selected: 0.0 }, draw_icon: { selected: 0.0 }, draw_text: { selected: 0.0 } }); )+
            };
        }

        // Clear all nav items
        clear_selection!(
            self,
            cx,
            ids!(home_tab),
            ids!(dialog_tab),
            ids!(review_tab),
            ids!(scenes_tab),
            ids!(reading_tab),
            ids!(dictionary_tab),
            ids!(settings_tab)
        );
    }
}

impl SidebarRef {
    /// Restore the selection visual state (call when sidebar becomes visible)
    pub fn restore_selection_state(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            // First clear all selections
            inner.clear_all_selections(cx);

            // Then restore based on current selection
            if let Some(selection) = inner.selection.clone() {
                match selection {
                    SidebarSelection::Home => {
                        inner
                            .view
                            .button(ids!(home_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                    SidebarSelection::Dialog => {
                        inner
                            .view
                            .button(ids!(dialog_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                    SidebarSelection::Review => {
                        inner
                            .view
                            .button(ids!(review_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                    SidebarSelection::Scenes => {
                        inner
                            .view
                            .button(ids!(scenes_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                    SidebarSelection::Reading => {
                        inner
                            .view
                            .button(ids!(reading_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                    SidebarSelection::Dictionary => {
                        inner
                            .view
                            .button(ids!(dictionary_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                    SidebarSelection::Settings => {
                        inner
                            .view
                            .button(ids!(settings_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } });
                    }
                }
            }
            inner.view.redraw(cx);
        }
    }

    /// Update dark mode for this widget
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Sidebar background
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Update all tabs with dark mode
            let tabs = [
                ids!(home_tab),
                ids!(dialog_tab),
                ids!(review_tab),
                ids!(scenes_tab),
                ids!(reading_tab),
                ids!(dictionary_tab),
                ids!(settings_tab),
            ];

            for tab_id in tabs {
                inner.view.button(tab_id).apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                        draw_text: { dark_mode: (dark_mode) }
                        draw_icon: { dark_mode: (dark_mode) }
                    },
                );
            }

            inner.view.redraw(cx);
        }
    }

    /// Update sidebar selection based on current route path
    pub fn select_by_path(&self, cx: &mut Cx, path: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Determine selection based on path
            let selection = if path == "/" {
                SidebarSelection::Home
            } else if path.starts_with("/chat") {
                SidebarSelection::Dialog
            } else if path.starts_with("/review") {
                SidebarSelection::Review
            } else if path.starts_with("/scenes") {
                SidebarSelection::Scenes
            } else if path.starts_with("/reading") {
                SidebarSelection::Reading
            } else if path.starts_with("/dictionary") {
                SidebarSelection::Dictionary
            } else if path.starts_with("/settings") {
                SidebarSelection::Settings
            } else {
                return; // Unknown path
            };

            // Update selection state
            inner.selection = Some(selection.clone());
            inner.clear_all_selections(cx);

            // Apply selected state (bg, icon, and text)
            let tab_id = match selection {
                SidebarSelection::Home => ids!(home_tab),
                SidebarSelection::Dialog => ids!(dialog_tab),
                SidebarSelection::Review => ids!(review_tab),
                SidebarSelection::Scenes => ids!(scenes_tab),
                SidebarSelection::Reading => ids!(reading_tab),
                SidebarSelection::Dictionary => ids!(dictionary_tab),
                SidebarSelection::Settings => ids!(settings_tab),
            };

            inner.view.button(tab_id).apply_over(
                cx,
                live! { draw_bg: { selected: 1.0 }, draw_icon: { selected: 1.0 }, draw_text: { selected: 1.0 } },
            );
            inner.view.redraw(cx);
        }
    }
}

// Navigation uses button clicks, handled in app.rs
