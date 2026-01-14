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

    pub StatsScreen = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 16

        controls = <CardBase> {
            width: Fill, height: Fit
            padding: 16
            flow: Right
            align: {y: 0.5}
            <SectionTitle> { text: "📌 时间范围" }
            <PillButton> { text: "近 7 天" margin: {left: 10} }
            <PillButton> { text: "近 30 天" }
            <PillButton> { text: "本学期" }
            <View> { width: Fill }
            <SecondaryButton> { text: "导出" }
            <PrimaryButton> { text: "分享" margin: {left: 8} }
        }

        kpis = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12
            k1 = <CardBase> { width: Fill, height: Fit padding: 16 flow: Down spacing: 6
                <SectionTitle> { text: "🔥 连续学习" }
                <Label> { text: "12 天" draw_text: { text_style: <FONT_BOLD>{ font_size: 22.0 } color: (ACCENT_GREEN) } }
                <MutedText> { text: "今天已完成：8/23" }
            }
            k2 = <CardBase> { width: Fill, height: Fit padding: 16 flow: Down spacing: 6
                <SectionTitle> { text: "🃏 复习量" }
                <Label> { text: "156" draw_text: { text_style: <FONT_BOLD>{ font_size: 22.0 } color: (ACCENT_INDIGO) } }
                <MutedText> { text: "本月复习的词/句卡" }
            }
            k3 = <CardBase> { width: Fill, height: Fit padding: 16 flow: Down spacing: 6
                <SectionTitle> { text: "🎯 准确率" }
                <Label> { text: "86%" draw_text: { text_style: <FONT_BOLD>{ font_size: 22.0 } color: (TEXT_PRIMARY) } }
                <MutedText> { text: "更重要：敢开口次数 +3" }
            }
            k4 = <CardBase> { width: Fill, height: Fit padding: 16 flow: Down spacing: 6
                <SectionTitle> { text: "🗣️ 开口时长" }
                <Label> { text: "47 min" draw_text: { text_style: <FONT_BOLD>{ font_size: 22.0 } color: (TEXT_PRIMARY) } }
                <MutedText> { text: "本周 交流对话/跟读合计" }
            }
        }

        mid = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 16

            trend_card = <CardBase> {
                width: Fill, height: Fit
                padding: 16
                flow: Down
                spacing: 8
                <SectionTitle> { text: "📈 趋势（坚持与效果）" }
                <MutedText> { text: "只看趋势，不制造焦虑：复习完成数与准确率" }
                <PanelBase> {
                    width: Fill, height: 170
                    padding: 12
                    <MutedText> { text: "（这里是趋势图占位，后续可接真实数据）" }
                }
            }

            breakdown_card = <CardBase> {
                width: 420, height: Fit
                padding: 16
                flow: Down
                spacing: 8
                <SectionTitle> { text: "🧩 易错点分布" }
                <MutedText> { text: "把时间花在最划算的薄弱点上" }
                row1 = <PanelBase> { width: Fill height: Fit padding: 12 flow: Right align: {y: 0.5}
                    <Label> { text: "语法（冠词/时态）"
                        draw_text: { instance dark_mode: 0.0 text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self)->vec4{ return mix((TEXT_PRIMARY),(TEXT_PRIMARY_DARK), self.dark_mode); }
                        }
                    }
                    <View> { width: Fill }
                    <MutedText> { text: "42%" }
                }
                row2 = <PanelBase> { width: Fill height: Fit padding: 12 flow: Right align: {y: 0.5}
                    <Label> { text: "发音（重音/连读）"
                        draw_text: { instance dark_mode: 0.0 text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self)->vec4{ return mix((TEXT_PRIMARY),(TEXT_PRIMARY_DARK), self.dark_mode); }
                        }
                    }
                    <View> { width: Fill }
                    <MutedText> { text: "31%" }
                }
                row3 = <PanelBase> { width: Fill height: Fit padding: 12 flow: Right align: {y: 0.5}
                    <Label> { text: "用词（搭配/语境）"
                        draw_text: { instance dark_mode: 0.0 text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                            fn get_color(self)->vec4{ return mix((TEXT_PRIMARY),(TEXT_PRIMARY_DARK), self.dark_mode); }
                        }
                    }
                    <View> { width: Fill }
                    <MutedText> { text: "27%" }
                }
            }
        }

        next = <CardBase> {
            width: Fill, height: Fit
            padding: 16
            flow: Down
            spacing: 10
            <SectionTitle> { text: "🧠 下一步（自动生成，不让用户纠结）" }
            panel = <PanelBase> {
                width: Fill, height: Fit
                padding: 12
                flow: Down
                spacing: 6
                <Label> {
                    text: "今天最划算的 3 件事："
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }
                <Label> { text: "1) 复习到期 23 张（预计 6 分钟）" draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 } color: (ACCENT_INDIGO) } }
                <Label> { text: "2) 专项：冠词 a/the（15 秒微练 + 带入对话）" draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 } color: (ACCENT_INDIGO) } }
                <Label> { text: "3) 发音：reservation 重音（影子跟读 2 轮）" draw_text: { text_style: <FONT_MEDIUM>{ font_size: 12.0 } color: (ACCENT_INDIGO) } }
                actions = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    <View> { width: Fill }
                    <PrimaryButton> { text: "一键加入今日任务" }
                    <SecondaryButton> { text: "安排到明天" margin: {left: 6} }
                }
            }
        }
    }
}
