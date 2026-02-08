//! # Feedback Screen
//!
//! User feedback submission with type selection and form.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Feedback type button
    FeedbackTypeBtn = <Button> {
        width: Fill, height: Fit
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        align: {x: 0.0, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance selected: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                let light_bg = mix((WHITE), (SLATE_50), self.hover);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.hover);
                let bg = mix(light_bg, dark_bg, self.dark_mode);

                let selected_light = (ORANGE_50);
                let selected_dark = #7c2d12;
                let selected_bg = mix(selected_light, selected_dark, self.dark_mode);
                let final_bg = mix(bg, selected_bg, self.selected);

                sdf.fill(final_bg);

                let normal_border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let selected_border = (ACCENT_PRIMARY);
                sdf.stroke(mix(normal_border, selected_border, self.selected), mix(1.0, 2.0, self.selected));

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }

        animator: {
            hover = {
                default: off
                off = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 0.0}} }
                on = { from: {all: Forward {duration: 0.1}} apply: {draw_bg: {hover: 1.0}} }
            }
        }
    }

    // Form section card
    FormSection = <RoundedView> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 12.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }
    }

    // Textarea style
    FeedbackTextarea = <TextInput> {
        width: Fill, height: 200
        padding: {left: 16, right: 16, top: 12, bottom: 12}
        text: "请详细描述您的问题或建议..."

        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);
                let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    // Submit button
    SubmitButton = <Button> {
        width: Fill, height: Fit
        padding: {top: 14, bottom: 14}
        align: {x: 0.5}

        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                sdf.fill((ACCENT_PRIMARY));
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
            color: (WHITE)
        }
    }

    pub FeedbackScreen = {{FeedbackScreen}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: 24
                spacing: 20

                // Header
                header = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    title = <Label> {
                        text: "意见反馈"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 24.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    subtitle = <Label> {
                        text: "您的反馈对我们很重要"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Feedback type selection
                type_section = <FormSection> {
                    section_title = <Label> {
                        text: "反馈类型"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    types_grid = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 8

                        row_1 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 8

                            bug_btn = <FeedbackTypeBtn> {
                                text: "🐛 Bug报告"
                                draw_bg: { selected: 1.0 }
                            }

                            feature_btn = <FeedbackTypeBtn> {
                                text: "💡 功能建议"
                            }
                        }

                        row_2 = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 8

                            content_btn = <FeedbackTypeBtn> {
                                text: "📚 内容问题"
                            }

                            other_btn = <FeedbackTypeBtn> {
                                text: "💬 其他"
                            }
                        }
                    }
                }

                // Feedback content
                content_section = <FormSection> {
                    section_title = <Label> {
                        text: "反馈内容"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    feedback_textarea = <FeedbackTextarea> {}

                    char_count = <Label> {
                        text: "0/500"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Contact info (optional)
                contact_section = <FormSection> {
                    section_title = <Label> {
                        text: "联系方式（可选）"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    contact_input = <TextInput> {
                        width: Fill, height: Fit
                        padding: {left: 16, right: 16, top: 12, bottom: 12}
                        text: "邮箱或其他联系方式"

                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                                sdf.fill(bg);
                                let border = mix((SLATE_300), (SLATE_600), self.dark_mode);
                                sdf.stroke(border, 1.0);
                                return sdf.result;
                            }
                        }

                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    hint = <Label> {
                        text: "留下联系方式，以便我们与您跟进"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Submit button
                submit_btn = <SubmitButton> {
                    text: "提交反馈"
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FeedbackType {
    #[default]
    Bug,
    Feature,
    Content,
    Other,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum FeedbackScreenAction {
    None,
    Submit { feedback_type: String, content: String, contact: String },
}

#[derive(Live, LiveHook, Widget)]
pub struct FeedbackScreen {
    #[deref]
    view: View,
    #[rust]
    selected_type: FeedbackType,
}

impl Widget for FeedbackScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle type selection
        let type_buttons = [
            (ids!(content_scroll.content.type_section.types_grid.row_1.bug_btn), FeedbackType::Bug),
            (ids!(content_scroll.content.type_section.types_grid.row_1.feature_btn), FeedbackType::Feature),
            (ids!(content_scroll.content.type_section.types_grid.row_2.content_btn), FeedbackType::Content),
            (ids!(content_scroll.content.type_section.types_grid.row_2.other_btn), FeedbackType::Other),
        ];

        for (btn_id, feedback_type) in type_buttons {
            if self.view.button(btn_id).clicked(actions) {
                self.select_type(cx, feedback_type);
            }
        }

        // Handle submit
        if self.view.button(ids!(content_scroll.content.submit_btn)).clicked(actions) {
            let content = self.view.text_input(ids!(content_scroll.content.content_section.feedback_textarea)).text();
            let contact = self.view.text_input(ids!(content_scroll.content.contact_section.contact_input)).text();

            let type_str = match self.selected_type {
                FeedbackType::Bug => "bug",
                FeedbackType::Feature => "feature",
                FeedbackType::Content => "content",
                FeedbackType::Other => "other",
            };

            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                FeedbackScreenAction::Submit {
                    feedback_type: type_str.to_string(),
                    content,
                    contact,
                },
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl FeedbackScreen {
    fn select_type(&mut self, cx: &mut Cx, feedback_type: FeedbackType) {
        self.selected_type = feedback_type;

        let type_buttons = [
            (ids!(content_scroll.content.type_section.types_grid.row_1.bug_btn), FeedbackType::Bug),
            (ids!(content_scroll.content.type_section.types_grid.row_1.feature_btn), FeedbackType::Feature),
            (ids!(content_scroll.content.type_section.types_grid.row_2.content_btn), FeedbackType::Content),
            (ids!(content_scroll.content.type_section.types_grid.row_2.other_btn), FeedbackType::Other),
        ];

        for (btn_id, btn_type) in type_buttons {
            let selected = if btn_type == feedback_type { 1.0 } else { 0.0 };
            self.view.button(btn_id).apply_over(cx, live! { draw_bg: { selected: (selected) } });
        }

        self.view.redraw(cx);
    }
}
