//! Classic Dialogues Screen - Learn from movies, TV shows, and TED talks
//!
//! Features:
//! - Browse classic scenes by category
//! - Play and practice dialogues
//! - AI role-play mode
//! - Key phrase learning

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use widgets::theme::*;

    // ========================================================================
    // Classic Dialogues Components
    // ========================================================================

    CategoryTab = <Button> {
        padding: {left: 16, right: 16, top: 10, bottom: 10}
        draw_text: {
            instance dark_mode: 0.0
            instance active: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix(
                    mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode),
                    (WHITE),
                    self.active
                );
            }
        }
        draw_bg: {
            instance active: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                let inactive = mix((SLATE_100), (SLATE_700), self.dark_mode);
                let color = mix(inactive, (ACCENT_INDIGO), self.active);
                sdf.fill(color);
                return sdf.result;
            }
        }
    }

    SceneCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        spacing: 8
        cursor: Hand
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 10.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_50), (SLATE_700), self.dark_mode);
            }
        }

        thumbnail = <View> {
            width: Fill, height: 150
            show_bg: true
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix(vec4(0.12, 0.13, 0.15, 1.0), (SLATE_900), self.dark_mode);
                }
            }

            <Label> {
                text: "▶️"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 24.0 }
                    color: (TEXT_MUTED)
                }
            }
        }

        scene_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        scene_subtitle = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    FeaturedScene = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        video_area = <View> {
            width: Fill, height: 300
            show_bg: true
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    return mix(vec4(0.12, 0.13, 0.15, 1.0), (SLATE_900), self.dark_mode);
                }
            }

            play_icon = <View> {
                width: Fit, height: Fit
                flow: Down
                spacing: 8
                align: {x: 0.5}

                <Label> {
                    text: "▶️"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 32.0 }
                        color: (TEXT_MUTED)
                    }
                }

                <Label> {
                    text: "The Pursuit of Happyness (2006)"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 13.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }
        }

        scene_info = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8

            <Label> {
                text: "面试经典场景"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <Label> {
                text: "来自《当幸福来敲门》· 商务英语 · 中级"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        action_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12

            practice_btn = <Button> {
                width: Fit, height: Fit
                padding: {left: 16, right: 16, top: 10, bottom: 10}
                text: "🎭 开始练习"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    color: (WHITE)
                }
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        sdf.fill((ACCENT_GREEN));
                        return sdf.result;
                    }
                }
            }

            favorite_btn = <Button> {
                width: Fit, height: Fit
                padding: {left: 16, right: 16, top: 10, bottom: 10}
                text: "💾 收藏"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        let color = mix((SLATE_100), (SLATE_700), self.dark_mode);
                        sdf.fill(color);
                        return sdf.result;
                    }
                }
            }

            vocab_btn = <Button> {
                width: Fit, height: Fit
                padding: {left: 16, right: 16, top: 10, bottom: 10}
                text: "📊 查看词汇"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                        let color = mix((SLATE_100), (SLATE_700), self.dark_mode);
                        sdf.fill(color);
                        return sdf.result;
                    }
                }
            }
        }
    }

    // ========================================================================
    // Main Classic Dialogues Screen
    // ========================================================================

    pub ClassicDialoguesScreen = {{ClassicDialoguesScreen}} {
        width: Fill, height: Fill
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
                padding: 20
                spacing: 20

                // Header
                header = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    title_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}

                        <Label> {
                            text: "🎬 经典对白"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_BOLD>{ font_size: 24.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        <View> { width: Fill }

                        search_input = <TextInput> {
                            width: 300, height: 36
                            draw_text: {
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            }
                        }
                    }

                    <Label> {
                        text: "从电影、美剧、TED 学地道表达"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 13.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Category tabs
                category_tabs = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8

                    movie_tab = <CategoryTab> {
                        text: "🎬 电影"
                        draw_text: { active: 1.0 }
                        draw_bg: { active: 1.0 }
                    }

                    tv_tab = <CategoryTab> {
                        text: "📺 美剧"
                    }

                    ted_tab = <CategoryTab> {
                        text: "🎤 TED"
                    }

                    business_tab = <CategoryTab> {
                        text: "💼 商务"
                    }
                }

                // Main content
                main_section = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 20

                    // Featured scene
                    featured_area = <View> {
                        width: Fill, height: Fit

                        featured_scene = <FeaturedScene> {}
                    }

                    // Side details panel
                    details_panel = <RoundedView> {
                        width: 360, height: Fit
                        padding: 16
                        flow: Down
                        spacing: 16
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: 12.0
                            fn get_color(self) -> vec4 {
                                return mix((WHITE), (SLATE_800), self.dark_mode);
                            }
                        }

                        <Label> {
                            text: "📝 台词预览"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        <Label> {
                            text: "精选对白片段将在这里显示..."
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

                // More scenes
                more_scenes_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    <Label> {
                        text: "🎥 更多经典场景"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    scenes_grid = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 16

                        scene1 = <SceneCard> {
                            scene_title = { text: "Friends - Coffee Shop" }
                            scene_subtitle = { text: "日常对话 · 初级" }
                        }

                        scene2 = <SceneCard> {
                            scene_title = { text: "The Social Network" }
                            scene_subtitle = { text: "商务谈判 · 高级" }
                        }

                        scene3 = <SceneCard> {
                            scene_title = { text: "TED - Simon Sinek" }
                            scene_subtitle = { text: "演讲 · 中高级" }
                        }
                    }
                }

                // Tips
                tips = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 16
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 10.0
                        fn get_color(self) -> vec4 {
                            return mix(vec4(0.31, 0.27, 0.90, 0.1), vec4(0.31, 0.27, 0.90, 0.2), self.dark_mode);
                        }
                    }

                    <Label> {
                        text: "💡 提示: 选择你喜欢的场景，AI 会扮演对方角色与你对戏"
                        draw_text: {
                            text_style: <FONT_REGULAR>{ font_size: 13.0 }
                            color: (ACCENT_INDIGO)
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ClassicDialoguesScreen {
    #[deref]
    view: View,
}

impl Widget for ClassicDialoguesScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ClassicDialoguesScreen {
    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }
}
