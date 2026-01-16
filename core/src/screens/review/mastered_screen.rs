use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_component::*;

    use colang_widgets::theme::*;

    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::MutedText;
    use crate::screens::review::components::PanelBase;
    use crate::screens::review::components::PillButton;
    use crate::screens::review::components::PrimaryButton;
    use crate::screens::review::components::SecondaryButton;
    use crate::screens::review::components::SectionTitle;

    pub MasteredScreen = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 16

        left = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 16

            assets_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 8
                <SectionTitle> { text: "✅ 已掌握资产库" }
                <MutedText> { text: "已掌握不等于永远不会忘：这里做低打扰复测，防止回退" }
                filters = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    <PillButton> { text: "筛选：词卡/句卡" }
                    <PillButton> { text: "排序：最近复测" }
                    <PillButton> { text: "标签：全部" }
                    <View> { width: Fill }
                    <SecondaryButton> { text: "导出" }
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
                    <SectionTitle> { text: "📚 已掌握列表（可复测）" }
                    <View> { width: Fill }
                    <MutedText> { text: "共 312" }
                }

                item1 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    left_col = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 2
                        <Label> {
                            text: "reservation"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                        <MutedText> { text: "来源：酒店入住 · 1月10日对话" }
                    }
                    <SecondaryButton> { text: "复测" }
                }

                item2 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    left_col = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 2
                        <Label> {
                            text: "make a reservation"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                        <MutedText> { text: "句卡 · 可直接加入表达库" }
                    }
                    <SecondaryButton> { text: "加入库" }
                }

                item3 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    left_col = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 2
                        <Label> {
                            text: "availability"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                        <MutedText> { text: "发音卡 · 可进行多口音对比复测" }
                    }
                    <SecondaryButton> { text: "练听" }
                }

                footer = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    <MutedText> { text: "你只需要做偶尔复测，其余由 SRS 自动安排" }
                    <View> { width: Fill }
                    <PrimaryButton> { text: "开始复测队列" }
                }
            }
        }

        right = <View> {
            width: 420, height: Fit
            flow: Down
            spacing: 16

            retest_plan = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 8
                <SectionTitle> { text: "🧪 复测计划（低打扰）" }
                panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 6
                    <Label> {
                        text: "今天只复测：6 张（每张 10 秒）"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <MutedText> { text: "原则：不影响敢开口，复测像刷牙一样轻量" }
                    actions = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8
                        <PrimaryButton> { text: "加入今日任务" }
                        <SecondaryButton> { text: "稍后提醒" margin: {left: 6} }
                    }
                }
            }

            express_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 10
                <SectionTitle> { text: "🌟 表达库（从已掌握句卡沉淀）" }
                <MutedText> { text: "下一次 交流对话优先用这些句式引导你说出来" }
                line1 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    <Label> { text: "“I’d like to make a reservation for two.”"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self)->vec4{ return mix((TEXT_PRIMARY),(TEXT_PRIMARY_DARK), self.dark_mode); }
                        }
                    }
                    <View> { width: Fill }
                    <PillButton> { text: "已收藏" }
                }
                line2 = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Right
                    align: {y: 0.5}
                    <Label> { text: "“Would you have any availability?”"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self)->vec4{ return mix((TEXT_PRIMARY),(TEXT_PRIMARY_DARK), self.dark_mode); }
                        }
                    }
                    <View> { width: Fill }
                    <SecondaryButton> { text: "收藏" }
                }
            }

            alert_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 8
                <SectionTitle> { text: "🧠 遗忘预警（提前防回退）" }
                panel = <PanelBase> {
                    width: Fill, height: Fit
                    padding: 12
                    flow: Down
                    spacing: 6
                    <Label> {
                        text: "这些已掌握卡片近期有回退风险："
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                    <MutedText> { text: "• availability（发音评分从 90% → 80%）" }
                    <MutedText> { text: "• articles（对话中出现 2 次误用）" }
                    actions = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        <View> { width: Fill }
                        <PrimaryButton> { text: "一键加入" }
                    }
                }
            }
        }
    }
}
