use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    use crate::screens::review::components::TipBanner;
    use crate::screens::review::components::WordCard;
    use crate::screens::review::components::TIP_AMBER_BG;
    use crate::screens::review::components::TIP_AMBER_TEXT;

    pub MistakesScreen = <View> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 16

        // Tip banner
        tip = <TipBanner> {
            draw_bg: { tint: (TIP_AMBER_BG) }
            tip_text = {
                text: "🎯 这些是你经常出错的词汇，多加练习可以帮助你克服这些难点"
                draw_text: { tint: (TIP_AMBER_TEXT) }
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
                        word_label = { text: "affect vs effect" }
                    }
                    hint_label = { text: "affect (v.) 影响 / effect (n.) 效果" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.3 } }
                        progress_label = { text: "30%" }
                    }
                }

                card3 = <WordCard> {
                    header_row = {
                        word_label = { text: "complement vs compliment" }
                    }
                    hint_label = { text: "complement (补充) / compliment (赞美)" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.35 } }
                        progress_label = { text: "35%" }
                    }
                }
            }

            col2 = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16

                card2 = <WordCard> {
                    header_row = {
                        word_label = { text: "their vs there" }
                    }
                    hint_label = { text: "their (他们的) / there (那里)" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.4 } }
                        progress_label = { text: "40%" }
                    }
                }

                card4 = <WordCard> {
                    header_row = {
                        word_label = { text: "principal vs principle" }
                    }
                    hint_label = { text: "principal (主要的/校长) / principle (原则)" }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.25 } }
                        progress_label = { text: "25%" }
                    }
                }
            }
        }
    }
}
