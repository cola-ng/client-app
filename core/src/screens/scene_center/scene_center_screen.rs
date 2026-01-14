//! Scene Center - Scene selection and management
//!
//! Layout based on design/kiro/03-场景中心-sketch.svg (optimized v2):
//! - Continue Learning section with progress tracking
//! - Smart AI recommendations section
//! - Today's featured scenes
//! - Classic dialogues section
//! - Right panel with expanded scene preview and learning guide

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use widgets::theme::*;

    // ========================================================================
    // Design Tokens
    // ========================================================================

    CardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 16.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }
    }

    PanelBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }
    }

    SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    BodyText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    MutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
            }
        }
    }

    // ========================================================================
    // Scene Center Main Widget
    // ========================================================================

    pub SceneCenter = {{SceneCenter}} <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_50), (DARK_BG_DARK), self.dark_mode);
            }
        }

        // Scrollable content
        scene_list = <ScrollYView> {
            width: Fill, height: Fill
            flow: Down
            spacing: 20
            padding: {left: 40, right: 40, top: 30, bottom: 30}

            // Header
            header = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                title = <Label> {
                    text: "🎭 场景中心"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 28.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                subtitle = <Label> {
                    text: "沉浸式场景模拟 · AI智能推荐 · 经典对白学习 · 多口音体验"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                        }
                    }
                }
            }

            // Search and Filters
            search_bar = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 12
                align: {y: 0.5}

                search_input = <PanelBase> {
                    width: 300, height: 36
                    padding: {left: 12, right: 12}
                    flow: Right
                    align: {y: 0.5}

                    search_icon = <Label> {
                        text: "🔍"
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        }
                    }

                    search_placeholder = <Label> {
                        text: "搜索场景..."
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                filter_chips = <View> {
                    width: Fit, height: 36
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}

                    // Filter chip component (simplified for now)
                    filter_all = <RoundedView> {
                        width: Fit, height: 32
                        padding: {left: 12, right: 12}
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: 16.0
                            fn get_color(self) -> vec4 {
                                return mix((ACCENT_INDIGO), (INDIGO_500), self.dark_mode);
                            }
                        }
                        align: {x: 0.5, y: 0.5}

                        filter_label = <Label> {
                            text: "全部"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (WHITE)
                            }
                        }
                    }
                }
            }

            // Continue Learning Section
            continue_learning_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_title = <SectionTitle> {
                    text: "📚 继续学习"
                }

                continue_card = <CardBase> {
                    width: Fill, height: 120
                    padding: 16
                    flow: Right
                    spacing: 16
                    align: {y: 0.5}

                    icon_area = <View> {
                        width: 90, height: Fit
                        flow: Down
                        align: {x: 0.5, y: 0.5}

                        icon = <Label> {
                            text: "🏨"
                            draw_text: {
                                text_style: <FONT_BOLD>{ font_size: 48.0 }
                            }
                        }
                    }

                    content = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 8

                        title = <Label> {
                            text: "酒店入住"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        progress = <Label> {
                            text: "进度 60% · 还剩 3 个对话"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }

                        task = <Label> {
                            text: "下一个任务：前台预订房间"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        estimate = <Label> {
                            text: "预计耗时 8 分钟"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    button = <Button> {
                        width: 120, height: 40
                        text: "继续学习 →"
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let r = 8.0;
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, r);
                                let color = mix((ACCENT_INDIGO), (INDIGO_500), self.dark_mode);
                                sdf.fill(color);
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            color: (WHITE)
                        }
                    }
                }
            }

            // Smart Recommendations Section
            smart_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_header = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 8

                    section_title = <SectionTitle> {
                        text: "✨ 智能推荐"
                    }

                    section_subtitle = <Label> {
                        text: "基于你的学习历史 · AI为你挑选"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                smart_cards = <View> {
                    width: Fill, height: 200
                    flow: Right
                    spacing: 16
                    scroll_on: ("x", "y")

                    card1 = <CardBase> {
                        width: 200, height: Fill
                        padding: 12
                        flow: Down
                        spacing: 8

                        icon_area = <View> {
                            width: Fill, height: 85
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.396, 0.416, 0.961, 0.2);  // Indigo with transparency
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 12.0

                            icon = <Label> {
                                text: "🎬"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 36.0 }
                                }
                            }
                        }

                        ai_badge = <RoundedView> {
                            width: 45, height: 18
                            show_bg: true
                            draw_bg: {
                                fn get_color(self) -> vec4 {
                                    return vec4(0.553, 0.314, 0.996, 1.0);  // Purple
                                }
                            }
                            border_radius: 10.0
                            align: {x: 0.5, y: 0.5}

                            badge_text = <Label> {
                                text: "推荐"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 10.0 }
                                    color: (WHITE)
                                }
                            }
                        }

                        title = <Label> {
                            text: "电话客服"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        level = <Label> {
                            text: "⭐⭐ 中级"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }

                        description = <Label> {
                            text: "基于你最近的兴趣"
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                color: (ACCENT_INDIGO)
                            }
                        }

                        relevance = <Label> {
                            text: "💡 提示相关性 92%"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 10.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }
                }
            }

            // Today's Scenes Section
            today_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_title = <SectionTitle> {
                    text: "🌟 今日精选"
                }

                today_cards = <View> {
                    width: Fill, height: 160
                    flow: Right
                    spacing: 12

                    card1 = <CardBase> {
                        width: 180, height: Fill
                        padding: 12
                        flow: Down
                        spacing: 8

                        icon_area = <View> {
                            width: Fill, height: 75
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.063, 0.725, 0.502, 0.2);  // Green
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 12.0

                            icon = <Label> {
                                text: "🍽️"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                        }

                        title = <Label> {
                            text: "餐厅点餐"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        info = <Label> {
                            text: "⭐⭐ · 5分钟"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    card2 = <CardBase> {
                        width: 180, height: Fill
                        padding: 12
                        flow: Down
                        spacing: 8

                        icon_area = <View> {
                            width: Fill, height: 75
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.961, 0.624, 0.043, 0.2);  // Amber
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 12.0

                            icon = <Label> {
                                text: "✈️"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                        }

                        title = <Label> {
                            text: "机场登机"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        info = <Label> {
                            text: "⭐⭐ · 6分钟"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    card3 = <CardBase> {
                        width: 180, height: Fill
                        padding: 12
                        flow: Down
                        spacing: 8

                        icon_area = <View> {
                            width: Fill, height: 75
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.396, 0.416, 0.961, 0.2);  // Indigo
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 12.0

                            icon = <Label> {
                                text: "💼"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                        }

                        title = <Label> {
                            text: "工作面试"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        info = <Label> {
                            text: "⭐⭐⭐ · 10分钟"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    card4 = <CardBase> {
                        width: 180, height: Fill
                        padding: 12
                        flow: Down
                        spacing: 8

                        icon_area = <View> {
                            width: Fill, height: 75
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.063, 0.725, 0.502, 0.2);  // Green
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 12.0

                            icon = <Label> {
                                text: "🛒"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                                }
                            }
                        }

                        title = <Label> {
                            text: "超市购物"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        info = <Label> {
                            text: "⭐ · 4分钟"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }
                }
            }

            // Classic Dialogues Section
            classic_section = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 12

                section_header = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    align: {y: 0.5}

                    section_title = <SectionTitle> {
                        text: "🎬 经典对白"
                    }

                    section_subtitle = <Label> {
                        text: "从电影/美剧中学习地道表达"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                            }
                        }
                    }
                }

                classic_cards = <View> {
                    width: Fill, height: 100
                    flow: Right
                    spacing: 16

                    card1 = <CardBase> {
                        width: 220, height: Fill
                        padding: 12
                        flow: Right
                        spacing: 12

                        icon_area = <View> {
                            width: 60, height: 60
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.1, 0.1, 0.18, 1.0);  // Dark
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 8.0

                            icon = <Label> {
                                text: "🎥"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 28.0 }
                                }
                            }
                        }

                        content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 4
                            align: {y: 0.5}

                            title = <Label> {
                                text: "《当幸福来敲门》"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            description = <Label> {
                                text: "经典面试场景"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                    color: (ACCENT_INDIGO)
                                }
                            }
                        }
                    }

                    card2 = <CardBase> {
                        width: 220, height: Fill
                        padding: 12
                        flow: Right
                        spacing: 12

                        icon_area = <View> {
                            width: 60, height: 60
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.1, 0.1, 0.18, 1.0);
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 8.0

                            icon = <Label> {
                                text: "📺"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 28.0 }
                                }
                            }
                        }

                        content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 4
                            align: {y: 0.5}

                            title = <Label> {
                                text: "《老友记》"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            description = <Label> {
                                text: "日常生活场景"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                    color: (ACCENT_INDIGO)
                                }
                            }
                        }
                    }

                    card3 = <CardBase> {
                        width: 220, height: Fill
                        padding: 12
                        flow: Right
                        spacing: 12

                        icon_area = <View> {
                            width: 60, height: 60
                            show_bg: true
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return vec4(0.1, 0.1, 0.18, 1.0);
                                }
                            }
                            align: {x: 0.5, y: 0.5}
                            border_radius: 8.0

                            icon = <Label> {
                                text: "🎤"
                                draw_text: {
                                    text_style: <FONT_BOLD>{ font_size: 28.0 }
                                }
                            }
                        }

                        content = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 4
                            align: {y: 0.5}

                            title = <Label> {
                                text: "TED 演讲精选"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            description = <Label> {
                                text: "演讲技巧提升"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                    color: (ACCENT_INDIGO)
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
pub struct SceneCenter {
    #[deref]
    view: View,
}

impl Widget for SceneCenter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
