//! # Debug Window Widget
//!
//! A separate window for viewing system logs and debug information.
//! This window can be opened from anywhere in the app via a debug button.
//!
//! ## Features
//!
//! - **Log Filtering**: Filter by log level (DEBUG, INFO, WARN, ERROR)
//! - **Node Filtering**: Filter by source node (ASR, TTS, LLM, etc.)
//! - **Search**: Full-text search in log content
//! - **Copy**: Copy filtered logs to clipboard
//!
//! ## Usage
//!
//! ```rust,ignore
//! live_design! {
//!     use widgets::debug_window::DebugWindow;
//!
//!     App = {{App}} {
//!         debug_window = <DebugWindow> {}
//!     }
//! }
//! ```
//!
//! ## Opening/Closing
//!
//! ```rust,ignore
//! // Show the debug window
//! self.debug_window.show(cx);
//!
//! // Hide the debug window
//! self.debug_window.hide(cx);
//! ```

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    pub DebugWindow = {{DebugWindow}} {
        // This is a separate Window, not an overlay
        debug_window = <Window> {
            window: {
                title: "Debug Console"
                position: vec2(200.0, 100.0)
                inner_size: vec2(900.0, 680.0)
            }
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix((WHITE), (SLATE_800), self.dark_mode);
                }
            }

            body = <View> {
                width: Fill, height: Fill
                flow: Down
                padding: 0

                // Title bar (window header)
                title_bar = <View> {
                    width: Fill, height: 44
                    flow: Right
                    align: {y: 0.5}
                    padding: {left: 16, right: 8, top: 0, bottom: 0}
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let light = (SLATE_100);
                            let dark = (SLATE_700);
                            return mix(light, dark, self.dark_mode);
                        }
                    }

                    window_icon = <Label> {
                        text: "🔧"
                        draw_text: { text_style: <FONT_REGULAR>{ font_size: 14.0 } color: (TEXT_PRIMARY) }
                    }

                    title = <Label> {
                        margin: {left: 8}
                        text: "Debug Console"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> { width: Fill, height: 1 }
                }

                // Content area
                content = <View> {
                    width: Fill, height: Fill
                    flow: Down
                    padding: 16
                    spacing: 12

                    // Filter row
                    filter_row = <View> {
                        width: Fill, height: 36
                        flow: Right
                        align: {y: 0.5}
                        spacing: 8

                        level_filter = <DropDown> {
                            width: 90, height: 28
                            labels: ["ALL", "DEBUG", "INFO", "WARN", "ERROR"]
                            values: [ALL, DEBUG, INFO, WARN, ERROR]
                        }

                        node_filter = <DropDown> {
                            width: 130, height: 28
                            labels: ["All Nodes", "ASR", "TTS", "LLM", "Bridge", "Monitor", "App"]
                            values: [ALL, ASR, TTS, LLM, BRIDGE, MONITOR, APP]
                        }

                        search_input = <TextInput> {
                            width: Fill, height: 28
                            padding: {left: 10, right: 10}
                            empty_text: "Search logs..."
                            draw_bg: {
                                instance dark_mode: 0.0
                                border_radius: 6.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.border_radius);
                                    sdf.fill(bg);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        copy_btn = <Button> {
                            width: 36, height: 28
                            text: "📋"
                            draw_bg: {
                                instance hover: 0.0
                                instance copied: 0.0
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let light = mix((INDIGO_100), (INDIGO_200), self.hover);
                                    let dark = mix((SLATE_700), (SLATE_600), self.hover);
                                    let color = mix(light, dark, self.dark_mode);
                                    let color = mix(color, #22c55e, self.copied);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                    sdf.fill(color);
                                    return sdf.result;
                                }
                            }
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                color: (TEXT_PRIMARY)
                            }
                        }

                        clear_btn = <Button> {
                            width: Fit, height: 28
                            padding: {left: 10, right: 10}
                            text: "Clear"
                            draw_bg: {
                                instance hover: 0.0
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let light = mix((SLATE_200), (SLATE_300), self.hover);
                                    let dark = mix((SLATE_700), (SLATE_600), self.hover);
                                    let color = mix(light, dark, self.dark_mode);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                                    sdf.fill(color);
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
                        }
                    }

                    // Log content area
                    log_container = <RoundedView> {
                        width: Fill, height: Fill
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: 8.0
                            fn get_color(self) -> vec4 {
                                return mix((SLATE_50), (SLATE_900), self.dark_mode);
                            }
                        }
                        flow: Down

                        log_scroll = <ScrollYView> {
                            width: Fill, height: Fill
                            flow: Down
                            scroll_bars: <ScrollBars> {
                                show_scroll_x: false
                                show_scroll_y: true
                            }

                            log_content_wrapper = <View> {
                                width: Fill, height: Fit
                                padding: 12
                                flow: Down

                                log_content = <Markdown> {
                                    width: Fill, height: Fit
                                    font_size: 11.0
                                    font_color: (GRAY_600)
                                    paragraph_spacing: 4

                                    draw_normal: { text_style: <FONT_REGULAR>{ font_size: 11.0 } }
                                    draw_bold: { text_style: <FONT_SEMIBOLD>{ font_size: 11.0 } }
                                    draw_fixed: { text_style: <FONT_REGULAR>{ font_size: 10.0 } }
                                }
                            }
                        }
                    }

                    // Footer with stats
                    footer = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        spacing: 16

                        log_count = <Label> {
                            text: "0 entries"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        <View> { width: Fill, height: 1 }

                        auto_scroll_label = <Label> {
                            text: "Auto-scroll: On"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DebugWindow {
    #[deref]
    #[live]
    window: Window,

    #[rust]
    log_entries: Vec<String>,

    #[rust]
    log_level_filter: usize,

    #[rust]
    log_node_filter: usize,

    #[rust]
    auto_scroll: bool,

    #[rust]
    dark_mode: f64,

    #[rust]
    copy_feedback_timer: Timer,
}

impl Widget for DebugWindow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.window.handle_event(cx, event, scope);

        // Handle copy feedback timer
        if self.copy_feedback_timer.is_event(event).is_some() {
            self.window
                .button(ids!(debug_window.body.content.filter_row.copy_btn))
                .apply_over(cx, live! { draw_bg: { copied: 0.0 } });
            self.window.redraw(cx);
        }

        // Handle actions
        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Level filter
        if let Some(selected) = self
            .window
            .drop_down(ids!(debug_window.body.content.filter_row.level_filter))
            .selected(actions)
        {
            self.log_level_filter = selected;
            self.update_display(cx);
        }

        // Node filter
        if let Some(selected) = self
            .window
            .drop_down(ids!(debug_window.body.content.filter_row.node_filter))
            .selected(actions)
        {
            self.log_node_filter = selected;
            self.update_display(cx);
        }

        // Search text changed
        if self
            .window
            .text_input(ids!(debug_window.body.content.filter_row.search_input))
            .changed(actions)
            .is_some()
        {
            self.update_display(cx);
        }

        // Copy button
        if self.window.button(ids!(debug_window.body.content.filter_row.copy_btn)).clicked(actions) {
            self.copy_to_clipboard(cx);
            self.window
                .button(ids!(debug_window.body.content.filter_row.copy_btn))
                .apply_over(cx, live! { draw_bg: { copied: 1.0 } });
            self.window.redraw(cx);
            self.copy_feedback_timer = cx.start_timeout(1.0);
        }

        // Clear button
        if self.window.button(ids!(debug_window.body.content.filter_row.clear_btn)).clicked(actions) {
            self.log_entries.clear();
            self.update_display(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.window.draw_walk(cx, scope, walk)
    }
}

impl DebugWindow {
    /// Show the debug window
    pub fn show(&mut self, cx: &mut Cx) {
        self.window.set_visible(cx, true);
        self.window.redraw(cx);
    }

    /// Hide the debug window
    pub fn hide(&mut self, cx: &mut Cx) {
        self.window.set_visible(cx, false);
        self.window.redraw(cx);
    }

    /// Add a log entry
    pub fn add_log(&mut self, entry: String) {
        self.log_entries.push(entry);
        // Keep only last 1000 entries
        if self.log_entries.len() > 1000 {
            self.log_entries.remove(0);
        }
    }

    /// Update the log display with current filters
    fn update_display(&mut self, cx: &mut Cx) {
        let search_text = self
            .window
            .text_input(ids!(debug_window.body.content.filter_row.search_input))
            .text()
            .to_lowercase();

        let filtered: Vec<&String> = self
            .log_entries
            .iter()
            .filter(|entry| {
                // Level filter
                let level_ok = match self.log_level_filter {
                    0 => true, // ALL
                    1 => entry.contains("[DEBUG]"),
                    2 => entry.contains("[INFO]"),
                    3 => entry.contains("[WARN]"),
                    4 => entry.contains("[ERROR]"),
                    _ => true,
                };

                // Node filter
                let node_ok = match self.log_node_filter {
                    0 => true, // ALL
                    1 => entry.contains("[ASR]") || entry.contains("asr"),
                    2 => entry.contains("[TTS]") || entry.contains("tts"),
                    3 => entry.contains("[LLM]") || entry.contains("llm"),
                    4 => entry.contains("[Bridge]") || entry.contains("bridge"),
                    5 => entry.contains("[Monitor]") || entry.contains("monitor"),
                    6 => entry.contains("[App]") || entry.contains("app"),
                    _ => true,
                };

                // Search filter
                let search_ok = search_text.is_empty()
                    || entry.to_lowercase().contains(&search_text);

                level_ok && node_ok && search_ok
            })
            .collect();

        // Update log content
        let content = if filtered.is_empty() {
            "*No log entries matching filters*".to_string()
        } else {
            filtered
                .iter()
                .map(|s| format!("```\n{}\n```", s))
                .collect::<Vec<_>>()
                .join("\n")
        };

        self.window
            .markdown(ids!(debug_window.body.content.log_container.log_scroll.log_content_wrapper.log_content))
            .set_text(cx, &content);

        // Update count label
        let count_text = format!("{} entries", filtered.len());
        self.window
            .label(ids!(debug_window.body.content.footer.log_count))
            .set_text(cx, &count_text);

        self.window.redraw(cx);
    }

    /// Copy filtered logs to clipboard
    fn copy_to_clipboard(&self, cx: &mut Cx) {
        let search_text = self
            .window
            .text_input(ids!(debug_window.body.content.filter_row.search_input))
            .text()
            .to_lowercase();

        let filtered: Vec<&String> = self
            .log_entries
            .iter()
            .filter(|entry| {
                let level_ok = match self.log_level_filter {
                    0 => true,
                    1 => entry.contains("[DEBUG]"),
                    2 => entry.contains("[INFO]"),
                    3 => entry.contains("[WARN]"),
                    4 => entry.contains("[ERROR]"),
                    _ => true,
                };

                let node_ok = match self.log_node_filter {
                    0 => true,
                    1 => entry.contains("[ASR]"),
                    2 => entry.contains("[TTS]"),
                    3 => entry.contains("[LLM]"),
                    4 => entry.contains("[Bridge]"),
                    5 => entry.contains("[Monitor]"),
                    6 => entry.contains("[App]"),
                    _ => true,
                };

                let search_ok =
                    search_text.is_empty() || entry.to_lowercase().contains(&search_text);

                level_ok && node_ok && search_ok
            })
            .collect();

        let text: Vec<String> = filtered.into_iter().cloned().collect();
        cx.copy_to_clipboard(&text.join("\n"));
    }

    /// Update dark mode for the debug window
    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.dark_mode = dark_mode;

        // Update window background
        self.window.apply_over(
            cx,
            live! { debug_window: { draw_bg: { dark_mode: (dark_mode) } } },
        );

        // Update title bar
        self.window.view(ids!(debug_window.body.title_bar)).apply_over(
            cx,
            live! { draw_bg: { dark_mode: (dark_mode) } },
        );

        self.window.label(ids!(debug_window.body.title_bar.title)).apply_over(
            cx,
            live! { draw_text: { dark_mode: (dark_mode) } },
        );

        // Update log container
        self.window.view(ids!(debug_window.body.content.log_container)).apply_over(
            cx,
            live! { draw_bg: { dark_mode: (dark_mode) } },
        );

        self.window.redraw(cx);
    }
}

impl DebugWindowRef {
    /// Show the debug window
    pub fn show(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    /// Hide the debug window
    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    /// Add a log entry
    pub fn add_log(&self, entry: String) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_log(entry);
        }
    }

    /// Add multiple log entries and update display
    pub fn add_logs(&self, cx: &mut Cx, entries: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            for entry in entries {
                inner.add_log(entry);
            }
            inner.update_display(cx);
        }
    }

    /// Update dark mode
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_dark_mode(cx, dark_mode);
        }
    }

    /// Check if window is open
    pub fn is_open(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.window.visible()
        } else {
            false
        }
    }
}
