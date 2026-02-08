//! Context selection dialog component
//!
//! Modal dialog for selecting conversation context (characters, scenarios)
//! Displays a grid of available contexts with icons, names, and descriptions
//! Loads context data from the server asynchronously.

use crossbeam_channel::{bounded, Receiver, Sender};
use makepad_widgets::*;

use crate::asset_api::{get_asset_api, ChatContext};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Context item card for PortalList
    pub ContextCard = {{ContextCard}} {
        width: 170, height: Fit
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance dark_mode: 0.0
            instance border_radius: 12.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);

                let light_normal = vec4(1.0, 1.0, 1.0, 1.0);
                let light_hover = vec4(1.0, 0.969, 0.929, 1.0);  // orange-50
                let dark_normal = vec4(0.118, 0.141, 0.227, 1.0);
                let dark_hover = vec4(0.153, 0.180, 0.275, 1.0);

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover = mix(light_hover, dark_hover, self.dark_mode);
                sdf.fill(mix(normal, hover, self.hover));

                // Border
                let border_normal = mix(vec4(0.0, 0.0, 0.0, 0.08), vec4(1.0, 1.0, 1.0, 0.1), self.dark_mode);
                let border_hover = vec4(0.976, 0.451, 0.086, 0.5);  // orange with opacity
                sdf.stroke(mix(border_normal, border_hover, self.hover), 1.0);

                return sdf.result;
            }
        }
        padding: 16
        flow: Down
        spacing: 8
        align: {x: 0.5}
        cursor: Hand

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 0.0} }
                }
                on = {
                    from: {all: Forward {duration: 0.15}}
                    apply: { draw_bg: {hover: 1.0} }
                }
            }
        }

        icon_label = <Label> {
            width: Fit, height: Fit
            text: "💬"
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 28.0 }
            }
        }

        name_zh = <Label> {
            width: Fit, height: Fit
            text: "场景名称"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        name_en = <Label> {
            width: Fit, height: Fit
            text: "Context Name"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                }
            }
        }

        description = <Label> {
            width: Fill, height: Fit
            text: ""
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                }
                wrap: Word
            }
        }
    }

    // Main context dialog
    pub ContextDialog = {{ContextDialog}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        // Backdrop scrim
        scrim = <View> {
            width: Fill, height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.0, 0.0, 0.0, 0.5);
                }
            }
        }

        // Dialog container
        dialog_container = <View> {
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}

            dialog = <RoundedView> {
                width: 620, height: Fit
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    instance border_radius: 16.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                        let light = vec4(1.0, 1.0, 1.0, 1.0);
                        let dark = vec4(0.067, 0.090, 0.133, 1.0);
                        sdf.fill(mix(light, dark, self.dark_mode));
                        return sdf.result;
                    }
                }
                flow: Down

                // Header
                header = <View> {
                    width: Fill, height: Fit
                    padding: {left: 20, right: 20, top: 16, bottom: 16}
                    flow: Right
                    align: {y: 0.5}

                    title = <Label> {
                        width: Fill, height: Fit
                        text: "选择对话场景"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    close_btn = <Button> {
                        width: 32, height: 32
                        text: "✕"
                        draw_bg: {
                            instance hover: 0.0
                            instance border_radius: 8.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.0, 0.0, 0.0, 0.1), self.hover));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_400), self.dark_mode);
                            }
                        }
                        animator: {
                            hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                            down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        }
                    }
                }

                // Divider
                <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((SLATE_200), (SLATE_600), self.dark_mode);
                        }
                    }
                }

                // Content area with scrollable grid
                content = <ScrollYView> {
                    width: Fill, height: 420
                    padding: 20
                    flow: Down
                    spacing: 16
                    scroll_bars: <ScrollBars> {
                        show_scroll_x: false
                        show_scroll_y: true
                    }

                    // Loading indicator
                    loading_view = <View> {
                        width: Fill, height: 300
                        align: {x: 0.5, y: 0.5}
                        visible: true

                        <Label> {
                            text: "加载中..."
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_400), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Empty state
                    empty_view = <View> {
                        width: Fill, height: 300
                        align: {x: 0.5, y: 0.5}
                        visible: false

                        <Label> {
                            text: "暂无可用场景"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_400), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Context grid using PortalList for dynamic content
                    context_grid = <PortalList> {
                        width: Fill, height: Fit
                        flow: RightWrap
                        spacing: 12

                        context_card = <ContextCard> {}
                    }
                }
            }
        }
    }
}

/// Actions emitted by the context dialog
#[derive(Clone, Debug, DefaultNone)]
pub enum ContextDialogAction {
    Close,
    /// Selected a context with the given context data
    SelectContext { context: ChatContext },
    None,
}

/// Context card widget for individual context items
#[derive(Live, LiveHook, Widget)]
pub struct ContextCard {
    #[deref]
    view: View,
    #[animator]
    animator: Animator,
    #[rust]
    context_id: i64,
}

impl Widget for ContextCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ContextCardRef {
    pub fn set_context(&self, cx: &mut Cx, context: &ChatContext) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.context_id = context.id;
            inner
                .view
                .label(ids!(icon_label))
                .set_text(cx, context.icon_emoji.as_deref().unwrap_or("💬"));
            inner
                .view
                .label(ids!(name_zh))
                .set_text(cx, &context.name_zh);
            inner
                .view
                .label(ids!(name_en))
                .set_text(cx, &context.name_en);
            inner.view.label(ids!(description)).set_text(
                cx,
                context.description_zh.as_deref().unwrap_or(""),
            );
            inner.view.redraw(cx);
        }
    }

    pub fn context_id(&self) -> Option<i64> {
        self.borrow().map(|inner| inner.context_id)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ContextDialog {
    #[deref]
    view: View,
    #[rust]
    is_loading: bool,
    #[rust]
    contexts: Vec<ChatContext>,
    #[rust]
    contexts_loaded: bool,
    #[rust]
    result_receiver: Option<Receiver<Result<Vec<ChatContext>, String>>>,
    #[rust]
    poll_timer: Timer,
}

impl Widget for ContextDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Poll for async results
        if self.poll_timer.is_event(event).is_some() {
            self.poll_context_results(cx);
        }

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => &[],
        };

        // Handle close button
        if self
            .view
            .button(ids!(dialog_container.dialog.header.close_btn))
            .clicked(actions)
        {
            self.hide(cx);
            cx.widget_action(
                self.view.widget_uid(),
                &scope.path,
                ContextDialogAction::Close,
            );
        }

        // Handle scrim click to close
        if self.view.view(ids!(scrim)).finger_up(actions).is_some() {
            self.hide(cx);
            cx.widget_action(
                self.view.widget_uid(),
                &scope.path,
                ContextDialogAction::Close,
            );
        }

        // Handle context card clicks from PortalList
        let context_grid = self
            .view
            .portal_list(ids!(dialog_container.dialog.content.context_grid));

        for (item_id, item) in context_grid.items_with_actions(actions) {
            // Check for finger up on the item view
            if item.as_view().finger_up(actions).is_some() {
                // Clone the context before mutating self
                if let Some(context) = self.contexts.get(item_id).cloned() {
                    ::log::info!("Selected context: {} ({})", context.name_zh, context.id);
                    self.hide(cx);
                    cx.widget_action(
                        self.view.widget_uid(),
                        &scope.path,
                        ContextDialogAction::SelectContext { context },
                    );
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update the PortalList before drawing
        let context_count = self.contexts.len();

        while let Some(next) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = next.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, context_count);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < context_count {
                        let item = list.item(cx, item_id, live_id!(context_card));
                        let context = &self.contexts[item_id];

                        // Set context data on the card
                        item.label(ids!(icon_label))
                            .set_text(cx, context.icon_emoji.as_deref().unwrap_or("💬"));
                        item.label(ids!(name_zh)).set_text(cx, &context.name_zh);
                        item.label(ids!(name_en)).set_text(cx, &context.name_en);
                        item.label(ids!(description))
                            .set_text(cx, context.description_zh.as_deref().unwrap_or(""));

                        item.draw_all(cx, scope);
                    }
                }
            }
        }

        DrawStep::done()
    }
}

impl ContextDialog {
    pub fn show(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, true);

        // Load contexts if not already loaded
        if !self.contexts_loaded && !self.is_loading {
            self.load_contexts(cx);
        }

        self.view.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, false);
        cx.stop_timer(self.poll_timer);
        self.view.redraw(cx);
    }

    fn load_contexts(&mut self, cx: &mut Cx) {
        self.is_loading = true;

        // Update UI to show loading state
        self.view
            .view(ids!(dialog_container.dialog.content.loading_view))
            .set_visible(cx, true);
        self.view
            .view(ids!(dialog_container.dialog.content.empty_view))
            .set_visible(cx, false);
        self.view
            .portal_list(ids!(dialog_container.dialog.content.context_grid))
            .set_visible(cx, false);
        self.view.redraw(cx);

        // Create channel for async result
        let (sender, receiver): (
            Sender<Result<Vec<ChatContext>, String>>,
            Receiver<Result<Vec<ChatContext>, String>>,
        ) = bounded(1);
        self.result_receiver = Some(receiver);

        // Spawn thread to fetch contexts
        if let Some(api_lock) = get_asset_api() {
            let api = api_lock.read().unwrap().clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    let result = api.list_contexts().await;
                    let _ = sender.send(result);
                });
            });

            // Start polling timer
            self.poll_timer = cx.start_interval(0.1); // Poll every 100ms
        } else {
            ::log::error!("Asset API not initialized");
            self.is_loading = false;
            self.view
                .view(ids!(dialog_container.dialog.content.loading_view))
                .set_visible(cx, false);
            self.view
                .view(ids!(dialog_container.dialog.content.empty_view))
                .set_visible(cx, true);
            self.view.redraw(cx);
        }
    }

    fn poll_context_results(&mut self, cx: &mut Cx) {
        if let Some(receiver) = &self.result_receiver {
            if let Ok(result) = receiver.try_recv() {
                // Stop polling
                cx.stop_timer(self.poll_timer);
                self.result_receiver = None;
                self.is_loading = false;

                match result {
                    Ok(contexts) => {
                        ::log::info!("Loaded {} contexts from server", contexts.len());
                        self.contexts = contexts;
                        self.contexts_loaded = true;

                        // Update UI
                        self.view
                            .view(ids!(dialog_container.dialog.content.loading_view))
                            .set_visible(cx, false);

                        if self.contexts.is_empty() {
                            self.view
                                .view(ids!(dialog_container.dialog.content.empty_view))
                                .set_visible(cx, true);
                            self.view
                                .portal_list(ids!(dialog_container.dialog.content.context_grid))
                                .set_visible(cx, false);
                        } else {
                            self.view
                                .view(ids!(dialog_container.dialog.content.empty_view))
                                .set_visible(cx, false);
                            self.view
                                .portal_list(ids!(dialog_container.dialog.content.context_grid))
                                .set_visible(cx, true);
                        }
                    }
                    Err(e) => {
                        ::log::error!("Failed to load contexts: {}", e);
                        self.view
                            .view(ids!(dialog_container.dialog.content.loading_view))
                            .set_visible(cx, false);
                        self.view
                            .view(ids!(dialog_container.dialog.content.empty_view))
                            .set_visible(cx, true);
                    }
                }

                self.view.redraw(cx);
            }
        }
    }
}

impl ContextDialogRef {
    pub fn show(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .view
                .view(ids!(dialog_container.dialog))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .label(ids!(dialog_container.dialog.header.title))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .button(ids!(dialog_container.dialog.header.close_btn))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner.view.redraw(cx);
        }
    }
}
