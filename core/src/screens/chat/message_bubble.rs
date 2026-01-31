//! Message bubble components for chat display
//!
//! Provides distinct visual styles for user and AI messages:
//! - UserBubble: Orange background (#f97316), right-aligned
//! - AiBubble: Gray background, left-aligned with avatar

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Orange accent color for user bubbles
    BUBBLE_ORANGE = #f97316
    BUBBLE_ORANGE_LIGHT = #fed7aa

    // Gray for AI bubbles
    BUBBLE_GRAY = #f3f4f6
    BUBBLE_GRAY_DARK = #374151

    // AI Avatar
    AiAvatar = <RoundedView> {
        width: 36, height: 36
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 18.0
            fn pixel(self) -> vec4 {
                let light = vec4(0.976, 0.980, 0.984, 1.0);  // gray-50
                let dark = vec4(0.216, 0.255, 0.318, 1.0);   // slate-700
                return mix(light, dark, self.dark_mode);
            }
        }
        align: {x: 0.5, y: 0.5}

        <Label> {
            text: "🤖"
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 16.0 }
            }
        }
    }

    // User message bubble - orange, right-aligned
    pub UserBubble = {{UserBubble}} {
        width: Fill, height: Fit
        flow: Right
        align: {x: 1.0}  // Right align
        padding: {left: 60}  // Leave space on left

        bubble_container = <RoundedView> {
            width: Fit, height: Fit
            show_bg: true
            draw_bg: {
                color: (BUBBLE_ORANGE)
                border_radius: 16.0
            }
            padding: {left: 16, right: 16, top: 12, bottom: 12}
            flow: Down
            spacing: 8

            content_en = <Label> {
                width: Fit, height: Fit
                text: ""
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    color: (WHITE)
                    wrap: Word
                }
            }

            divider = <View> {
                width: Fill, height: 1
                show_bg: true
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return vec4(1.0, 1.0, 1.0, 0.2);
                    }
                }
            }

            content_zh = <Label> {
                width: Fit, height: Fit
                text: ""
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    color: #ffedd5  // orange-100
                    wrap: Word
                }
            }

            timestamp_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {x: 1.0, y: 0.5}
                spacing: 8

                timestamp = <Label> {
                    width: Fit, height: Fit
                    text: ""
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        color: #fed7aa  // orange-200
                    }
                }

                audio_btn = <Button> {
                    width: 24, height: 24
                    visible: false
                    text: "🔊"
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return vec4(0.0, 0.0, 0.0, 0.0);
                        }
                    }
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        color: #fed7aa
                    }
                    animator: {
                        hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                    }
                }
            }
        }
    }

    // AI message bubble - gray, left-aligned with avatar
    pub AiBubble = {{AiBubble}} {
        width: Fill, height: Fit
        flow: Right
        align: {x: 0.0}  // Left align
        spacing: 10
        padding: {right: 60}  // Leave space on right

        avatar = <AiAvatar> {}

        bubble_container = <RoundedView> {
            width: Fit, height: Fit
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 16.0
                fn pixel(self) -> vec4 {
                    let light = vec4(0.953, 0.957, 0.961, 1.0);  // gray-100
                    let dark = vec4(0.216, 0.255, 0.318, 1.0);   // slate-700
                    return mix(light, dark, self.dark_mode);
                }
            }
            padding: {left: 16, right: 16, top: 12, bottom: 12}
            flow: Down
            spacing: 8

            content_en = <Label> {
                width: Fit, height: Fit
                text: ""
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                    wrap: Word
                }
            }

            divider = <View> {
                width: Fill, height: 1
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        return mix(vec4(0.0, 0.0, 0.0, 0.08), vec4(1.0, 1.0, 1.0, 0.1), self.dark_mode);
                    }
                }
            }

            content_zh = <Label> {
                width: Fit, height: Fit
                text: ""
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (SLATE_400), self.dark_mode);
                    }
                    wrap: Word
                }
            }

            timestamp_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {x: 0.0, y: 0.5}
                spacing: 8

                timestamp = <Label> {
                    width: Fit, height: Fit
                    text: ""
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }

                audio_btn = <Button> {
                    width: 24, height: 24
                    text: "🔊"
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return vec4(0.0, 0.0, 0.0, 0.0);
                        }
                    }
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                    animator: {
                        hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                    }
                }

                favorite_btn = <Button> {
                    width: 24, height: 24
                    text: "♡"
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return vec4(0.0, 0.0, 0.0, 0.0);
                        }
                    }
                    draw_text: {
                        instance dark_mode: 0.0
                        instance is_favorite: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            let normal = mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            let favorite = vec4(0.937, 0.267, 0.267, 1.0);  // red-500
                            return mix(normal, favorite, self.is_favorite);
                        }
                    }
                    animator: {
                        hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                        down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                    }
                }
            }
        }
    }

    // Message list container for chat
    pub MessageList = {{MessageList}} {
        width: Fill, height: Fit
        flow: Down
        spacing: 16
        padding: 16
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct UserBubble {
    #[deref]
    view: View,
}

impl Widget for UserBubble {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl UserBubbleRef {
    #[allow(unused_mut)]
    pub fn set_content(&self, cx: &mut Cx, content_en: &str, content_zh: &str, timestamp: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .view
                .label(ids!(bubble_container.content_en))
                .set_text(cx, content_en);
            inner
                .view
                .label(ids!(bubble_container.content_zh))
                .set_text(cx, content_zh);
            inner
                .view
                .label(ids!(bubble_container.timestamp_row.timestamp))
                .set_text(cx, timestamp);
            inner.view.redraw(cx);
        }
    }

    pub fn set_audio_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .view
                .button(ids!(bubble_container.timestamp_row.audio_btn))
                .set_visible(cx, visible);
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct AiBubble {
    #[deref]
    view: View,
}

impl Widget for AiBubble {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl AiBubbleRef {
    #[allow(unused_mut)]
    pub fn set_content(&self, cx: &mut Cx, content_en: &str, content_zh: &str, timestamp: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner
                .view
                .label(ids!(bubble_container.content_en))
                .set_text(cx, content_en);
            inner
                .view
                .label(ids!(bubble_container.content_zh))
                .set_text(cx, content_zh);
            inner
                .view
                .label(ids!(bubble_container.timestamp_row.timestamp))
                .set_text(cx, timestamp);
            inner.view.redraw(cx);
        }
    }

    pub fn set_favorite(&self, cx: &mut Cx, is_favorite: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            let btn = inner
                .view
                .button(ids!(bubble_container.timestamp_row.favorite_btn));
            btn.set_text(cx, if is_favorite { "♥" } else { "♡" });
            btn.apply_over(
                cx,
                live! {
                    draw_text: { is_favorite: (if is_favorite { 1.0 } else { 0.0 }) }
                },
            );
        }
    }

    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.view(ids!(avatar)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            inner.view.view(ids!(bubble_container)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            inner
                .view
                .label(ids!(bubble_container.content_en))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner.view.view(ids!(bubble_container.divider)).apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            inner
                .view
                .label(ids!(bubble_container.content_zh))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .label(ids!(bubble_container.timestamp_row.timestamp))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .button(ids!(bubble_container.timestamp_row.audio_btn))
                .apply_over(
                    cx,
                    live! {
                        draw_text: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .button(ids!(bubble_container.timestamp_row.favorite_btn))
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

#[derive(Live, LiveHook, Widget)]
pub struct MessageList {
    #[deref]
    view: View,
}

impl Widget for MessageList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
