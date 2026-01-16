//! MofaHero Widget - Session control strip for the AI dialog screen

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_component::*;
    use widgets::theme::*;

    // Icons
    ICO_START = dep("crate://self/resources/icons/start.svg")
    ICO_STOP = dep("crate://self/resources/icons/stop.svg")

    // Badge for connection state
    StatusBadge = <RoundedView> {
        width: Fit, height: Fit
        padding: { left: 12, right: 12, top: 6, bottom: 6 }
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((ACCENT_GREEN), (GREEN_400), self.dark_mode);
            }
            border_radius: 12.0
        }
        status_label = <Label> {
            text: "Ready"
            draw_text: {
                color: (WHITE)
                text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }
            }
        }
    }

    // Section title
    SectionTitle = <Label> {
        draw_text: {
            color: (TEXT_PRIMARY)
            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
        }
    }

    // Muted helper text
    HintText = <Label> {
        draw_text: {
            color: (TEXT_SECONDARY)
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
        }
    }

    pub MofaHero = {{MofaHero}} {
        width: Fill, height: 78
        flow: Right
        spacing: 10

        // Start / Stop card
        action_section = <RoundedView> {
            width: 260, height: Fill
            padding: { left: 14, right: 14, top: 10, bottom: 10 }
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 12.0
                fn get_color(self) -> vec4 {
                    return mix((PANEL_BG), (PANEL_BG_DARK), self.dark_mode);
                }
            }
            flow: Right
            spacing: 10
            align: {x: 0.0, y: 0.5}

            start_view = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 10
                align: {x: 0.0, y: 0.5}
                cursor: Hand

                start_icon = <View> {
                    width: 32, height: 32
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 14.0);
                            let light_color = vec4(0.133, 0.773, 0.373, 0.15);
                            let dark_color = vec4(0.2, 0.85, 0.45, 0.25);
                            sdf.fill(mix(light_color, dark_color, self.dark_mode));
                            return sdf.result;
                        }
                    }
                    <Icon> {
                        draw_icon: {
                            instance dark_mode: 0.0
                            svg_file: (ICO_START)
                            fn get_color(self) -> vec4 {
                                let light_color = vec4(0.133, 0.773, 0.373, 1.0);
                                let dark_color = vec4(0.3, 0.9, 0.5, 1.0);
                                return mix(light_color, dark_color, self.dark_mode);
                            }
                        }
                        icon_walk: {width: 20, height: 20}
                    }
                }

                start_text = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 2
                    start_label = <Label> {
                        text: "Start"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    start_hint = <HintText> { text: "启动对话 & 数据流" }
                }
            }

            stop_view = <View> {
                visible: false
                width: Fill, height: Fit
                flow: Right
                spacing: 10
                align: {x: 0.0, y: 0.5}
                cursor: Hand

                stop_icon = <View> {
                    width: 32, height: 32
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 14.0);
                            let light_color = vec4(0.937, 0.267, 0.267, 0.15);
                            let dark_color = vec4(1.0, 0.4, 0.4, 0.25);
                            sdf.fill(mix(light_color, dark_color, self.dark_mode));
                            return sdf.result;
                        }
                    }
                    <Icon> {
                        draw_icon: {
                            instance dark_mode: 0.0
                            svg_file: (ICO_STOP)
                            fn get_color(self) -> vec4 {
                                let light_color = vec4(0.937, 0.267, 0.267, 1.0);
                                let dark_color = vec4(1.0, 0.45, 0.45, 1.0);
                                return mix(light_color, dark_color, self.dark_mode);
                            }
                        }
                        icon_walk: {width: 20, height: 20}
                    }
                }

                stop_text = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 2
                    stop_label = <Label> {
                        text: "Stop"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    stop_hint = <HintText> { text: "暂停当前会话" }
                }
            }
        }

        status_section = <RoundedView> {
            width: 180, height: Fill
            padding: { left: 14, right: 14, top: 10, bottom: 10 }
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 12.0
                fn get_color(self) -> vec4 {
                    return mix((SLATE_50), (PANEL_BG_DARK), self.dark_mode);
                }
            }
            flow: Down
            spacing: 6

            status_title = <SectionTitle> { text: "Connection" }
            status_row = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8
                align: {x: 0.0, y: 0.5}

                status_dot = <RoundedView> {
                    width: 12, height: 12
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((ACCENT_GREEN), (GREEN_400), self.dark_mode);
                        }
                        border_radius: 6.0
                    }
                }
                status_badge = <StatusBadge> {}
            }
            status_hint = <HintText> { text: "等待启动" }
        }

        guidance_section = <RoundedView> {
            width: Fill, height: Fill
            padding: { left: 14, right: 14, top: 10, bottom: 10 }
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 12.0
                fn get_color(self) -> vec4 {
                    return mix((WHITE), (PANEL_BG_DARK), self.dark_mode);
                }
            }
            flow: Down
            spacing: 4

            guidance_title = <SectionTitle> { text: "小贴士" }
            guidance_body = <HintText> { text: "和 AI 教练对话时，随时让它帮你润色或改口。" }
        }
    }
}

/// Actions emitted by MofaHero
#[derive(Clone, Debug, DefaultNone)]
pub enum MofaHeroAction {
    None,
    StartClicked,
    StopClicked,
}

#[derive(Live, LiveHook, Widget)]
pub struct MofaHero {
    #[deref]
    view: View,

    #[rust]
    is_running: bool,

    #[rust]
    connection_status: ConnectionStatus,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ConnectionStatus {
    Ready,
    Connecting,
    #[default]
    Connected,
    Stopping,
    Stopped,
    Failed,
}

impl Widget for MofaHero {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Handle start/stop button clicks (using view containers to match conference-dashboard)
        let start_view = self.view.view(ids!(action_section.start_view));
        let stop_view = self.view.view(ids!(action_section.stop_view));

        match event.hits(cx, start_view.area()) {
            Hit::FingerUp(_) => {
                cx.widget_action(self.widget_uid(), &scope.path, MofaHeroAction::StartClicked);
            }
            _ => {}
        }

        match event.hits(cx, stop_view.area()) {
            Hit::FingerUp(_) => {
                cx.widget_action(self.widget_uid(), &scope.path, MofaHeroAction::StopClicked);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MofaHero {
    /// Set the running state (shows start or stop view - matches conference-dashboard)
    pub fn set_running(&mut self, cx: &mut Cx, running: bool) {
        self.is_running = running;
        self.view
            .view(ids!(action_section.start_view))
            .set_visible(cx, !running);
        self.view
            .view(ids!(action_section.stop_view))
            .set_visible(cx, running);
        self.view.redraw(cx);
    }

    /// Set connection status
    pub fn set_connection_status(&mut self, cx: &mut Cx, status: ConnectionStatus) {
        self.connection_status = status.clone();

        let (text, color, hint) = match status {
            ConnectionStatus::Ready => ("Ready", vec4(0.133, 0.773, 0.373, 1.0), "等待启动"),
            ConnectionStatus::Connecting => (
                "Connecting",
                vec4(0.933, 0.702, 0.067, 1.0),
                "正在连接数据流",
            ),
            ConnectionStatus::Connected => (
                "Connected",
                vec4(0.365, 0.384, 0.953, 1.0),
                "已联通，对话准备就绪",
            ),
            ConnectionStatus::Stopping => {
                ("Stopping", vec4(0.922, 0.533, 0.196, 1.0), "正在停止会话")
            }
            ConnectionStatus::Stopped => ("Stopped", vec4(0.514, 0.553, 0.620, 1.0), "已停止"),
            ConnectionStatus::Failed => {
                ("Failed", vec4(0.937, 0.267, 0.267, 1.0), "连接异常，请重试")
            }
        };

        self.view
            .label(ids!(status_section.status_badge.status_label))
            .set_text(cx, text);
        self.view
            .view(ids!(status_section.status_badge))
            .apply_over(
                cx,
                live! {
                    draw_bg: { color: (color) }
                },
            );
        self.view.view(ids!(status_section.status_dot)).apply_over(
            cx,
            live! {
                draw_bg: { color: (color) }
            },
        );
        self.view
            .label(ids!(status_section.status_hint))
            .set_text(cx, hint);

        self.view.redraw(cx);
    }

    /// Get the current running state
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

impl MofaHeroRef {
    pub fn set_running(&self, cx: &mut Cx, running: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_running(cx, running);
        }
    }

    pub fn set_connection_status(&self, cx: &mut Cx, status: ConnectionStatus) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_connection_status(cx, status);
        }
    }

    /// Update dark mode for this widget
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            // Action section
            inner.view.view(ids!(action_section)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            inner
                .view
                .label(ids!(action_section.start_view.start_text.start_label))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .label(ids!(action_section.stop_view.stop_text.stop_label))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );

            // Status section
            inner.view.view(ids!(status_section)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            let status_title_color = if dark_mode > 0.5 {
                vec4(0.929, 0.957, 0.988, 1.0) // TEXT_PRIMARY_DARK
            } else {
                vec4(0.043, 0.063, 0.102, 1.0) // TEXT_PRIMARY
            };
            inner
                .view
                .label(ids!(status_section.status_title))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { color: (status_title_color) }
                    },
                );
            let status_hint_color = if dark_mode > 0.5 {
                vec4(0.710, 0.757, 0.831, 1.0) // TEXT_SECONDARY_DARK
            } else {
                vec4(0.294, 0.333, 0.388, 1.0) // TEXT_SECONDARY
            };
            inner
                .view
                .label(ids!(status_section.status_hint))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { color: (status_hint_color) }
                    },
                );

            // Guidance section
            inner.view.view(ids!(guidance_section)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            inner
                .view
                .label(ids!(guidance_section.guidance_title))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { color: (status_title_color) }
                    },
                );
            inner
                .view
                .label(ids!(guidance_section.guidance_body))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { color: (status_hint_color) }
                    },
                );

            inner.view.redraw(cx);
        }
    }
}
