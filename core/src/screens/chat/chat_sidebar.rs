//! Chat sidebar component
//!
//! Provides the left sidebar with:
//! - Header with "随便聊" + "选场景" toggle buttons
//! - Dynamic chat session list with name, preview, timestamp
//! - Context menu for rename, delete operations
//! - New chat button

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    ORANGE_500 = #f97316
    ORANGE_100 = #ffedd5

    // Chat type toggle button
    ChatTypeBtn = <RoundedView> {
        width: Fill, height: 32
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance dark_mode: 0.0
            border_radius: 6.0
            fn pixel(self) -> vec4 {
                let orange = vec4(0.976, 0.451, 0.086, 1.0);
                let light_unselected = vec4(1.0, 1.0, 1.0, 1.0);
                let dark_unselected = vec4(0.216, 0.255, 0.318, 1.0);
                let unselected = mix(light_unselected, dark_unselected, self.dark_mode);
                return mix(unselected, orange, self.selected);
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand
    }

    // Conversation list item
    ConvListItem = <View> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 10, bottom: 10}
        flow: Right
        spacing: 10
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance hover: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let light_normal = vec4(0.976, 0.980, 0.984, 1.0);  // gray-50
                let light_hover = vec4(1.0, 1.0, 1.0, 1.0);
                let light_selected = vec4(1.0, 1.0, 1.0, 1.0);

                let dark_normal = vec4(0.059, 0.090, 0.165, 1.0);   // slate-900
                let dark_hover = vec4(0.118, 0.141, 0.227, 1.0);    // slate-800
                let dark_selected = vec4(0.118, 0.141, 0.227, 1.0);

                let normal = mix(light_normal, dark_normal, self.dark_mode);
                let hover = mix(light_hover, dark_hover, self.dark_mode);
                let selected = mix(light_selected, dark_selected, self.dark_mode);

                let base = mix(normal, hover, self.hover);
                return mix(base, selected, self.selected);
            }
        }

        // Left border indicator for selected item
        selected_indicator = <View> {
            width: 4, height: Fill
            show_bg: true
            draw_bg: {
                instance selected: 0.0
                fn pixel(self) -> vec4 {
                    let orange = vec4(0.976, 0.451, 0.086, 1.0);
                    return mix(vec4(0.0, 0.0, 0.0, 0.0), orange, self.selected);
                }
            }
        }

        conv_avatar = <RoundedView> {
            width: 36, height: 36
            show_bg: true
            draw_bg: {
                color: (ORANGE_100)
                border_radius: 18.0
            }
            align: {x: 0.5, y: 0.5}

            icon_label = <Label> {
                text: "💬"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                }
            }
        }

        conv_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 3

            title_row = <View> {
                width: Fill, height: Fit
                flow: Right
                align: {y: 0.5}

                conv_title = <Label> {
                    width: Fill, height: Fit
                    text: "新对话"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                conv_time = <Label> {
                    width: Fit, height: Fit
                    text: "刚刚"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 10.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }
            }

            conv_preview = <Label> {
                width: Fill, height: Fit
                text: "开始对话..."
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                    }
                }
            }
        }

        // Menu button (visible on hover)
        menu_btn = <Button> {
            width: 24, height: 24
            visible: false
            text: "⋮"
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                    sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.0, 0.0, 0.0, 0.1), self.hover));
                    return sdf.result;
                }
            }
            draw_text: {
                text_style: <FONT_BOLD>{ font_size: 14.0 }
                color: (TEXT_MUTED)
            }
            animator: {
                hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
            }
        }
    }

    // Context menu popup
    ContextMenu = <RoundedView> {
        width: 120, height: Fit
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let light = vec4(1.0, 1.0, 1.0, 1.0);
                let dark = vec4(0.118, 0.141, 0.227, 1.0);
                sdf.fill(mix(light, dark, self.dark_mode));
                sdf.stroke(mix(vec4(0.0, 0.0, 0.0, 0.1), vec4(1.0, 1.0, 1.0, 0.1), self.dark_mode), 1.0);
                return sdf.result;
            }
        }
        flow: Down
        padding: 4

        pin_btn = <Button> {
            width: Fill, height: 32
            text: "📌 置顶"
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    return mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.0, 0.0, 0.0, 0.05), self.hover);
                }
            }
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: (TEXT_PRIMARY)
            }
            animator: {
                hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
            }
        }

        rename_btn = <Button> {
            width: Fill, height: 32
            text: "✏️ 重命名"
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    return mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.0, 0.0, 0.0, 0.05), self.hover);
                }
            }
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: (TEXT_PRIMARY)
            }
            animator: {
                hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
            }
        }

        clear_btn = <Button> {
            width: Fill, height: 32
            text: "🔄 清空"
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    return mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.0, 0.0, 0.0, 0.05), self.hover);
                }
            }
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: #f97316  // orange
            }
            animator: {
                hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
            }
        }

        delete_btn = <Button> {
            width: Fill, height: 32
            text: "🗑️ 删除"
            draw_bg: {
                instance hover: 0.0
                fn pixel(self) -> vec4 {
                    return mix(vec4(0.0, 0.0, 0.0, 0.0), vec4(0.0, 0.0, 0.0, 0.05), self.hover);
                }
            }
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: #ef4444  // red
            }
            animator: {
                hover = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
                down = { default: off, off = { from: {all: Snap} apply: {} } on = { from: {all: Snap} apply: {} } }
            }
        }
    }

    // Main sidebar component
    pub ChatSidebar = {{ChatSidebar}} {
        width: 280, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix(vec4(0.976, 0.980, 0.984, 1.0), (SLATE_900), self.dark_mode);
            }
        }

        // Header with chat type toggles
        header_section = <View> {
            width: Fill, height: Fit
            padding: 12
            flow: Down
            spacing: 8
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn get_color(self) -> vec4 {
                    return mix((WHITE), (SLATE_800), self.dark_mode);
                }
            }

            toggle_row = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 8

                free_chat_btn = <ChatTypeBtn> {
                    draw_bg: { selected: 1.0 }
                    <Label> {
                        text: "随便聊"
                        draw_text: {
                            instance selected: 1.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (WHITE), self.selected);
                            }
                        }
                    }
                }

                context_chat_btn = <ChatTypeBtn> {
                    <Label> {
                        text: "选场景"
                        draw_text: {
                            instance selected: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (WHITE), self.selected);
                            }
                        }
                    }
                }
            }

            new_chat_btn = <Button> {
                width: Fill, height: 36
                text: "+ 新对话"
                draw_text: {
                    text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                    color: (WHITE)
                }
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill(vec4(0.976, 0.451, 0.086, 1.0));
                        return sdf.result;
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

        // Conversation list
        conv_scroll = <ScrollYView> {
            width: Fill, height: Fill
            flow: Down

            conv_list = <View> {
                width: Fill, height: Fit
                flow: Down

                // Items will be added dynamically
                // For now, show static placeholder items
                item_1 = <ConvListItem> {
                    draw_bg: { selected: 1.0 }
                    selected_indicator = { draw_bg: { selected: 1.0 } }
                    conv_content = {
                        title_row = {
                            conv_title = { text: "旅行计划讨论" }
                            conv_time = { text: "30分钟前" }
                        }
                        conv_preview = { text: "That sounds like a great trip!" }
                    }
                }

                item_2 = <ConvListItem> {
                    conv_content = {
                        title_row = {
                            conv_title = { text: "工作面试准备" }
                            conv_time = { text: "2小时前" }
                        }
                        conv_preview = { text: "Let's practice some common questions." }
                    }
                }

                item_3 = <ConvListItem> {
                    conv_avatar = {
                        icon_label = { text: "🍽️" }
                    }
                    conv_content = {
                        title_row = {
                            conv_title = { text: "餐厅点餐练习" }
                            conv_time = { text: "昨天" }
                        }
                        conv_preview = { text: "Would you like to see the menu?" }
                    }
                }
            }
        }
    }
}

/// Actions emitted by the sidebar
#[derive(Clone, Debug, DefaultNone)]
pub enum ChatSidebarAction {
    NewFreeChat,
    OpenContextDialog,
    SelectChat(usize),
    PinChat(usize),
    RenameChat(usize),
    ClearChat(usize),
    DeleteChat(usize),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct ChatSidebar {
    #[deref]
    view: View,
    #[rust]
    selected_index: usize,
    #[rust]
    chat_mode: ChatMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ChatMode {
    #[default]
    FreeChat,
    ContextChat,
}

impl Widget for ChatSidebar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => &[],
        };

        // Handle new chat button
        if self
            .view
            .button(ids!(header_section.new_chat_btn))
            .clicked(actions)
        {
            match self.chat_mode {
                ChatMode::FreeChat => {
                    cx.widget_action(
                        self.view.widget_uid(),
                        &scope.path,
                        ChatSidebarAction::NewFreeChat,
                    );
                }
                ChatMode::ContextChat => {
                    cx.widget_action(
                        self.view.widget_uid(),
                        &scope.path,
                        ChatSidebarAction::OpenContextDialog,
                    );
                }
            }
        }

        // Handle free chat toggle
        if self
            .view
            .view(ids!(header_section.toggle_row.free_chat_btn))
            .finger_up(actions)
            .is_some()
        {
            self.set_chat_mode(cx, ChatMode::FreeChat);
        }

        // Handle context chat toggle
        if self
            .view
            .view(ids!(header_section.toggle_row.context_chat_btn))
            .finger_up(actions)
            .is_some()
        {
            self.set_chat_mode(cx, ChatMode::ContextChat);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ChatSidebar {
    fn set_chat_mode(&mut self, cx: &mut Cx, mode: ChatMode) {
        if self.chat_mode == mode {
            return;
        }
        self.chat_mode = mode;

        let (free_selected, context_selected) = match mode {
            ChatMode::FreeChat => (1.0, 0.0),
            ChatMode::ContextChat => (0.0, 1.0),
        };

        self.view
            .view(ids!(header_section.toggle_row.free_chat_btn))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (free_selected) }
                },
            );
        self.view
            .view(ids!(header_section.toggle_row.context_chat_btn))
            .apply_over(
                cx,
                live! {
                    draw_bg: { selected: (context_selected) }
                },
            );

        self.view.redraw(cx);
    }
}

impl ChatSidebarRef {
    pub fn update_dark_mode(&self, cx: &mut Cx, dark_mode: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { dark_mode: (dark_mode) }
                },
            );
            inner
                .view
                .view(ids!(header_section))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .view(ids!(header_section.toggle_row.free_chat_btn))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner
                .view
                .view(ids!(header_section.toggle_row.context_chat_btn))
                .apply_over(
                    cx,
                    live! {
                        draw_bg: { dark_mode: (dark_mode) }
                    },
                );
            inner.view.redraw(cx);
        }
    }

    pub fn set_selected_index(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.selected_index = index;
            // Update visual selection (would need to iterate items)
            inner.view.redraw(cx);
        }
    }
}
