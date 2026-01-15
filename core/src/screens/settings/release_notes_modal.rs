//! Release Notes Modal - Dialog for displaying app version history

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use widgets::theme::*;

    // Modal close button
    ModalCloseButton = <Button> {
        width: 32, height: 32
        icon_walk: {width: 16, height: 16}
        draw_icon: {
            svg_file: dep("crate://self/resources/icons/close.svg")
            fn get_color(self) -> vec4 {
                return (GRAY_500);
            }
        }
        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let color = mix(
                    mix(vec4(0.0, 0.0, 0.0, 0.0), (GRAY_100), self.hover),
                    (GRAY_200),
                    self.pressed
                );
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                sdf.fill(color);
                return sdf.result;
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

    // Version entry header
    VersionHeader = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 12
        margin: {bottom: 8}

        version_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        date_label = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                }
            }
        }

        <View> { width: Fill, height: 1 }

        tag = <RoundedView> {
            width: Fit, height: Fit
            padding: {left: 8, right: 8, top: 2, bottom: 2}
            show_bg: true
            draw_bg: {
                color: (GREEN_100)
                border_radius: 4.0
            }
            tag_label = <Label> {
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                    color: (GREEN_700)
                }
            }
        }
    }

    // Release note item
    ReleaseNoteItem = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 8
        margin: {bottom: 6}

        bullet = <Label> {
            width: 16
            text: "•"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_BLUE), (ACCENT_BLUE_DARK), self.dark_mode);
                }
            }
        }

        note_text = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Version section
    VersionSection = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: {bottom: 20}
        margin: {bottom: 16}

        header = <VersionHeader> {}

        notes = <View> {
            width: Fill, height: Fit
            flow: Down
            padding: {left: 4}
        }

        divider = <View> {
            width: Fill, height: 1
            margin: {top: 16}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix((GRAY_200), (SLATE_700), self.dark_mode);
                }
            }
        }
    }

    // Release notes modal dialog
    pub ReleaseNotesModal = {{ReleaseNotesModal}} {
        width: Fill, height: Fill
        flow: Overlay
        visible: false

        // Overlay background
        overlay = <View> {
            width: Fill, height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return vec4(0.0, 0.0, 0.0, 0.5);
                }
            }
        }

        // Center the dialog
        dialog_container = <View> {
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}

            // Modal dialog
            dialog = <RoundedView> {
                width: 560, height: 520
                flow: Down
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    border_radius: 12.0
                    fn get_color(self) -> vec4 {
                        return mix((WHITE), (SLATE_800), self.dark_mode);
                    }
                }

                // Header
                header = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    padding: 20
                    align: {y: 0.5}

                    <Label> {
                        text: "Release Notes"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 16.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> { width: Fill, height: 1 }

                    close_button = <ModalCloseButton> {}
                }

                // Divider
                <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((GRAY_200), (SLATE_700), self.dark_mode);
                        }
                    }
                }

                // Content scroll area
                content_scroll = <ScrollYView> {
                    width: Fill, height: Fill
                    flow: Down
                    scroll_bars: <ScrollBars> {
                        show_scroll_x: false
                        show_scroll_y: true
                    }

                    content = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        padding: 20

                        // Version 1.0.0 - Current
                        v100 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            margin: {bottom: 24}

                            <View> {
                                width: Fill, height: Fit
                                flow: Right
                                align: {y: 0.5}
                                spacing: 12
                                margin: {bottom: 12}

                                <Label> {
                                    text: "Version 1.0.0"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_BOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <Label> {
                                    text: "January 15, 2026"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <View> { width: Fill, height: 1 }

                                <RoundedView> {
                                    width: Fit, height: Fit
                                    padding: {left: 8, right: 8, top: 2, bottom: 2}
                                    show_bg: true
                                    draw_bg: { color: (GREEN_100), border_radius: 4.0 }
                                    <Label> {
                                        text: "Current"
                                        draw_text: {
                                            text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                                            color: (GREEN_700)
                                        }
                                    }
                                }
                            }

                            // Features
                            <Label> {
                                text: "New Features"
                                margin: {bottom: 8}
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            <View> { width: Fill, height: Fit, flow: Down, padding: {left: 8} }

                            <ReleaseNoteItem> { note_text = { text: "AI-powered English conversation practice with real-time voice interaction" } }
                            <ReleaseNoteItem> { note_text = { text: "Multi-scenario learning: casual chat, interview prep, travel, business, and more" } }
                            <ReleaseNoteItem> { note_text = { text: "Real-time speech recognition with accurate pronunciation feedback" } }
                            <ReleaseNoteItem> { note_text = { text: "Multiple AI provider support: OpenAI, DeepSeek, Alibaba Cloud" } }
                            <ReleaseNoteItem> { note_text = { text: "Beautiful native UI with dark mode support" } }
                            <ReleaseNoteItem> { note_text = { text: "Debug console for troubleshooting and monitoring" } }
                        }

                        // Divider
                        <View> {
                            width: Fill, height: 1
                            margin: {bottom: 24}
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    return mix((GRAY_200), (SLATE_700), self.dark_mode);
                                }
                            }
                        }

                        // Version 0.9.0 - Beta
                        v090 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            margin: {bottom: 24}

                            <View> {
                                width: Fill, height: Fit
                                flow: Right
                                align: {y: 0.5}
                                spacing: 12
                                margin: {bottom: 12}

                                <Label> {
                                    text: "Version 0.9.0 Beta"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_BOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <Label> {
                                    text: "December 20, 2025"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <View> { width: Fill, height: 1 }

                                <RoundedView> {
                                    width: Fit, height: Fit
                                    padding: {left: 8, right: 8, top: 2, bottom: 2}
                                    show_bg: true
                                    draw_bg: { color: (BLUE_100), border_radius: 4.0 }
                                    <Label> {
                                        text: "Beta"
                                        draw_text: {
                                            text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                                            color: (BLUE_700)
                                        }
                                    }
                                }
                            }

                            <Label> {
                                text: "Improvements"
                                margin: {bottom: 8}
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            <ReleaseNoteItem> { note_text = { text: "Improved audio processing with lower latency" } }
                            <ReleaseNoteItem> { note_text = { text: "Enhanced TTS quality with multiple voice options" } }
                            <ReleaseNoteItem> { note_text = { text: "Settings panel with provider configuration" } }
                            <ReleaseNoteItem> { note_text = { text: "Fixed memory leaks in long conversation sessions" } }
                        }

                        // Divider
                        <View> {
                            width: Fill, height: 1
                            margin: {bottom: 24}
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    return mix((GRAY_200), (SLATE_700), self.dark_mode);
                                }
                            }
                        }

                        // Version 0.8.0 - Alpha
                        v080 = <View> {
                            width: Fill, height: Fit
                            flow: Down

                            <View> {
                                width: Fill, height: Fit
                                flow: Right
                                align: {y: 0.5}
                                spacing: 12
                                margin: {bottom: 12}

                                <Label> {
                                    text: "Version 0.8.0 Alpha"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_BOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <Label> {
                                    text: "November 1, 2025"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_MUTED), (TEXT_MUTED_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <View> { width: Fill, height: 1 }

                                <RoundedView> {
                                    width: Fit, height: Fit
                                    padding: {left: 8, right: 8, top: 2, bottom: 2}
                                    show_bg: true
                                    draw_bg: { color: #fff7ed, border_radius: 4.0 }
                                    <Label> {
                                        text: "Alpha"
                                        draw_text: {
                                            text_style: <FONT_MEDIUM>{ font_size: 10.0 }
                                            color: #c2410c
                                        }
                                    }
                                }
                            }

                            <Label> {
                                text: "Initial Release"
                                margin: {bottom: 8}
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            <ReleaseNoteItem> { note_text = { text: "Initial alpha release with core conversation features" } }
                            <ReleaseNoteItem> { note_text = { text: "Basic speech-to-text integration" } }
                            <ReleaseNoteItem> { note_text = { text: "OpenAI GPT integration for responses" } }
                            <ReleaseNoteItem> { note_text = { text: "Proof-of-concept native desktop UI" } }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ReleaseNotesModal {
    #[deref]
    view: View,

    #[rust]
    dark_mode: f64,
}

impl Widget for ReleaseNotesModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle close button click
        if self
            .view
            .button(ids!(dialog_container.dialog.header.close_button))
            .clicked(actions)
        {
            self.hide(cx);
        }

        // Handle click on overlay to close
        if self.view.view(ids!(overlay)).finger_up(actions).is_some() {
            self.hide(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReleaseNotesModal {
    pub fn show(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, true);
        self.view.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.view.set_visible(cx, false);
        self.view.redraw(cx);
    }

    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.dark_mode = dark_mode;

        // Update dialog background
        self.view
            .view(ids!(dialog_container.dialog))
            .apply_over(cx, live! { draw_bg: { dark_mode: (dark_mode) } });

        self.view.redraw(cx);
    }
}

impl ReleaseNotesModalRef {
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
            inner.update_dark_mode(cx, dark_mode);
        }
    }
}
