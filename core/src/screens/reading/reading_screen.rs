//! Reading Practice Screen - 大声跟读 (Read Aloud)
//!
//! Features:
//! - Audio waveform comparison (side by side)
//! - Real-time pronunciation scoring
//! - Detailed feedback on pronunciation, fluency, and intonation
//! - Progress tracking

use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use colang_widgets::theme::*;

    // Orange accent colors
    ACCENT_ORANGE = #f97316

    // ========================================================================
    // Reading Practice Components
    // ========================================================================

    // Tips card for reading practice
    TipCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 8.0
            fn get_color(self) -> vec4 {
                return mix((SLATE_100), (SLATE_800), self.dark_mode);
            }
        }
        flow: Down
        spacing: 4

        tip_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        tip_desc = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    SentenceDisplay = <RoundedView> {
        width: Fill, height: Fit
        padding: 24
        flow: Down
        spacing: 12
        align: {x: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        <Label> {
            text: "今日练习"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                }
            }
        }

        sentence_en = <Label> {
            text: "Could you please help me with this?"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        sentence_zh = <Label> {
            text: "你能帮我一下吗？"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    WaveformCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 16
        flow: Down
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn get_color(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}
            spacing: 8

            icon_label = <Label> {
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                }
            }

            <View> { width: Fill }

            play_btn = <Button> {
                width: 32, height: 32
                text: "▶"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                    color: (WHITE)
                }
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.circle(16., 16., 16.);
                        sdf.fill((ACCENT_GREEN));
                        return sdf.result;
                    }
                }
            }
        }

        waveform_view = <RoundedView> {
            width: Fill, height: 100
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                border_radius: 8.0
                fn get_color(self) -> vec4 {
                    return mix((SLATE_100), (SLATE_900), self.dark_mode);
                }
            }
        }
    }

    ScoreCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 20
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
            text: "🧠 AI 评分与建议"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        score_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 20
            align: {y: 0.5}

            total_score = <View> {
                width: Fit, height: Fit
                flow: Down
                spacing: 8
                align: {x: 0.5}

                score_circle = <View> {
                    width: 80, height: 80
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.circle(40., 40., 40.);
                            sdf.fill(vec4(0.067, 0.725, 0.506, 0.15));
                            sdf.circle(40., 40., 40.);
                            sdf.stroke((ACCENT_GREEN), 4.0);
                            return sdf.result;
                        }
                    }
                    align: {x: 0.5, y: 0.5}

                    score_value = <Label> {
                        text: "85"
                        draw_text: {
                            text_style: <FONT_BOLD>{ font_size: 28.0 }
                            color: (ACCENT_GREEN)
                        }
                    }
                }

                score_label = <Label> {
                    text: "总分"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }

            detailed_scores = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 10

                pronunciation_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 10

                    <Label> {
                        width: 90
                        text: "发音准确度"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.9
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        width: 45
                        text: "90%"
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            color: (ACCENT_GREEN)
                        }
                    }
                }

                fluency_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 10

                    <Label> {
                        width: 90
                        text: "流畅度"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.8
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.984, 0.749, 0.141, 1.0));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        width: 45
                        text: "80%"
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            color: vec4(0.984, 0.749, 0.141, 1.0)
                        }
                    }
                }

                intonation_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {y: 0.5}
                    spacing: 10

                    <Label> {
                        width: 90
                        text: "语调"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_REGULAR>{ font_size: 12.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.85
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.12, 0.13, 0.15, 1.0));
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    <Label> {
                        width: 45
                        text: "85%"
                        draw_text: {
                            text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                            color: (ACCENT_GREEN)
                        }
                    }
                }
            }
        }

        feedback_text = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 10
            padding: {top: 8}

            feedback_title = <Label> {
                text: "详细反馈"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 8

                <Label> {
                    text: "⚠️ 需要注意: \"help\" 的发音稍重，注意轻读"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                        }
                    }
                }

                <Label> {
                    text: "✓ 做得好: \"Could you\" 的连读非常自然！"
                    draw_text: {
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        color: (ACCENT_GREEN)
                    }
                }
            }
        }
    }

    ActionButtons = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 12
        align: {x: 0.5}

        prev_btn = <Button> {
            width: 120, height: 48
            text: "⏮️ 上一句"
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
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let color = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
        }

        record_btn = <Button> {
            width: 120, height: 48
            text: "🎙️ 重录"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    sdf.fill(vec4(0.937, 0.267, 0.267, 1.0));
                    return sdf.result;
                }
            }
        }

        next_btn = <Button> {
            width: 120, height: 48
            text: "下一句 ⏭️"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    sdf.fill((ACCENT_GREEN));
                    return sdf.result;
                }
            }
        }
    }

    // ========================================================================
    // Main Reading Practice Screen
    // ========================================================================

    pub ReadingScreen = {{ReadingScreen}} {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                // Gradient: orange-50 (#fff7ed) → amber-50 (#fffbeb) → yellow-50 (#fefce8)
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0);
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);

                let t = self.pos.x;
                let light_color = vec4(0.0);
                if t < 0.5 {
                    light_color = mix(orange_50, amber_50, t * 2.0);
                } else {
                    light_color = mix(amber_50, yellow_50, (t - 0.5) * 2.0);
                }
                return mix(light_color, dark_bg, self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: {left: 40, right: 40, top: 24, bottom: 24}
                spacing: 16

                // Header Card (like website)
                header_card = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 20
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

                    // Title row
                    header = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 4

                        <Label> {
                            text: "🎤 大声跟读"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_BOLD>{ font_size: 20.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        <Label> {
                            text: "发音纠正 · 音波对比 · AI 智能评分"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Progress bar
                    progress_bar = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        align: {y: 0.5}
                        spacing: 12

                        <Label> {
                            text: "练习进度"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }

                        progress_track = <View> {
                            width: Fill, height: 8
                            show_bg: true
                            draw_bg: {
                                instance progress: 0.5
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                    sdf.fill(vec4(0.91, 0.92, 0.93, 1.0));
                                    sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                    sdf.fill(vec4(0.133, 0.773, 0.369, 1.0)); // green-500
                                    return sdf.result;
                                }
                            }
                        }

                        progress_label = <Label> {
                            text: "3/6 句"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }
                    }
                }

                // Sentence display
                sentence_display = <SentenceDisplay> {}

                // Waveforms side by side
                waveforms_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 16

                    native_waveform = <WaveformCard> {
                        header = {
                            icon_label = { text: "🔊 标准发音" }
                        }
                    }

                    user_waveform = <WaveformCard> {
                        header = {
                            icon_label = { text: "🎙️ 你的发音" }
                        }
                    }
                }

                // Score and feedback
                score_card = <ScoreCard> {}

                // Action buttons
                action_buttons = <ActionButtons> {}

                // Tips section (like website)
                tips_section = <RoundedView> {
                    width: Fill, height: Fit
                    padding: 20
                    flow: Down
                    spacing: 12
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 12.0
                        fn get_color(self) -> vec4 {
                            return mix((WHITE), (SLATE_800), self.dark_mode);
                        }
                    }

                    <Label> {
                        text: "💡 跟读技巧"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 15.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    tips_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 12

                        tip1 = <TipCard> {
                            tip_title = { text: "🎯 模仿语调" }
                            tip_desc = { text: "注意句子的升降调" }
                        }

                        tip2 = <TipCard> {
                            tip_title = { text: "🔗 注意连读" }
                            tip_desc = { text: "单词之间的自然衔接" }
                        }

                        tip3 = <TipCard> {
                            tip_title = { text: "⏱️ 控制节奏" }
                            tip_desc = { text: "不要太快或太慢" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ReadingScreen {
    #[deref]
    view: View,
}

impl Widget for ReadingScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReadingScreen {
    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }
}
