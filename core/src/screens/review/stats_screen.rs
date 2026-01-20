use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    use crate::screens::review::components::SectionTitle;
    use crate::screens::review::components::PanelBase;
    use crate::screens::review::components::StatOverviewItem;
    use crate::screens::review::components::ChartBar;

    pub StatsScreen = <View> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 24

        // Learning overview section
        overview_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 12

            <SectionTitle> { text: "学习概览" }

            overview_grid = <View> {
                width: Fill, height: Fit
                flow: Right
                spacing: 16

                col1 = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 16

                    item1 = <StatOverviewItem> {
                        item_label = { text: "总学习词汇" }
                        item_value = { text: "187" }
                    }
                    item3 = <StatOverviewItem> {
                        item_label = { text: "复习次数" }
                        item_value = { text: "456" }
                    }
                }

                col2 = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 16

                    item2 = <StatOverviewItem> {
                        item_label = { text: "学习天数" }
                        item_value = { text: "32" }
                    }
                    item4 = <StatOverviewItem> {
                        item_label = { text: "平均掌握度" }
                        item_value = { text: "78%" }
                    }
                }
            }
        }

        // Weekly review chart section
        chart_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 12

            <SectionTitle> { text: "本周复习" }

            chart_container = <PanelBase> {
                width: Fill, height: Fit
                padding: 20
                flow: Down
                spacing: 8

                // Bar chart
                bars = <View> {
                    width: Fill, height: 120
                    flow: Right
                    align: {y: 1.0}
                    spacing: 12

                    bar1 = <ChartBar> {
                        bar = { height: 72 }
                        day_label = { text: "一" }
                    }
                    bar2 = <ChartBar> {
                        bar = { height: 96 }
                        day_label = { text: "二" }
                    }
                    bar3 = <ChartBar> {
                        bar = { height: 54 }
                        day_label = { text: "三" }
                    }
                    bar4 = <ChartBar> {
                        bar = { height: 108 }
                        day_label = { text: "四" }
                    }
                    bar5 = <ChartBar> {
                        bar = { height: 84 }
                        day_label = { text: "五" }
                    }
                    bar6 = <ChartBar> {
                        bar = { height: 36 }
                        day_label = { text: "六" }
                    }
                    bar7 = <ChartBar> {
                        bar = { height: 60 }
                        day_label = { text: "日" }
                    }
                }
            }
        }
    }
}
