use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use widgets::theme::*;

    CardBase = <RoundedView> {
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
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

    MutedText = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_REGULAR>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
            }
        }
    }

    SectionTitle = <Label> {
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    ReviewTabButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 14, right: 14, top: 8, bottom: 8}
        margin: {right: 8}
        draw_bg: {
            instance dark_mode: 0.0
            instance selected: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0);
                let normal = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let active = mix((ACCENT_INDIGO), (ACCENT_INDIGO), self.dark_mode);
                sdf.fill(mix(normal, active, self.selected));
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            instance selected: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
            fn get_color(self) -> vec4 {
                let normal = mix((SLATE_600), (SLATE_200), self.dark_mode);
                return mix(normal, (WHITE), self.selected);
            }
        }
    }

    ReviewActionButton = <Button> {
        width: Fill, height: Fit
        padding: {left: 12, right: 12, top: 12, bottom: 12}
        draw_bg: {
            instance dark_mode: 0.0
            instance tint: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                let base = mix((SLATE_50), (SLATE_800), self.dark_mode);
                let tint = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(mix(base, tint, self.tint));
                return sdf.result;
            }
        }
        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((SLATE_700), (SLATE_200), self.dark_mode);
            }
        }
    }

    MiniStat = <PanelBase> {
        width: Fill, height: Fit
        padding: 12
        flow: Down
        align: {x: 0.5}
        stat_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 18.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
        stat_label = <MutedText> {}
    }

    CalendarCell = <RoundedView> {
        width: 14, height: 14
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0
            instance done: 0.0
            border_radius: 3.0
            fn get_color(self) -> vec4 {
                let normal = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let done = mix((ACCENT_GREEN), (ACCENT_GREEN), self.dark_mode);
                let active = mix((ACCENT_INDIGO), (ACCENT_INDIGO), self.dark_mode);
                return mix(mix(normal, done, self.done), active, self.active);
            }
        }
    }

    ProgressBar = <RoundedView> {
        width: 200, height: 8
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance progress: 0.7
            border_radius: 4.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                let bg = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(bg);
                let fill_w = self.rect_size.x * self.progress;
                sdf.box(0., 0., fill_w, self.rect_size.y, 4.0);
                let fill = mix((ACCENT_GREEN), (ACCENT_INDIGO), self.dark_mode);
                sdf.fill(fill);
                return sdf.result;
            }
        }
    }

    ReviewDuePage = <View> {
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
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            fn get_color(self) -> vec4 {
                                return (ACCENT_INDIGO);
                            }
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
                    <MutedText> { text: "这些内容会自动融入你的下次 AI 对话中" }
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

    pub ReviewScene = {{ReviewScene}} {
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
                spacing: 12
                padding: {left: 16, right: 16, top: 16, bottom: 16}

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <Label> {
                        text: "📚 复习中心"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 18.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <View> { width: Fill }
                    tabs = <View> {
                        width: Fit, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        due_tab = <ReviewTabButton> {
                            text: "待复习"
                            draw_bg: { selected: 1.0 }
                            draw_text: { selected: 1.0 }
                        }
                        mistakes_tab = <ReviewTabButton> { text: "易错点" }
                        mastered_tab = <ReviewTabButton> { text: "已掌握" }
                        stats_tab = <ReviewTabButton> { text: "统计" }
                    }
                }

                pages = <PageFlip> {
                    width: Fill, height: Fit
                    active_page: due_page
                    due_page = <ReviewDuePage> {}
                    mistakes_page = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 12
                        <CardBase> {
                            width: Fill, height: Fit
                            padding: 16
                            <SectionTitle> { text: "易错点" }
                            <MutedText> { text: "这里展示易错点列表与专项练习入口" }
                        }
                    }
                    mastered_page = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 12
                        <CardBase> {
                            width: Fill, height: Fit
                            padding: 16
                            <SectionTitle> { text: "已掌握" }
                            <MutedText> { text: "这里展示已掌握卡片与复测入口" }
                        }
                    }
                    stats_page = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 12
                        <CardBase> {
                            width: Fill, height: Fit
                            padding: 16
                            <SectionTitle> { text: "统计" }
                            <MutedText> { text: "这里展示遗忘曲线、难度分布与连胜趋势" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum ReviewTab {
    #[default]
    Due,
    Mistakes,
    Mastered,
    Stats,
}

#[derive(Live, LiveHook, Widget)]
pub struct ReviewScene {
    #[deref]
    view: View,
    #[rust]
    tab: ReviewTab,
}

impl Widget for ReviewScene {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.due_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Due;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.mistakes_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Mistakes;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.mastered_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Mastered;
            self.apply_tab_state(cx);
        }
        if self
            .view
            .button(ids!(content_scroll.content.header_row.tabs.stats_tab))
            .clicked(actions)
        {
            self.tab = ReviewTab::Stats;
            self.apply_tab_state(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReviewScene {
    fn apply_tab_state(&mut self, cx: &mut Cx) {
        let is_due = self.tab == ReviewTab::Due;
        let is_mistakes = self.tab == ReviewTab::Mistakes;
        let is_mastered = self.tab == ReviewTab::Mastered;
        let is_stats = self.tab == ReviewTab::Stats;

        self.view
            .button(ids!(content_scroll.content.header_row.tabs.due_tab))
            .apply_over(cx, live! { draw_bg: { selected: (if is_due { 1.0 } else { 0.0 }) } draw_text: { selected: (if is_due { 1.0 } else { 0.0 }) } });
        self.view
            .button(ids!(content_scroll.content.header_row.tabs.mistakes_tab))
            .apply_over(cx, live! { draw_bg: { selected: (if is_mistakes { 1.0 } else { 0.0 }) } draw_text: { selected: (if is_mistakes { 1.0 } else { 0.0 }) } });
        self.view
            .button(ids!(content_scroll.content.header_row.tabs.mastered_tab))
            .apply_over(cx, live! { draw_bg: { selected: (if is_mastered { 1.0 } else { 0.0 }) } draw_text: { selected: (if is_mastered { 1.0 } else { 0.0 }) } });
        self.view
            .button(ids!(content_scroll.content.header_row.tabs.stats_tab))
            .apply_over(cx, live! { draw_bg: { selected: (if is_stats { 1.0 } else { 0.0 }) } draw_text: { selected: (if is_stats { 1.0 } else { 0.0 }) } });

        let page = match self.tab {
            ReviewTab::Due => live_id!(due_page),
            ReviewTab::Mistakes => live_id!(mistakes_page),
            ReviewTab::Mastered => live_id!(mastered_page),
            ReviewTab::Stats => live_id!(stats_page),
        };
        self.view
            .page_flip(ids!(content_scroll.content.pages))
            .set_active_page(cx, page);

        self.view.redraw(cx);
    }
}
