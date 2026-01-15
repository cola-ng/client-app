use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Import fonts and colors from shared theme (single source of truth)
    use widgets::theme::FONT_REGULAR;
    use widgets::theme::FONT_BOLD;
    use widgets::theme::SLATE_50;
    use widgets::theme::SLATE_200;
    use widgets::theme::SLATE_400;
    use widgets::theme::SLATE_500;
    use widgets::theme::SLATE_600;
    use widgets::theme::SLATE_700;
    use widgets::theme::SLATE_800;
    use widgets::theme::SLATE_900;
    use widgets::theme::BLUE_100;
    use widgets::theme::BLUE_900;
    use widgets::theme::DIVIDER;
    use widgets::theme::DIVIDER_DARK;
    use widgets::theme::AMBER_500;
    use widgets::theme::INDIGO_500;
    use widgets::theme::TEXT_PRIMARY_DARK;
    use widgets::theme::TEXT_SECONDARY_DARK;

    // Chevron icon for expand/collapse
    ChevronRight = <Icon> {
        draw_icon: {
            svg_file: dep("crate://makepad-widgets/resources/icons/arrow.svg")
            color: (SLATE_400)
        }
        icon_walk: {width: 10, height: 10}
    }

    // Chevron pointing down (rotated)
    ChevronDown = <Icon> {
        draw_icon: {
            svg_file: dep("crate://makepad-widgets/resources/icons/arrow.svg")
            color: (SLATE_400)
            fn get_rotation_z(self) -> f64 {
                return 90.0;
            }
        }
        icon_walk: {width: 10, height: 10}
    }

    // Custom sidebar button using Button instead of RadioButton - with dark mode
    pub SidebarMenuButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 12, bottom: 12, left: 12, right: 12}
        margin: 0
        align: {x: 0.0, y: 0.5}
        icon_walk: {width: 20, height: 20, margin: {right: 12}}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Light mode: SLATE_50 -> SLATE_200 (hover) -> BLUE_100 (selected)
                // Dark mode: SLATE_800 -> SLATE_700 (hover) -> BLUE_900 (selected)
                let light_normal = (SLATE_50);
                let light_hover = (SLATE_200);
                let light_selected = (BLUE_100);
                let dark_normal = (SLATE_800);
                let dark_hover = (SLATE_700);
                let dark_selected = (BLUE_900);
                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);
                let selected_color = mix(light_selected, dark_selected, self.dark_mode);
                let color = mix(
                    mix(normal, hover_color, self.hover),
                    selected_color,
                    self.selected
                );
                sdf.box(2.0, 2.0, self.rect_size.x - 4.0, self.rect_size.y - 4.0, 6.0);
                sdf.fill(color);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }

            fn get_color(self) -> vec4 {
                return mix((SLATE_500), (SLATE_400), self.dark_mode);
            }
        }

        draw_icon: {
            fn get_color(self) -> vec4 {
                return (SLATE_500);
            }
        }
    }


    // Main sidebar container - with dark mode support
    // Height is Fit so sidebar adapts to content (compact when collapsed)
    pub Sidebar = {{Sidebar}} {
        width: Fill, height: Fit
        flow: Down
        spacing: 4.0
        padding: {top: 15, bottom: 15, left: 10, right: 10}
        margin: 0

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Main rectangle with subtle rounded corners
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let bg = mix((SLATE_50), (SLATE_800), self.dark_mode);
                sdf.fill(bg);

                return sdf.result;
            }
        }

        // Logo area (empty spacer)
        logo_area = <View> {
            width: Fill, height: 5
        }

        // Navigation buttons
        home_tab = <SidebarMenuButton> {
            text: "首页"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/home.svg")
            }
        }

        dialog_tab = <SidebarMenuButton> {
            text: "交流对话"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/colang.svg")
            }
        }

        review_tab = <SidebarMenuButton> {
            text: "复习巩固"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/fm.svg")
            }
        }

        scene_center_tab = <SidebarMenuButton> {
            text: "场景中心"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/colang.svg")
            }
        }

        reading_tab = <SidebarMenuButton> {
            text: "跟读练习"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/mic.svg")
            }
        }

        // Divider before settings
        <View> {
            width: Fill, height: 1
            margin: {top: 8, bottom: 8}
            show_bg: true
            draw_bg: { color: (DIVIDER) }
        }

        settings_tab = <SidebarMenuButton> {
            text: "Settings"
            draw_icon: {
                svg_file: dep("crate://self/resources/icons/settings.svg")
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum SidebarSelection {
    Home,
    Colang,
    MofaFM,
    SceneCenter,
    Reading,
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
            self.handle_selection(cx, SidebarSelection::Colang);
        }

        if self.view.button(ids!(review_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::MofaFM);
        }

        if self.view.button(ids!(scene_center_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::SceneCenter);
        }

        if self.view.button(ids!(reading_tab)).clicked(actions) {
            self.handle_selection(cx, SidebarSelection::Reading);
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

        // Apply selected state based on what was clicked
        match &selection {
            SidebarSelection::Home => {
                println!("Home selected");
                self.view
                    .button(ids!(home_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::Colang => {
                println!("Colang selected");
                self.view
                    .button(ids!(dialog_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::MofaFM => {
                println!("MofaFM selected");
                self.view
                    .button(ids!(review_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::SceneCenter => {
                println!("SceneCenter selected");
                self.view
                    .button(ids!(scene_center_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::Reading => {
                self.view
                    .button(ids!(reading_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
            }
            SidebarSelection::Settings => {
                self.view
                    .button(ids!(settings_tab))
                    .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
            }
        }

        self.view.redraw(cx);
    }

    fn clear_all_selections(&mut self, cx: &mut Cx) {
        // Macro to clear selection on multiple buttons
        macro_rules! clear_selection {
            ($self:expr, $cx:expr, $($path:expr),+ $(,)?) => {
                $( $self.view.button($path).apply_over($cx, live!{ draw_bg: { selected: 0.0 } }); )+
            };
        }

        // Clear all nav items
        clear_selection!(
            self,
            cx,
            ids!(home_tab),
            ids!(dialog_tab),
            ids!(review_tab),
            ids!(scene_center_tab),
            ids!(reading_tab),
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
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
                    }
                    SidebarSelection::Colang => {
                        inner
                            .view
                            .button(ids!(dialog_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
                    }
                    SidebarSelection::MofaFM => {
                        inner
                            .view
                            .button(ids!(review_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
                    }
                    SidebarSelection::SceneCenter => {
                        inner
                            .view
                            .button(ids!(scene_center_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
                    }
                    SidebarSelection::Reading => {
                        inner
                            .view
                            .button(ids!(reading_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
                    }
                    SidebarSelection::Settings => {
                        inner
                            .view
                            .button(ids!(settings_tab))
                            .apply_over(cx, live! { draw_bg: { selected: 1.0 } });
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

            // Colang
            inner.view.button(ids!(dialog_tab)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                    draw_text: { dark_mode: (dark_mode) }
                },
            );

            inner.view.button(ids!(review_tab)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                    draw_text: { dark_mode: (dark_mode) }
                },
            );

            inner.view.button(ids!(reading_tab)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                    draw_text: { dark_mode: (dark_mode) }
                },
            );

            // Settings tab
            inner.view.button(ids!(settings_tab)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                    draw_text: { dark_mode: (dark_mode) }
                },
            );

            inner.view.redraw(cx);
        }
    }
}

// Navigation uses button clicks, handled in app.rs
