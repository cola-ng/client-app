use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    use crate::screens::review::components::WordCard;

    pub DueScreen = <View> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 16

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
                        word_label = { text: "accommodation" }
                        due_badge = {
                            visible: true
                            draw_bg: { urgent: 1.0 }
                            due_text = { text: "Now", draw_text: { urgent: 1.0 } }
                        }
                    }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.6 } }
                        progress_label = { text: "60%" }
                    }
                }

                card3 = <WordCard> {
                    header_row = {
                        word_label = { text: "reservation" }
                        due_badge = {
                            visible: true
                            draw_bg: { urgent: 0.0 }
                            due_text = { text: "2小时", draw_text: { urgent: 0.0 } }
                        }
                    }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.7 } }
                        progress_label = { text: "70%" }
                    }
                }

                card5 = <WordCard> {
                    header_row = {
                        word_label = { text: "schedule" }
                        due_badge = {
                            visible: true
                            draw_bg: { urgent: 0.0 }
                            due_text = { text: "4小时", draw_text: { urgent: 0.0 } }
                        }
                    }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.55 } }
                        progress_label = { text: "55%" }
                    }
                }
            }

            col2 = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16

                card2 = <WordCard> {
                    header_row = {
                        word_label = { text: "itinerary" }
                        due_badge = {
                            visible: true
                            draw_bg: { urgent: 1.0 }
                            due_text = { text: "Now", draw_text: { urgent: 1.0 } }
                        }
                    }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.45 } }
                        progress_label = { text: "45%" }
                    }
                }

                card4 = <WordCard> {
                    header_row = {
                        word_label = { text: "appointment" }
                        due_badge = {
                            visible: true
                            draw_bg: { urgent: 0.0 }
                            due_text = { text: "3小时", draw_text: { urgent: 0.0 } }
                        }
                    }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.65 } }
                        progress_label = { text: "65%" }
                    }
                }

                card6 = <WordCard> {
                    header_row = {
                        word_label = { text: "availability" }
                        due_badge = {
                            visible: true
                            draw_bg: { urgent: 0.0 }
                            due_text = { text: "明天", draw_text: { urgent: 0.0 } }
                        }
                    }
                    progress_row = {
                        progress_bar = { draw_bg: { progress: 0.5 } }
                        progress_label = { text: "50%" }
                    }
                }
            }
        }

        // Empty state (hidden by default)
        empty_state = <View> {
            visible: false
            width: Fill, height: Fit
            padding: 32
            align: {x: 0.5}
            <Label> {
                text: "太棒了！暂时没有需要复习的内容"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }
}
