//! Reading Practice Screen - 大声跟读 (Read Aloud)
//!
//! Features:
//! - Exercise selection with tabs
//! - Audio waveform comparison (side by side)
//! - Real-time pronunciation scoring
//! - Detailed feedback on pronunciation, fluency, and intonation
//! - Progress tracking

use std::sync::mpsc;

use makepad_widgets::*;
use makepad_component::*;

use crate::asset_api::{get_asset_api, ReadingExercise, ReadingSentence};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use colang_widgets::theme::*;

    // Orange accent colors
    ACCENT_ORANGE = #f97316
    ACCENT_ORANGE_HOVER = #ea580c

    // ========================================================================
    // Reading Practice Components
    // ========================================================================

    // Exercise tab button
    ExerciseTab = <RoundedView> {
        width: Fit, height: 32
        padding: {left: 12, right: 12}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance dark_mode: 0.0
            border_radius: 8.0
            fn pixel(self) -> vec4 {
                let orange = vec4(0.976, 0.451, 0.086, 1.0); // #f97316
                let light_bg = vec4(0.95, 0.96, 0.97, 1.0);  // gray-100
                let dark_bg = vec4(0.2, 0.22, 0.25, 1.0);
                let unselected = mix(light_bg, dark_bg, self.dark_mode);
                return mix(unselected, orange, self.selected);
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand

        tab_label = <Label> {
            draw_text: {
                instance selected: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    let white = vec4(1.0, 1.0, 1.0, 1.0);
                    let gray = vec4(0.392, 0.455, 0.545, 1.0);
                    return mix(gray, white, self.selected);
                }
            }
        }
    }

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

        exercise_title = <Label> {
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
            text: "点击开始录音"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        sentence_zh = <Label> {
            text: "请选择一个练习开始"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }

        sentence_tips = <Label> {
            visible: false
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: (ACCENT_ORANGE)
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
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
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
            width: Fill, height: 80
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance has_audio: 0.0
                instance is_native: 0.0
                border_radius: 8.0
                fn pixel(self) -> vec4 {
                    let bg = mix((SLATE_100), (SLATE_900), self.dark_mode);

                    // Simple waveform visualization placeholder
                    if self.has_audio > 0.5 {
                        let wave_color = mix(
                            vec4(0.976, 0.451, 0.086, 1.0),  // orange for user
                            vec4(0.133, 0.773, 0.369, 1.0), // green for native
                            self.is_native
                        );
                        let x = self.pos.x;
                        let y = self.pos.y;
                        let center_y = 0.5;
                        let amplitude = 0.3 * sin(x * 50.0) * sin(x * 23.0 + 1.5);
                        let dist = abs(y - center_y - amplitude);
                        if dist < 0.05 {
                            return wave_color;
                        }
                    }
                    return bg;
                }
            }
            align: {x: 0.5, y: 0.5}

            placeholder_text = <Label> {
                text: "点击录音开始"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                    }
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

                    pronunciation_bar = <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.9
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let bg_color = mix(vec4(0.91, 0.92, 0.93, 1.0), vec4(0.2, 0.22, 0.25, 1.0), self.dark_mode);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(bg_color);
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    pronunciation_score = <Label> {
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

                    fluency_bar = <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.8
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let bg_color = mix(vec4(0.91, 0.92, 0.93, 1.0), vec4(0.2, 0.22, 0.25, 1.0), self.dark_mode);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(bg_color);
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill(vec4(0.984, 0.749, 0.141, 1.0));
                                return sdf.result;
                            }
                        }
                    }

                    fluency_score = <Label> {
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

                    intonation_bar = <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: {
                            instance progress: 0.85
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let bg_color = mix(vec4(0.91, 0.92, 0.93, 1.0), vec4(0.2, 0.22, 0.25, 1.0), self.dark_mode);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                sdf.fill(bg_color);
                                sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                sdf.fill((ACCENT_GREEN));
                                return sdf.result;
                            }
                        }
                    }

                    intonation_score = <Label> {
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

        feedback_section = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8
            padding: {top: 8}

            warning_feedback = <Label> {
                text: "⚠️ 需要注意: 注意单词之间的连读和停顿"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            success_feedback = <Label> {
                text: "✓ 做得好: 发音清晰，语速适中！"
                draw_text: {
                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                    color: (ACCENT_GREEN)
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
            text: "⏮ 上一句"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
            draw_bg: {
                instance dark_mode: 0.0
                instance disabled: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let enabled_color = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    let disabled_color = mix((SLATE_200), (SLATE_800), self.dark_mode);
                    let color = mix(enabled_color, disabled_color, self.disabled);
                    sdf.fill(color);
                    return sdf.result;
                }
            }
        }

        record_btn = <Button> {
            width: 140, height: 48
            text: "🎙 开始录音"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
            draw_bg: {
                instance recording: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let normal = vec4(0.976, 0.451, 0.086, 1.0); // orange
                    let recording_color = vec4(0.937, 0.267, 0.267, 1.0); // red
                    sdf.fill(mix(normal, recording_color, self.recording));
                    return sdf.result;
                }
            }
        }

        next_btn = <Button> {
            width: 120, height: 48
            text: "下一句 ⏭"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
            draw_bg: {
                instance disabled: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let enabled = vec4(0.133, 0.773, 0.369, 1.0); // green
                    let disabled_color = vec4(0.6, 0.65, 0.68, 1.0);
                    sdf.fill(mix(enabled, disabled_color, self.disabled));
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

                // Header Card
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
                        flow: Right
                        spacing: 12
                        align: {y: 0.5}

                        <Label> {
                            text: "🎤 跟读练习"
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

                    // Exercise selector tabs
                    exercise_tabs = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 8

                        // Loading indicator
                        loading_label = <Label> {
                            text: "加载练习中..."
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }

                        // Dynamic tabs will be added here
                        tab0 = <ExerciseTab> { visible: false, tab_label = { text: "练习1" } }
                        tab1 = <ExerciseTab> { visible: false, tab_label = { text: "练习2" } }
                        tab2 = <ExerciseTab> { visible: false, tab_label = { text: "练习3" } }
                        tab3 = <ExerciseTab> { visible: false, tab_label = { text: "练习4" } }
                        tab4 = <ExerciseTab> { visible: false, tab_label = { text: "练习5" } }
                    }

                    // Progress bar
                    progress_section = <View> {
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
                                instance progress: 0.0
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    let bg_color = mix(vec4(0.91, 0.92, 0.93, 1.0), vec4(0.2, 0.22, 0.25, 1.0), self.dark_mode);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                                    sdf.fill(bg_color);
                                    sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 4.0);
                                    sdf.fill(vec4(0.133, 0.773, 0.369, 1.0)); // green-500
                                    return sdf.result;
                                }
                            }
                        }

                        progress_label = <Label> {
                            text: "0/0 句"
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
                        waveform_view = {
                            draw_bg: { is_native: 1.0, has_audio: 1.0 }
                            placeholder_text = { visible: false }
                        }
                    }

                    user_waveform = <WaveformCard> {
                        header = {
                            icon_label = { text: "🎙 你的发音" }
                        }
                        waveform_view = {
                            draw_bg: { is_native: 0.0, has_audio: 0.0 }
                        }
                    }
                }

                // Score and feedback (hidden until recorded)
                score_card = <ScoreCard> {
                    visible: false
                }

                // Action buttons
                action_buttons = <ActionButtons> {}

                // Tips section
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
                            tip_title = { text: "⏱ 控制节奏" }
                            tip_desc = { text: "不要太快或太慢" }
                        }
                    }
                }
            }
        }
    }
}

/// Data fetch result types
enum FetchResult {
    Exercises(Result<Vec<ReadingExercise>, String>),
    Sentences(Result<Vec<ReadingSentence>, String>),
}

#[derive(Live, LiveHook, Widget)]
pub struct ReadingScreen {
    #[deref]
    view: View,

    /// All available exercises
    #[rust]
    exercises: Vec<ReadingExercise>,

    /// Sentences for current exercise
    #[rust]
    sentences: Vec<ReadingSentence>,

    /// Currently selected exercise index
    #[rust]
    selected_exercise_index: usize,

    /// Current sentence index
    #[rust]
    current_sentence_index: usize,

    /// Whether exercises are being loaded
    #[rust]
    exercises_loading: bool,

    /// Whether sentences are being loaded
    #[rust]
    sentences_loading: bool,

    /// Whether data has been loaded initially
    #[rust]
    data_loaded: bool,

    /// Whether currently recording
    #[rust]
    is_recording: bool,

    /// Whether user has recorded for current sentence
    #[rust]
    has_recorded: bool,

    /// Channel to receive fetch results
    #[rust]
    fetch_rx: Option<mpsc::Receiver<FetchResult>>,
}

impl Widget for ReadingScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        // Get commonly used parent views
        let header_card = self.view.view(ids!(header_card));
        let exercise_tabs = header_card.view(ids!(exercise_tabs));
        let action_buttons = self.view.view(ids!(action_buttons));
        let waveforms_row = self.view.view(ids!(waveforms_row));

        // Handle exercise tab clicks
        let tab_views = [
            exercise_tabs.view(ids!(tab0)),
            exercise_tabs.view(ids!(tab1)),
            exercise_tabs.view(ids!(tab2)),
            exercise_tabs.view(ids!(tab3)),
            exercise_tabs.view(ids!(tab4)),
        ];

        for (i, tab) in tab_views.iter().enumerate() {
            if tab.finger_up(&actions).is_some() {
                if i < self.exercises.len() && i != self.selected_exercise_index {
                    self.selected_exercise_index = i;
                    self.update_exercise_tabs(cx);
                    self.load_sentences(cx);
                    self.view.redraw(cx);
                }
            }
        }

        // Handle prev button click
        if action_buttons.button(ids!(prev_btn)).clicked(&actions) {
            if self.current_sentence_index > 0 {
                self.current_sentence_index -= 1;
                self.has_recorded = false;
                self.update_sentence_display(cx);
                self.update_button_states(cx);
                self.view.redraw(cx);
            }
        }

        // Handle next button click
        if action_buttons.button(ids!(next_btn)).clicked(&actions) {
            if self.current_sentence_index < self.sentences.len().saturating_sub(1) {
                self.current_sentence_index += 1;
                self.has_recorded = false;
                self.update_sentence_display(cx);
                self.update_button_states(cx);
                self.view.redraw(cx);
            }
        }

        // Handle record button click
        if action_buttons.button(ids!(record_btn)).clicked(&actions) {
            self.is_recording = !self.is_recording;
            if !self.is_recording {
                // Stopped recording, show score
                self.has_recorded = true;
                self.show_score_card(cx);
            }
            self.update_record_button(cx);
            self.update_user_waveform(cx);
            self.view.redraw(cx);
        }

        // Handle native audio play button
        let native_waveform = waveforms_row.view(ids!(native_waveform));
        if native_waveform.view(ids!(header)).button(ids!(play_btn)).clicked(&actions) {
            // TODO: Play native audio
        }

        // Handle user audio play button
        let user_waveform = waveforms_row.view(ids!(user_waveform));
        if user_waveform.view(ids!(header)).button(ids!(play_btn)).clicked(&actions) {
            // TODO: Play user recorded audio
        }

        // Process fetch results
        let mut exercises_result: Option<Result<Vec<ReadingExercise>, String>> = None;
        let mut sentences_result: Option<Result<Vec<ReadingSentence>, String>> = None;

        if let Some(rx) = &self.fetch_rx {
            while let Ok(result) = rx.try_recv() {
                match result {
                    FetchResult::Exercises(r) => exercises_result = Some(r),
                    FetchResult::Sentences(r) => sentences_result = Some(r),
                }
            }
        }

        // Handle exercises fetch result
        if let Some(result) = exercises_result {
            match result {
                Ok(exercises) => {
                    self.exercises = exercises;
                    self.exercises_loading = false;
                    exercise_tabs.label(ids!(loading_label)).set_visible(cx, false);
                    self.update_exercise_tabs(cx);
                    // Auto-select first exercise and load its sentences
                    if !self.exercises.is_empty() {
                        self.selected_exercise_index = 0;
                        self.load_sentences(cx);
                    }
                    self.view.redraw(cx);
                }
                Err(e) => {
                    eprintln!("Failed to fetch exercises: {}", e);
                    self.exercises_loading = false;
                    exercise_tabs.label(ids!(loading_label))
                        .set_text(cx, &format!("加载失败: {}", e));
                }
            }
        }

        // Handle sentences fetch result
        if let Some(result) = sentences_result {
            match result {
                Ok(sentences) => {
                    self.sentences = sentences;
                    self.sentences_loading = false;
                    self.current_sentence_index = 0;
                    self.has_recorded = false;
                    self.update_sentence_display(cx);
                    self.update_progress(cx);
                    self.update_button_states(cx);
                    self.view.view(ids!(score_card)).set_visible(cx, false);
                    self.view.redraw(cx);
                }
                Err(e) => {
                    eprintln!("Failed to fetch sentences: {}", e);
                    self.sentences_loading = false;
                }
            }
        }

        // Trigger initial data load on first draw
        if let Event::Draw(_) = event {
            if !self.data_loaded {
                self.data_loaded = true;
                self.load_exercises(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ReadingScreen {
    /// Load exercises from API
    fn load_exercises(&mut self, cx: &mut Cx) {
        self.exercises_loading = true;

        // Show loading - use parent chain
        let header_card = self.view.view(ids!(header_card));
        header_card.view(ids!(exercise_tabs)).label(ids!(loading_label))
            .set_visible(cx, true);

        let (tx, rx) = mpsc::channel();
        self.fetch_rx = Some(rx);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(api) = get_asset_api() {
                    if let Ok(client) = api.read() {
                        let result = client.list_reading_exercises(None, None, Some(5)).await;
                        let _ = tx.send(FetchResult::Exercises(result));
                    }
                }
            });
        });

        self.view.redraw(cx);
    }

    /// Load sentences for current exercise
    fn load_sentences(&mut self, cx: &mut Cx) {
        if self.selected_exercise_index >= self.exercises.len() {
            return;
        }

        self.sentences_loading = true;
        let exercise_id = self.exercises[self.selected_exercise_index].id;

        // Create new channel if needed
        let (tx, rx) = mpsc::channel();
        self.fetch_rx = Some(rx);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                if let Some(api) = get_asset_api() {
                    if let Ok(client) = api.read() {
                        let result = client.get_reading_sentences(exercise_id).await;
                        let _ = tx.send(FetchResult::Sentences(result));
                    }
                }
            });
        });
    }

    /// Update exercise tab visibility and selection state
    fn update_exercise_tabs(&mut self, cx: &mut Cx) {
        // Get exercise_tabs through parent chain
        let header_card = self.view.view(ids!(header_card));
        let exercise_tabs = header_card.view(ids!(exercise_tabs));

        let tabs = [
            exercise_tabs.view(ids!(tab0)),
            exercise_tabs.view(ids!(tab1)),
            exercise_tabs.view(ids!(tab2)),
            exercise_tabs.view(ids!(tab3)),
            exercise_tabs.view(ids!(tab4)),
        ];

        for (i, tab) in tabs.iter().enumerate() {
            if i < self.exercises.len() {
                let exercise = &self.exercises[i];
                tab.set_visible(cx, true);

                // Set label text
                tab.label(ids!(tab_label)).set_text(cx, &exercise.title_zh);

                // Update selected state
                let is_selected = i == self.selected_exercise_index;
                let selected_val = if is_selected { 1.0f64 } else { 0.0f64 };

                tab.apply_over(cx, live! {
                    draw_bg: { selected: (selected_val) }
                });
                tab.label(ids!(tab_label)).apply_over(cx, live! {
                    draw_text: { selected: (selected_val) }
                });
            } else {
                tab.set_visible(cx, false);
            }
        }
    }

    /// Update sentence display with current sentence
    fn update_sentence_display(&mut self, cx: &mut Cx) {
        let display = self.view.view(ids!(sentence_display));

        if let Some(sentence) = self.sentences.get(self.current_sentence_index) {
            // Get exercise title
            let exercise_title_text = self.exercises
                .get(self.selected_exercise_index)
                .map(|e| e.title_zh.as_str())
                .unwrap_or("今日练习");

            display.label(ids!(exercise_title)).set_text(cx, exercise_title_text);
            display.label(ids!(sentence_en)).set_text(cx, &sentence.content_en);
            display.label(ids!(sentence_zh)).set_text(cx, &sentence.content_zh);

            // Show tips if available from phonetic transcription
            if let Some(phonetic) = &sentence.phonetic_transcription {
                if !phonetic.is_empty() {
                    display.label(ids!(sentence_tips)).set_text(cx, &format!("💡 {}", phonetic));
                    display.label(ids!(sentence_tips)).set_visible(cx, true);
                } else {
                    display.label(ids!(sentence_tips)).set_visible(cx, false);
                }
            } else {
                display.label(ids!(sentence_tips)).set_visible(cx, false);
            }
        } else {
            display.label(ids!(exercise_title)).set_text(cx, "今日练习");
            display.label(ids!(sentence_en)).set_text(cx, "暂无练习内容");
            display.label(ids!(sentence_zh)).set_text(cx, "请选择其他练习");
            display.label(ids!(sentence_tips)).set_visible(cx, false);
        }
    }

    /// Update progress bar and label
    fn update_progress(&mut self, cx: &mut Cx) {
        let total = self.sentences.len();
        let current = if total > 0 { self.current_sentence_index + 1 } else { 0 };
        let progress = if total > 0 {
            current as f64 / total as f64
        } else {
            0.0
        };

        // Get progress track through parent chain
        let header_card = self.view.view(ids!(header_card));
        let progress_section = header_card.view(ids!(progress_section));
        progress_section.view(ids!(progress_track))
            .apply_over(cx, live! {
                draw_bg: { progress: (progress) }
            });

        progress_section.label(ids!(progress_label))
            .set_text(cx, &format!("{}/{} 句", current, total));
    }

    /// Update button enabled/disabled states
    fn update_button_states(&mut self, cx: &mut Cx) {
        let can_prev = self.current_sentence_index > 0;
        let can_next = self.current_sentence_index < self.sentences.len().saturating_sub(1);

        // Get action buttons through parent
        let action_buttons = self.view.view(ids!(action_buttons));

        // Update prev button
        let prev_disabled = if can_prev { 0.0f64 } else { 1.0f64 };
        action_buttons.button(ids!(prev_btn)).apply_over(cx, live! {
            draw_bg: { disabled: (prev_disabled) }
        });

        // Update next button
        let next_disabled = if can_next { 0.0f64 } else { 1.0f64 };
        action_buttons.button(ids!(next_btn)).apply_over(cx, live! {
            draw_bg: { disabled: (next_disabled) }
        });

        self.update_progress(cx);
    }

    /// Update record button appearance
    fn update_record_button(&mut self, cx: &mut Cx) {
        let recording_val = if self.is_recording { 1.0f64 } else { 0.0f64 };

        let text = if self.is_recording {
            "⏹ 停止录音"
        } else if self.has_recorded {
            "🎙 重新录音"
        } else {
            "🎙 开始录音"
        };

        // Get record button through parent
        let action_buttons = self.view.view(ids!(action_buttons));
        let record_btn = action_buttons.button(ids!(record_btn));
        record_btn.set_text(cx, text);
        record_btn.apply_over(cx, live! {
            draw_bg: { recording: (recording_val) }
        });
    }

    /// Update user waveform display
    fn update_user_waveform(&mut self, cx: &mut Cx) {
        let has_audio = if self.has_recorded || self.is_recording { 1.0f64 } else { 0.0f64 };

        // Get waveform view through parent chain
        let waveforms_row = self.view.view(ids!(waveforms_row));
        let user_waveform = waveforms_row.view(ids!(user_waveform));
        let waveform_view = user_waveform.view(ids!(waveform_view));

        waveform_view.apply_over(cx, live! {
            draw_bg: { has_audio: (has_audio) }
        });

        // Hide/show placeholder text
        let show_placeholder = !self.has_recorded && !self.is_recording;
        waveform_view.label(ids!(placeholder_text))
            .set_visible(cx, show_placeholder);
    }

    /// Show score card with mock scores
    fn show_score_card(&mut self, cx: &mut Cx) {
        let score_card = self.view.view(ids!(score_card));
        score_card.set_visible(cx, true);

        // Generate mock scores (in real app, this would come from AI evaluation)
        let pronunciation = 85 + (self.current_sentence_index % 10) as i32;
        let fluency = 75 + ((self.current_sentence_index * 3) % 15) as i32;
        let intonation = 80 + ((self.current_sentence_index * 7) % 12) as i32;
        let total = (pronunciation + fluency + intonation) / 3;

        // Get nested views through parent chain
        let score_row = score_card.view(ids!(score_row));
        let total_score = score_row.view(ids!(total_score));
        let detailed_scores = score_row.view(ids!(detailed_scores));

        // Update total score
        total_score.view(ids!(score_circle)).label(ids!(score_value))
            .set_text(cx, &format!("{}", total));

        // Update pronunciation score
        let pronunciation_row = detailed_scores.view(ids!(pronunciation_row));
        pronunciation_row.label(ids!(pronunciation_score))
            .set_text(cx, &format!("{}%", pronunciation));
        pronunciation_row.view(ids!(pronunciation_bar))
            .apply_over(cx, live! { draw_bg: { progress: (pronunciation as f64 / 100.0) } });

        // Update fluency score
        let fluency_row = detailed_scores.view(ids!(fluency_row));
        fluency_row.label(ids!(fluency_score))
            .set_text(cx, &format!("{}%", fluency));
        fluency_row.view(ids!(fluency_bar))
            .apply_over(cx, live! { draw_bg: { progress: (fluency as f64 / 100.0) } });

        // Update intonation score
        let intonation_row = detailed_scores.view(ids!(intonation_row));
        intonation_row.label(ids!(intonation_score))
            .set_text(cx, &format!("{}%", intonation));
        intonation_row.view(ids!(intonation_bar))
            .apply_over(cx, live! { draw_bg: { progress: (intonation as f64 / 100.0) } });
    }

    pub fn update_dark_mode(&mut self, cx: &mut Cx, dark_mode: f64) {
        self.view.apply_over(
            cx,
            live! {
                draw_bg: { dark_mode: (dark_mode) }
            },
        );
    }
}

impl ReadingScreenRef {
    /// Refresh data from API
    pub fn refresh_data(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.data_loaded = false;
            inner.load_exercises(cx);
        }
    }
}
