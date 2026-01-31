//! Providers Panel - List of AI providers

use makepad_widgets::*;
use makepad_component::*;

use crate::models::ProviderId;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    ICO_OPENAI = dep("crate://self/resources/icons/openai.svg")
    ICO_DEEPSEEK = dep("crate://self/resources/icons/deepseek.svg")
    IMG_QWEN = dep("crate://self/resources/icons/qwen.png")

    // Provider item - matching moly-ai pattern
    ProviderItemBg = <RoundedView> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        margin: 0
        show_bg: true
        draw_bg: {
            border_radius: 0
        }
        flow: Right
        align: {x: 0.0, y: 0.5}
    }

    // Add provider button
    AddProviderButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let light_base = mix((SLATE_50), (HOVER_BG), self.hover);
                let dark_base = mix((SLATE_800), (SLATE_700), self.hover);
                let base = mix(light_base, dark_base, self.dark_mode);
                let light_pressed = (SLATE_200);
                let dark_pressed = (SLATE_600);
                let pressed_color = mix(light_pressed, dark_pressed, self.dark_mode);
                let color = mix(base, pressed_color, self.pressed);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0);
                sdf.fill(color);

                // Top border
                let border = mix((BORDER), (BORDER_DARK), self.dark_mode);
                sdf.box(0.0, 0.0, self.rect_size.x, 1.0, 0.0);
                sdf.fill(border);

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }

            fn get_color(self) -> vec4 {
                return mix((ACCENT_BLUE), (ACCENT_BLUE_DARK), self.dark_mode);
            }
        }

        text: "+ Add Custom Provider"
    }

    // Provider item - using instance variables for hover/selected states
    ProviderItem = <View> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        margin: 0
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                // Normal colors
                let light_normal = (WHITE);
                let dark_normal = #1f293b;
                let normal = mix(light_normal, dark_normal, self.dark_mode);

                // Hover colors
                let light_hover = #f1f5f9;
                let dark_hover = #334155;
                let hover_color = mix(light_hover, dark_hover, self.dark_mode);

                // Selected colors
                let light_selected = #DBEAFE;
                let dark_selected = #1E3A5F;
                let selected_color = mix(light_selected, dark_selected, self.dark_mode);

                // Mix states: normal -> hover -> selected
                let base = mix(normal, hover_color, self.hover);
                return mix(base, selected_color, self.selected);
            }
        }
        flow: Right
        align: {x: 0.0, y: 0.5}
    }

    // Provider label with dark mode support
    ProviderLabel = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                let light = #374151;
                let dark = #f1f5f9;
                return mix(light, dark, self.dark_mode);
            }
        }
    }


    // Providers panel - left side of settings
    pub ProvidersPanel = {{ProvidersPanel}} {
        width: 280, height: Fill
        flow: Down
        spacing: 0

        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        // Header
        header = <View> {
            width: Fill, height: Fit
            padding: {left: 16, right: 16, top: 16, bottom: 12}

            header_label = <Label> {
                text: "Providers"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((SLATE_800), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Provider list
        list_container = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 0

            openai_item = <ProviderItem> {
                <Icon> {
                    draw_icon: {
                        svg_file: (ICO_OPENAI)
                        fn get_color(self) -> vec4 { return #10A37F; }
                    }
                    icon_walk: {width: 24, height: 24, margin: {right: 10}}
                }
                openai_label = <ProviderLabel> {
                    text: "OpenAI"
                }
            }

            deepseek_item = <ProviderItem> {
                <Icon> {
                    draw_icon: {
                        svg_file: (ICO_DEEPSEEK)
                        fn get_color(self) -> vec4 { return #4D6BFE; }
                    }
                    icon_walk: {width: 20, height: 20, margin: {right: 10}}
                }
                deepseek_label = <ProviderLabel> {
                    text: "DeepSeek"
                }
            }

            alibaba_item = <ProviderItem> {
                <Icon> {
                    draw_icon: {
                        svg_file: (ICO_DEEPSEEK)
                        fn get_color(self) -> vec4 { return #6366f1; }
                    }
                    icon_walk: {width: 20, height: 20, margin: {right: 10}}
                }
                alibaba_label = <ProviderLabel> {
                    text: "Alibaba Cloud (Qwen)"
                }
            }
        }

        // Spacer
        <View> { width: Fill, height: Fill }

        // Add button at bottom
        add_button = <AddProviderButton> {}
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ProvidersPanelAction {
    None,
    Selected(ProviderId),
}

#[derive(Live, LiveHook, Widget)]
pub struct ProvidersPanel {
    #[deref]
    view: View,

    #[rust]
    selected_provider_id: Option<ProviderId>,

    #[rust]
    dark_mode: bool,
}

impl Widget for ProvidersPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let uid = self.widget_uid();

        // Provider items for hover and click handling
        let items = [
            ids!(list_container.openai_item),
            ids!(list_container.deepseek_item),
            ids!(list_container.alibaba_item),
        ];

        // Handle hover effects using FingerHover events
        for item_id in &items {
            let item = self.view.view(item_id.clone());
            match event.hits(cx, item.area()) {
                Hit::FingerHoverIn(_) => {
                    // Only apply hover if not currently selected
                    let is_selected = match self.selected_provider_id.as_ref().map(|id| id.as_str())
                    {
                        Some("openai") => item_id == &ids!(list_container.openai_item),
                        Some("deepseek") => item_id == &ids!(list_container.deepseek_item),
                        Some("alibaba_cloud") => item_id == &ids!(list_container.alibaba_item),
                        _ => false,
                    };
                    if !is_selected {
                        self.view.view(item_id.clone()).apply_over(
                            cx,
                            live! { draw_bg: { hover: 1.0 } },
                        );
                        self.view.redraw(cx);
                    }
                }
                Hit::FingerHoverOut(_) => {
                    // Only reset if not currently selected
                    let is_selected = match self.selected_provider_id.as_ref().map(|id| id.as_str())
                    {
                        Some("openai") => item_id == &ids!(list_container.openai_item),
                        Some("deepseek") => item_id == &ids!(list_container.deepseek_item),
                        Some("alibaba_cloud") => item_id == &ids!(list_container.alibaba_item),
                        _ => false,
                    };
                    if !is_selected {
                        self.view.view(item_id.clone()).apply_over(
                            cx,
                            live! { draw_bg: { hover: 0.0 } },
                        );
                        self.view.redraw(cx);
                    }
                }
                _ => {}
            }
        }

        // Extract actions - return early if not an Actions event
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle provider item clicks
        let mut new_selection: Option<ProviderId> = None;

        if self
            .view
            .view(ids!(list_container.openai_item))
            .finger_up(actions)
            .is_some()
        {
            new_selection = Some(ProviderId::from("openai"));
        }
        if self
            .view
            .view(ids!(list_container.deepseek_item))
            .finger_up(actions)
            .is_some()
        {
            new_selection = Some(ProviderId::from("deepseek"));
        }
        if self
            .view
            .view(ids!(list_container.alibaba_item))
            .finger_up(actions)
            .is_some()
        {
            new_selection = Some(ProviderId::from("alibaba_cloud"));
        }

        if let Some(id) = new_selection {
            // Only process if different from current selection
            if self.selected_provider_id.as_ref() != Some(&id) {
                let selected = id.as_str();
                // First reset all to normal (not selected, not hovered)
                for item_id in &items {
                    self.view.view(item_id.clone()).apply_over(
                        cx,
                        live! { draw_bg: { selected: 0.0, hover: 0.0 } },
                    );
                }
                // Then set selected state on the chosen item
                match selected {
                    "openai" => {
                        self.view.view(ids!(list_container.openai_item)).apply_over(
                            cx,
                            live! { draw_bg: { selected: 1.0 } },
                        );
                    }
                    "deepseek" => {
                        self.view.view(ids!(list_container.deepseek_item)).apply_over(
                            cx,
                            live! { draw_bg: { selected: 1.0 } },
                        );
                    }
                    "alibaba_cloud" => {
                        self.view.view(ids!(list_container.alibaba_item)).apply_over(
                            cx,
                            live! { draw_bg: { selected: 1.0 } },
                        );
                    }
                    _ => {}
                }
                self.selected_provider_id = Some(id.clone());
                self.view.redraw(cx);
                cx.widget_action(uid, &scope.path, ProvidersPanelAction::Selected(id));
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ProvidersPanelRef {
    /// Get the currently selected provider ID
    pub fn selected_provider_id(&self) -> Option<ProviderId> {
        self.borrow()
            .and_then(|inner| inner.selected_provider_id.clone())
    }

    /// Set the selected provider
    pub fn select_provider(&self, cx: &mut Cx, provider_id: &ProviderId) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.selected_provider_id = Some(provider_id.clone());
            inner.view.redraw(cx);
        }
    }

    /// Map item name to provider ID
    pub fn item_to_provider_id(name: &str) -> Option<ProviderId> {
        match name {
            "openai_item" => Some(ProviderId::from("openai")),
            "deepseek_item" => Some(ProviderId::from("deepseek")),
            "alibaba_item" => Some(ProviderId::from("alibaba_cloud")),
            _ => None,
        }
    }

    /// Map button name to provider ID (legacy alias)
    pub fn button_to_provider_id(name: &str) -> Option<ProviderId> {
        Self::item_to_provider_id(name)
    }

    /// Update dark mode for this widget
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Store dark mode state for hover/selection logic
            inner.dark_mode = dark_mode > 0.5;

            // Panel background
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );

            // Header label
            inner.view.label(ids!(header.header_label)).apply_over(
                cx,
                live! {
                    draw_text: { dark_mode: (dark_mode) }
                },
            );

            // Update provider items with dark_mode and selected instance variables
            let selected = inner.selected_provider_id.as_ref().map(|id| id.as_str());

            // OpenAI item
            let is_openai_selected = if selected == Some("openai") { 1.0 } else { 0.0 };
            inner.view.view(ids!(list_container.openai_item)).apply_over(
                cx,
                live! { draw_bg: { dark_mode: (dark_mode), selected: (is_openai_selected) } },
            );

            // DeepSeek item
            let is_deepseek_selected = if selected == Some("deepseek") { 1.0 } else { 0.0 };
            inner.view.view(ids!(list_container.deepseek_item)).apply_over(
                cx,
                live! { draw_bg: { dark_mode: (dark_mode), selected: (is_deepseek_selected) } },
            );

            // Alibaba item
            let is_alibaba_selected = if selected == Some("alibaba_cloud") { 1.0 } else { 0.0 };
            inner.view.view(ids!(list_container.alibaba_item)).apply_over(
                cx,
                live! { draw_bg: { dark_mode: (dark_mode), selected: (is_alibaba_selected) } },
            );

            // Provider labels - update dark_mode
            inner.view.label(ids!(list_container.openai_item.openai_label))
                .apply_over(cx, live! { draw_text: { dark_mode: (dark_mode) } });
            inner.view.label(ids!(list_container.deepseek_item.deepseek_label))
                .apply_over(cx, live! { draw_text: { dark_mode: (dark_mode) } });
            inner.view.label(ids!(list_container.alibaba_item.alibaba_label))
                .apply_over(cx, live! { draw_text: { dark_mode: (dark_mode) } });

            // Add button
            inner.view.button(ids!(add_button)).apply_over(
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
