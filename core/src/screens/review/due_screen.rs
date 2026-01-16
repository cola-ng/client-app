use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_component::*;

    use widgets::theme::*;

    use crate::screens::review::components::CalendarCell;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::MiniStat;
    use crate::screens::review::components::MutedText;
    use crate::screens::review::components::PanelBase;
    use crate::screens::review::components::ProgressBar;
    use crate::screens::review::components::ReviewActionButton;
    use crate::screens::review::components::SectionTitle;

    pub DueScreen = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 16

        left = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 16

            today_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Right
                spacing: 12

                left_side = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    header_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        <SectionTitle> { text: "📋 今日复习" }
                    }

                    stats_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        due_stat = <MiniStat> {
                            stat_value = { text: "23" }
                            stat_label = { text: "待复习" }
                        }
                        done_stat = <MiniStat> {
                            stat_value = { text: "8" }
                            stat_label = { text: "已完成" }
                        }
                        new_stat = <MiniStat> {
                            stat_value = { text: "5" }
                            stat_label = { text: "新词" }
                        }
                    }
                }

                start_col = <View> {
                    width: 44, height: Fill
                    margin: {left: 6}
                    flow: Down
                    align: {x: 0.5, y: 0.5}
                    start_btn = <Button> {
                        width: 34, height: Fill
                        text: "开始"
                        draw_bg: {
                            border_radius: 12.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                                sdf.fill((ACCENT_INDIGO));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_BOLD>{ font_size: 12.0 }
                            color: (WHITE)
                        }
                    }
                }
            }

            error_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 12

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <SectionTitle> { text: "⚠️ 易错点分类" }
                    <View> { width: Fill }
                    view_all = <Label> {
                        text: "查看全部 →"
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (ACCENT_INDIGO)
                        }
                    }
                }

                cat_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 10

                    cat_grammar = <PanelBase> {
                        width: Fill, height: Fit
                        padding: 12
                        flow: Down
                        align: {x: 0.5}
                        cat_num = <Label> { text: "12"
                            draw_text: { text_style: <FONT_BOLD>{ font_size: 16.0 } color: (SLATE_600) }
                        }
                        cat_name = <MutedText> { text: "语法错误" }
                        cat_desc = <MutedText> { text: "时态 · 冠词 · 介词" }
                    }

                    cat_pron = <PanelBase> {
                        width: Fill, height: Fit
                        padding: 12
                        flow: Down
                        align: {x: 0.5}
                        cat_num = <Label> { text: "8"
                            draw_text: { text_style: <FONT_BOLD>{ font_size: 16.0 } color: (SLATE_600) }
                        }
                        cat_name = <MutedText> { text: "发音问题" }
                        cat_desc = <MutedText> { text: "th · r · 重音" }
                    }

                    cat_wording = <PanelBase> {
                        width: Fill, height: Fit
                        padding: 12
                        flow: Down
                        align: {x: 0.5}
                        cat_num = <Label> { text: "5"
                            draw_text: { text_style: <FONT_BOLD>{ font_size: 16.0 } color: (SLATE_600) }
                        }
                        cat_name = <MutedText> { text: "用词不当" }
                        cat_desc = <MutedText> { text: "搭配 · 语境" }
                    }
                }

                item_row = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    item_text = <Label> {
                        text: "🔸 want to + 动词原形"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <View> { width: Fill }
                    item_count = <MutedText> { text: "出错 3 次" }
                    practice_btn = <Button> {
                        text: "练习"
                        margin: {left: 8}
                        padding: {left: 12, right: 12, top: 8, bottom: 8}
                        draw_bg: {
                            border_radius: 16.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                                sdf.fill((SLATE_200));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (SLATE_700)
                        }
                    }
                }
            }

            cards_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 12

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <SectionTitle> { text: "🃏 复习卡片" }
                    <View> { width: Fill }
                    progress_label = <MutedText> { text: "12/23" }
                }

                current_card = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 16
                    flow: Down
                    spacing: 10
                    align: {x: 0.5}

                    word = <Label> {
                        text: "reservation"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 22.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    phon = <MutedText> { text: "/ˌrez.əˈveɪ.ʃən/" }
                    play_btn = <Button> {
                        text: "🔊 播放"
                        padding: {left: 14, right: 14, top: 8, bottom: 8}
                        draw_bg: {
                            border_radius: 16.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                                sdf.fill((SLATE_200));
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (SLATE_700)
                        }
                    }
                    hint = <MutedText> { text: "点击显示释义，或直接评估掌握程度" }
                }

                actions_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 10

                    btn_bad = <ReviewActionButton> { text: "😰 完全不会" draw_bg: { tint: 0.15 } }
                    btn_warn = <ReviewActionButton> { text: "🤔 有点印象" draw_bg: { tint: 0.10 } }
                    btn_ok = <ReviewActionButton> { text: "😊 记得" draw_bg: { tint: 0.08 } }
                    btn_good = <ReviewActionButton> { text: "🎯 掌握" draw_bg: { tint: 0.22 } }
                }
            }
        }

        right = <View> {
            width: 420, height: Fit
            flow: Down
            spacing: 16

            calendar_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <SectionTitle> { text: "📅 学习日历" }
                    <View> { width: Fill }
                    <MutedText> { text: "本月" }
                }

                grid = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 6

                    col_labels = <View> {
                        width: 20, height: Fit
                        flow: Down
                        spacing: 6
                        <MutedText> { text: "一" }
                        <MutedText> { text: "三" }
                        <MutedText> { text: "五" }
                        <MutedText> { text: "日" }
                    }

                    cells = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 6

                        row1 = <View> { width: Fill, height: Fit flow: Right spacing: 6
                            c1 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c2 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c3 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c4 = <CalendarCell> {}
                            c5 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c6 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c7 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c8 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c9 = <CalendarCell> {}
                            c10 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c11 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c12 = <CalendarCell> { draw_bg: { active: 1.0 } }
                        }
                        row2 = <View> { width: Fill, height: Fit flow: Right spacing: 6
                            c1 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c2 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c3 = <CalendarCell> {}
                            c4 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c5 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c6 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c7 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c8 = <CalendarCell> {}
                            c9 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c10 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c11 = <CalendarCell> { draw_bg: { done: 1.0 } }
                            c12 = <CalendarCell> {}
                        }
                    }
                }

                footer_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <Label> {
                        text: "🔥 连续学习 12 天"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return mix((ACCENT_GREEN), (ACCENT_GREEN), self.dark_mode);
                            }
                        }
                    }
                    <View> { width: Fill }
                    <MutedText> { text: "本月复习 156 个词汇" }
                }
            }

            ai_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10
                <SectionTitle> { text: "🧠 智能复习建议" }
                panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 6
                    <Label> {
                        text: "💡 根据你的学习数据，建议今天重点复习："
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <Label> {
                        text: "• 过去时态相关词汇（遗忘率较高）"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return (ACCENT_INDIGO);
                            }
                        }
                    }
                    <MutedText> { text: "这些内容会自动融入你的下次 交流对话中" }
                }
            }

            detail_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10
                <SectionTitle> { text: "📖 词汇详情" }
                detail_panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 10

                    title_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        <Label> {
                            text: "reservation"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_BOLD>{ font_size: 16.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                        <View> { width: Fill }
                        <MutedText> { text: "n. 预订；预约" }
                    }

                    pron_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        <MutedText> { text: "发音：" }
                        <Label> {
                            text: "/ˌrez.əˈveɪ.ʃən/"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                        <View> { width: Fill }
                        us_btn = <Button> {
                            text: "🔊 美"
                            padding: {left: 10, right: 10, top: 6, bottom: 6}
                            draw_bg: { border_radius: 16.0 fn pixel(self)->vec4{
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0.,0.,self.rect_size.x,self.rect_size.y,16.0);
                                sdf.fill((SLATE_200));
                                return sdf.result;
                            }}
                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 11.0 } color: (SLATE_700) }
                        }
                        uk_btn = <Button> {
                            text: "🔊 英"
                            margin: {left: 8}
                            padding: {left: 10, right: 10, top: 6, bottom: 6}
                            draw_bg: { border_radius: 16.0 fn pixel(self)->vec4{
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0.,0.,self.rect_size.x,self.rect_size.y,16.0);
                                sdf.fill((SLATE_200));
                                return sdf.result;
                            }}
                            draw_text: { text_style: <FONT_MEDIUM>{ font_size: 11.0 } color: (SLATE_700) }
                        }
                    }

                    example = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 4
                        <MutedText> { text: "例句：" }
                        <Label> {
                            text: "I'd like to make a reservation for two."
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }

                    source = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        <MutedText> { text: "来源：" }
                        <Label> {
                            text: "酒店入住场景 · 1月10日对话"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return (ACCENT_INDIGO);
                                }
                            }
                        }
                    }

                    mastery = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        <MutedText> { text: "掌握度：" }
                        bar = <ProgressBar> { draw_bg: { progress: 0.7 } }
                        <View> { width: Fill }
                        <Label> {
                            text: "70%"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                                fn get_color(self) -> vec4 {
                                    return (ACCENT_GREEN);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
