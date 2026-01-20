use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    use crate::screens::review::components::TipBanner;
    use crate::screens::review::components::WordCard;
    use crate::screens::review::components::TIP_GREEN_BG;
    use crate::screens::review::components::TIP_GREEN_TEXT;

    pub MasteredScreen = <View> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 16

        // Tip banner
        tip = <TipBanner> {
            draw_bg: { tint: (TIP_GREEN_BG) }
            tip_text = {
                text: "✅ 这些词汇你已经掌握得很好了，继续保持！"
                draw_text: { tint: (TIP_GREEN_TEXT) }
            }
        }

        // Word cards grid - 2 columns
        cards_grid = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 16

            col1 = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16

                card1 = <WordCard> {
                    header_row = {
                        word_label = { text: "schedule" }
                    }
                    hint_label = { text: "日程安排" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.95 } }
                        progress_label = { text: "95%" }
                    }
                }

                card3 = <WordCard> {
                    header_row = {
                        word_label = { text: "confirm" }
                    }
                    hint_label = { text: "确认" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.92 } }
                        progress_label = { text: "92%" }
                    }
                }

                card5 = <WordCard> {
                    header_row = {
                        word_label = { text: "available" }
                    }
                    hint_label = { text: "可用的，有空的" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.88 } }
                        progress_label = { text: "88%" }
                    }
                }
            }

            col2 = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16

                card2 = <WordCard> {
                    header_row = {
                        word_label = { text: "appointment" }
                    }
                    hint_label = { text: "预约" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.90 } }
                        progress_label = { text: "90%" }
                    }
                }

                card4 = <WordCard> {
                    header_row = {
                        word_label = { text: "cancel" }
                    }
                    hint_label = { text: "取消" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.94 } }
                        progress_label = { text: "94%" }
                    }
                }

                card6 = <WordCard> {
                    header_row = {
                        word_label = { text: "reschedule" }
                    }
                    hint_label = { text: "重新安排" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.86 } }
                        progress_label = { text: "86%" }
                    }
                }
            }
        }
    }
}
