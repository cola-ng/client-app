use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use widgets::theme::*;

    use crate::scenes::review::components::CardBase;
    use crate::scenes::review::components::MutedText;
    use crate::scenes::review::components::PanelBase;
    use crate::scenes::review::components::PillButton;
    use crate::scenes::review::components::PrimaryButton;
    use crate::scenes::review::components::SecondaryButton;
    use crate::scenes::review::components::SectionTitle;

    pub MistakesScreen = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 16

        left = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 16

            overview_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 8

                <SectionTitle> { text: "🧭 易错点概览" }
                <MutedText> { text: "把错误当作训练入口：按影响优先级排序，避免盲目刷题" }

                filters = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    filter_type = <PillButton> { text: "筛选：全部类型" }
                    filter_sort = <PillButton> { text: "排序：影响优先" }
                    filter_range = <PillButton> { text: "范围：近 7 天" }
                    <View> { width: Fill }
                    search_btn = <SecondaryButton> { text: "搜索" }
                }
            }

            categories_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 12

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <SectionTitle> { text: "🧩 分类与专项" }
                    <View> { width: Fill }
                    <Label> {
                        text: "生成训练计划 →"
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

                    <PanelBase> {
                        width: Fill, height: Fit
                        padding: 12
                        flow: Down
                        spacing: 4
                        <Label> { text: "12" draw_text: { text_style: <FONT_BOLD>{ font_size: 16.0 } color: (SLATE_600) } }
                        <MutedText> { text: "语法错误" }
                        <MutedText> { text: "时态·冠词·介词" }
                    }
                    <PanelBase> {
                        width: Fill, height: Fit
                        padding: 12
                        flow: Down
                        spacing: 4
                        <Label> { text: "8" draw_text: { text_style: <FONT_BOLD>{ font_size: 16.0 } color: (SLATE_600) } }
                        <MutedText> { text: "发音问题" }
                        <MutedText> { text: "th·r·重音" }
                    }
                    <PanelBase> {
                        width: Fill, height: Fit
                        padding: 12
                        flow: Down
                        spacing: 4
                        <Label> { text: "5" draw_text: { text_style: <FONT_BOLD>{ font_size: 16.0 } color: (SLATE_600) } }
                        <MutedText> { text: "用词不当" }
                        <MutedText> { text: "搭配·语境" }
                    }
                }
            }

            list_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 12

                header_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    <SectionTitle> { text: "📌 易错点列表（可点进训练）" }
                    <View> { width: Fill }
                    <MutedText> { text: "Top 10" }
                }

                item1 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    <Label> {
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
                    <MutedText> { text: "出错 3 次" }
                    <SecondaryButton> { text: "练" margin: {left: 8} }
                    <PillButton> { text: "带入聊" margin: {left: 6} }
                }

                item2 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    <Label> {
                        text: "🔸 冠词：a / the 的使用"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <View> { width: Fill }
                    <MutedText> { text: "出错 2 次" }
                    <SecondaryButton> { text: "练" margin: {left: 8} }
                    <PillButton> { text: "带入聊" margin: {left: 6} }
                }

                item3 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    <Label> {
                        text: "🔸 发音：reservation 重音"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <View> { width: Fill }
                    <Label> {
                        text: "卡壳 4 次"
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (ACCENT_RED)
                        }
                    }
                    <SecondaryButton> { text: "练" margin: {left: 8} }
                    <PillButton> { text: "带入聊" margin: {left: 6} }
                }
            }
        }

        right = <View> {
            width: 420, height: Fit
            flow: Down
            spacing: 16

            trainer_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10
                <SectionTitle> { text: "🎯 专项训练（当前选中：want to）" }
                <MutedText> { text: "1) 规则一句话 2) 15 秒替换练 3) 立刻回到复习/对话" }
                panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 6
                    <Label> {
                        text: "规则：want to + 动词原形"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <MutedText> { text: "你：I want book a room." }
                    <Label> {
                        text: "✅ 建议：I want to book a room."
                        draw_text: {
                            text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                            color: (ACCENT_GREEN)
                        }
                    }
                    actions = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8
                        <View> { width: Fill }
                        <PrimaryButton> { text: "开始 15s" }
                        <SecondaryButton> { text: "加入计划" margin: {left: 6} }
                    }
                }
            }

            ai_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10
                <SectionTitle> { text: "🧠 AI 建议（用于无感带入对话）" }
                panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 6
                    <Label> {
                        text: "今天对话里我会刻意制造 3 次 “want to” 的使用场景"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <MutedText> { text: "你只需要继续说，AI 会在对话中温和提醒，不打断" }
                    <View> { width: Fill, height: Fit }
                    <PrimaryButton> { text: "带入下一次对话" }
                }
            }

            trace_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10
                <SectionTitle> { text: "📎 错误溯源（为什么会错）" }
                panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 6
                    <Label> {
                        text: "来源：酒店入住场景 · 1月10日对话"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <MutedText> { text: "回放：你说错的那一句 + AI 当时的温和纠错" }
                    actions = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8
                        <SecondaryButton> { text: "回放" }
                        <View> { width: Fill }
                        <PrimaryButton> { text: "生成卡片" }
                    }
                }
            }
        }
    }
}
