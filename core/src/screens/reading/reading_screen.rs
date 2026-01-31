//! Reading Practice Screen - 大声跟读 (Read Aloud)
//!
//! Features aligned with website design:
//! - Subject/topic selection with pill buttons
//! - User level display (Lv.X)
//! - Compact audio controls (原声, 录音, 回放, 下一句)
//! - AI pronunciation evaluation
//! - Inline score display with colored bars
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
    // Reading Practice Components - Website-aligned design
    // ========================================================================

    // Subject pill button - matches website's rounded-full style
    SubjectPill = <View> {
        width: Fit, height: 32
        padding: {left: 12, right: 12}
        show_bg: true
        draw_bg: {
            instance selected: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let orange = vec4(0.976, 0.451, 0.086, 1.0); // orange-500
                let light_bg = vec4(1.0, 1.0, 1.0, 1.0);  // white
                let light_border = vec4(0.9, 0.91, 0.92, 1.0); // gray-200
                let dark_bg = vec4(0.2, 0.22, 0.25, 1.0);
                let unselected = mix(light_bg, dark_bg, self.dark_mode);
                let color = mix(unselected, orange, self.selected);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 16.0); // rounded-full
                sdf.fill(color);
                // Border for unselected state
                if self.selected < 0.5 && self.dark_mode < 0.5 {
                    sdf.stroke(light_border, 1.0);
                }
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}
        cursor: Hand

        pill_label = <Label> {
            draw_text: {
                instance selected: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    let white = vec4(1.0, 1.0, 1.0, 1.0);
                    let gray = vec4(0.4, 0.45, 0.5, 1.0); // gray-600
                    return mix(gray, white, self.selected);
                }
            }
        }
    }

    // User level badge - matches website's Lv.X style
    LevelBadge = <View> {
        width: Fit, height: 20
        padding: {left: 6, right: 6}
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let orange_100 = vec4(1.0, 0.933, 0.886, 1.0); // orange-100
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                sdf.fill(orange_100);
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}

        level_text = <Label> {
            draw_text: {
                text_style: <FONT_SEMIBOLD>{ font_size: 10.0 }
                color: #ea580c // orange-600
            }
        }
    }

    // Compact audio control button
    AudioButton = <View> {
        width: Fit, height: 32
        padding: {left: 12, right: 12}
        show_bg: true
        cursor: Hand
        align: {x: 0.5, y: 0.5}
        draw_bg: {
            instance is_primary: 0.0
            instance recording: 0.0
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let orange = vec4(0.976, 0.451, 0.086, 1.0);
                let red = vec4(0.937, 0.267, 0.267, 1.0);
                let outline_light = vec4(0.9, 0.91, 0.92, 1.0);
                let outline_dark = vec4(0.3, 0.32, 0.35, 1.0);

                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                if self.is_primary > 0.5 {
                    // Primary (record) button
                    let color = mix(orange, red, self.recording);
                    sdf.fill(color);
                } else {
                    // Outline button
                    let bg = mix(vec4(1., 1., 1., 1.), vec4(0.15, 0.16, 0.18, 1.), self.dark_mode);
                    sdf.fill(bg);
                    sdf.stroke(mix(outline_light, outline_dark, self.dark_mode), 1.0);
                }
                return sdf.result;
            }
        }

        btn_label = <Label> {
            draw_text: {
                instance is_primary: 0.0
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    if self.is_primary > 0.5 {
                        return vec4(1., 1., 1., 1.);
                    }
                    return mix(vec4(0.25, 0.28, 0.3, 1.), vec4(0.9, 0.91, 0.92, 1.), self.dark_mode);
                }
            }
        }
    }

    // Score bar component
    ScoreBar = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 8

        bar_label = <Label> {
            width: 50
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix(vec4(0.4, 0.45, 0.5, 1.), vec4(0.65, 0.68, 0.7, 1.), self.dark_mode);
                }
            }
        }

        bar_track = <View> {
            width: Fill, height: 6
            show_bg: true
            draw_bg: {
                instance progress: 0.0
                instance score_color: 0.0 // 0=red, 0.5=amber, 1.0=green
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let bg = mix(vec4(0.9, 0.91, 0.92, 1.), vec4(0.2, 0.22, 0.25, 1.), self.dark_mode);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                    sdf.fill(bg);
                    // Progress fill with color based on score
                    let red = vec4(0.937, 0.267, 0.267, 1.0);
                    let amber = vec4(0.984, 0.749, 0.141, 1.0);
                    let green = vec4(0.133, 0.773, 0.369, 1.0);
                    let fill_color = vec4(0.);
                    if self.score_color < 0.33 {
                        fill_color = red;
                    } else if self.score_color < 0.67 {
                        fill_color = amber;
                    } else {
                        fill_color = green;
                    }
                    sdf.box(0., 0., self.rect_size.x * self.progress, self.rect_size.y, 3.0);
                    sdf.fill(fill_color);
                    return sdf.result;
                }
            }
        }

        bar_value = <Label> {
            width: 32
            draw_text: {
                instance score_color: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    let red = vec4(0.862, 0.196, 0.196, 1.0);
                    let amber = vec4(0.854, 0.647, 0.125, 1.0);
                    let green = vec4(0.133, 0.545, 0.329, 1.0);
                    if self.score_color < 0.33 {
                        return red;
                    } else if self.score_color < 0.67 {
                        return amber;
                    }
                    return green;
                }
            }
        }
    }

    // Legacy ExerciseTab kept for compatibility
    ExerciseTab = <SubjectPill> {
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

    // Compact tips row - matches website's inline tips style
    TipsRow = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        spacing: 8
        padding: 12

        <Label> {
            text: "💡 技巧:"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix(vec4(0.65, 0.68, 0.7, 1.), vec4(0.5, 0.52, 0.55, 1.), self.dark_mode);
                }
            }
        }

        <Label> {
            text: "模仿语调"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix(vec4(0.4, 0.45, 0.5, 1.), vec4(0.7, 0.72, 0.75, 1.), self.dark_mode);
                }
            }
        }

        <Label> {
            text: "·"
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: #d1d5db
            }
        }

        <Label> {
            text: "注意连读"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix(vec4(0.4, 0.45, 0.5, 1.), vec4(0.7, 0.72, 0.75, 1.), self.dark_mode);
                }
            }
        }

        <Label> {
            text: "·"
            draw_text: {
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                color: #d1d5db
            }
        }

        <Label> {
            text: "控制节奏"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                fn get_color(self) -> vec4 {
                    return mix(vec4(0.4, 0.45, 0.5, 1.), vec4(0.7, 0.72, 0.75, 1.), self.dark_mode);
                }
            }
        }
    }

    // Compact score circle - matches website's smaller score display
    ScoreCircle = <View> {
        width: 56, height: 56
        show_bg: true
        draw_bg: {
            instance score: 0.85
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let center = self.rect_size * 0.5;
                let radius = min(center.x, center.y) - 2.;
                // Color based on score
                let red = vec4(0.937, 0.267, 0.267, 1.0);
                let amber = vec4(0.984, 0.749, 0.141, 1.0);
                let green = vec4(0.133, 0.773, 0.369, 1.0);
                let color = vec4(0.);
                if self.score < 0.6 {
                    color = red;
                } else if self.score < 0.8 {
                    color = amber;
                } else {
                    color = green;
                }
                // Background
                let bg = mix(color, vec4(1., 1., 1., 1.), 0.9);
                sdf.circle(center.x, center.y, radius);
                sdf.fill(bg);
                // Border ring
                sdf.circle(center.x, center.y, radius);
                sdf.stroke(color, 3.0);
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.5}

        score_text = <Label> {
            draw_text: {
                instance score: 0.85
                text_style: <FONT_BOLD>{ font_size: 20.0 }
                fn get_color(self) -> vec4 {
                    let red = vec4(0.862, 0.196, 0.196, 1.0);
                    let amber = vec4(0.854, 0.647, 0.125, 1.0);
                    let green = vec4(0.133, 0.545, 0.329, 1.0);
                    if self.score < 0.6 {
                        return red;
                    } else if self.score < 0.8 {
                        return amber;
                    }
                    return green;
                }
            }
        }
    }

    // Compact action buttons - website uses inline button row
    CompactActionButtons = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 12
        align: {x: 0.5}

        // Standard audio button
        standard_btn = <AudioButton> {
            draw_bg: { is_primary: 0.0 }
            btn_label = { text: "🔊 原声", draw_text: { is_primary: 0.0 } }
        }

        // Record button (primary)
        record_btn = <AudioButton> {
            width: Fit, height: 40
            padding: {left: 20, right: 20}
            draw_bg: { is_primary: 1.0 }
            btn_label = {
                text: "🎙 录音"
                draw_text: { is_primary: 1.0, text_style: <FONT_SEMIBOLD>{ font_size: 14.0 } }
            }
        }

        // Playback button
        playback_btn = <AudioButton> {
            draw_bg: { is_primary: 0.0 }
            btn_label = { text: "▶ 回放", draw_text: { is_primary: 0.0 } }
        }

        // Next button
        next_btn = <AudioButton> {
            draw_bg: { is_primary: 0.0 }
            btn_label = { text: "下一句 ▶", draw_text: { is_primary: 0.0 } }
        }
    }

    // Legacy action buttons kept for compatibility
    ActionButtons = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 12
        align: {x: 0.5}

        prev_btn = <View> {
            width: 120, height: 48
            visible: false
        }
        record_btn = <View> {
            width: 140, height: 48
            show_bg: true
            cursor: Hand
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                instance recording: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let normal = vec4(0.976, 0.451, 0.086, 1.0);
                    let recording_color = vec4(0.937, 0.267, 0.267, 1.0);
                    sdf.fill(mix(normal, recording_color, self.recording));
                    return sdf.result;
                }
            }
            btn_label = <Label> {
                text: "🎙 开始录音"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    color: (WHITE)
                }
            }
        }
        next_btn = <View> {
            width: 120, height: 48
            show_bg: true
            cursor: Hand
            align: {x: 0.5, y: 0.5}
            draw_bg: {
                instance disabled: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let enabled = vec4(0.133, 0.773, 0.369, 1.0);
                    let disabled_color = vec4(0.6, 0.65, 0.68, 1.0);
                    sdf.fill(mix(enabled, disabled_color, self.disabled));
                    return sdf.result;
                }
            }
            btn_label = <Label> {
                text: "下一句 ⏭"
                draw_text: {
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    color: (WHITE)
                }
            }
        }
    }

    // ========================================================================
    // Main Reading Practice Screen - Website-aligned compact layout
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
                padding: {left: 16, right: 16, top: 16, bottom: 16}
                spacing: 16

                // Header Card - compact style matching website
                header_card = <RoundedView> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 0
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 12.0
                        fn get_color(self) -> vec4 {
                            return mix((WHITE), (SLATE_800), self.dark_mode);
                        }
                    }

                    // Title row with level badge
                    title_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        padding: {left: 16, right: 16, top: 12, bottom: 12}
                        align: {y: 0.5}
                        spacing: 12

                        <Label> {
                            text: "🎤 大声跟读"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_BOLD>{ font_size: 18.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        level_badge = <LevelBadge> {
                            level_text = { text: "Lv.5" }
                        }

                        <View> { width: Fill }

                        progress_label = <Label> {
                            text: "0/0"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }
                    }

                    // Subject pills row with border-top
                    subject_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        padding: {left: 16, right: 16, top: 12, bottom: 12}
                        spacing: 8
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                // Background
                                let bg = mix(vec4(0.97, 0.97, 0.98, 1.), vec4(0.11, 0.12, 0.14, 1.), self.dark_mode);
                                sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                                sdf.fill(bg);
                                // Top border
                                let border = mix(vec4(0.9, 0.91, 0.92, 1.), vec4(0.2, 0.22, 0.25, 1.), self.dark_mode);
                                sdf.rect(0., 0., self.rect_size.x, 1.);
                                sdf.fill(border);
                                return sdf.result;
                            }
                        }

                        // Loading indicator
                        loading_label = <Label> {
                            text: "加载场景..."
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                }
                            }
                        }

                        // "All subjects" pill
                        all_pill = <SubjectPill> {
                            visible: false
                            pill_label = { text: "全部场景" }
                        }

                        // Subject pills (max 5 visible, plus "more" button)
                        tab0 = <SubjectPill> { visible: false, pill_label = { text: "场景1" } }
                        tab1 = <SubjectPill> { visible: false, pill_label = { text: "场景2" } }
                        tab2 = <SubjectPill> { visible: false, pill_label = { text: "场景3" } }
                        tab3 = <SubjectPill> { visible: false, pill_label = { text: "场景4" } }
                        tab4 = <SubjectPill> { visible: false, pill_label = { text: "场景5" } }
                    }

                    // Keep exercise_tabs hidden for backward compatibility
                    exercise_tabs = <View> {
                        visible: false
                        tab0 = <SubjectPill> { visible: false }
                        tab1 = <SubjectPill> { visible: false }
                        tab2 = <SubjectPill> { visible: false }
                        tab3 = <SubjectPill> { visible: false }
                        tab4 = <SubjectPill> { visible: false }
                    }

                    // Hidden progress section for backward compatibility
                    progress_section = <View> {
                        visible: false
                        progress_track = <View> { show_bg: true, draw_bg: { instance progress: 0.0 } }
                    }
                }

                // Main content card - sentence and controls
                main_card = <RoundedView> {
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

                    // Sentence display
                    sentence_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 4
                        align: {x: 0.5}

                        sentence_en = <Label> {
                            text: "Click to start recording"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_SEMIBOLD>{ font_size: 18.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        sentence_zh = <Label> {
                            text: "选择一个场景开始练习"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
                        }

                        difficulty_label = <View> {
                            width: Fit, height: Fit
                            visible: false
                            <Label> {
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                    color: #9ca3af
                                }
                            }
                        }
                    }

                    // Audio control buttons - compact inline style
                    audio_controls = <CompactActionButtons> {}

                    // Evaluation result section (hidden until recorded)
                    evaluation_section = <View> {
                        width: Fill, height: Fit
                        visible: false
                        flow: Down
                        spacing: 12
                        padding: {top: 12}
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                // Top border
                                let border = mix(vec4(0.9, 0.91, 0.92, 1.), vec4(0.25, 0.27, 0.3, 1.), self.dark_mode);
                                sdf.rect(0., 0., self.rect_size.x, 1.);
                                sdf.fill(border);
                                return sdf.result;
                            }
                        }

                        // Score and bars row
                        score_content = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 16
                            align: {y: 0.5}

                            // Score circle
                            score_circle = <ScoreCircle> {
                                score_text = { text: "85" }
                            }

                            // Score bars
                            score_bars = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 6

                                pronunciation_bar = <ScoreBar> {
                                    bar_label = { text: "发音" }
                                    bar_value = { text: "90" }
                                }

                                fluency_bar = <ScoreBar> {
                                    bar_label = { text: "流畅" }
                                    bar_value = { text: "80" }
                                }

                                intonation_bar = <ScoreBar> {
                                    bar_label = { text: "语调" }
                                    bar_value = { text: "85" }
                                }
                            }
                        }

                        // Feedback tags
                        feedback_row = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 8

                            good_feedback = <View> {
                                width: Fit, height: 24
                                padding: {left: 8, right: 8}
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.);
                                        sdf.fill(vec4(0.86, 0.97, 0.9, 1.)); // green-100
                                        return sdf.result;
                                    }
                                }
                                align: {y: 0.5}
                                good_text = <Label> {
                                    text: "✓ 发音清晰"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: #15803d // green-700
                                    }
                                }
                            }

                            warning_feedback = <View> {
                                width: Fit, height: 24
                                padding: {left: 8, right: 8}
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.);
                                        sdf.fill(vec4(0.996, 0.953, 0.878, 1.)); // amber-100
                                        return sdf.result;
                                    }
                                }
                                align: {y: 0.5}
                                warning_text = <Label> {
                                    text: "⚠ 注意连读"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: #b45309 // amber-700
                                    }
                                }
                            }
                        }

                        // Mastery indicator (shown when score >= 90)
                        mastery_indicator = <View> {
                            width: Fill, height: Fit
                            visible: false
                            align: {x: 0.5}
                            mastery_text = <Label> {
                                text: "✓ 太棒了！这个句子已标记为掌握"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    color: #16a34a // green-600
                                }
                            }
                        }
                    }
                }

                // Legacy elements for backward compatibility (hidden)
                sentence_display = <View> {
                    visible: false
                    exercise_title = <Label> {}
                    sentence_en = <Label> {}
                    sentence_zh = <Label> {}
                    sentence_tips = <Label> {}
                }

                waveforms_row = <View> {
                    visible: false
                    native_waveform = <View> {
                        header = <View> { play_btn = <Button> {} }
                        waveform_view = <View> { placeholder_text = <Label> {} }
                    }
                    user_waveform = <View> {
                        header = <View> { play_btn = <Button> {} }
                        waveform_view = <View> { placeholder_text = <Label> {} }
                    }
                }

                score_card = <View> {
                    visible: false
                    score_row = <View> {
                        total_score = <View> {
                            score_circle = <View> { score_value = <Label> {} }
                        }
                        detailed_scores = <View> {
                            pronunciation_row = <View> {
                                pronunciation_bar = <View> {}
                                pronunciation_score = <Label> {}
                            }
                            fluency_row = <View> {
                                fluency_bar = <View> {}
                                fluency_score = <Label> {}
                            }
                            intonation_row = <View> {
                                intonation_bar = <View> {}
                                intonation_score = <Label> {}
                            }
                        }
                    }
                }

                action_buttons = <View> {
                    visible: false
                    prev_btn = <View> { btn_label = <Label> {} }
                    record_btn = <View> { btn_label = <Label> {} }
                    next_btn = <View> { btn_label = <Label> {} }
                }

                // Tips row - compact inline style matching website
                tips_card = <RoundedView> {
                    width: Fill, height: Fit
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        border_radius: 12.0
                        fn get_color(self) -> vec4 {
                            return mix((WHITE), (SLATE_800), self.dark_mode);
                        }
                    }
                    tips_row = <TipsRow> {}
                }

                // Legacy tips_section (hidden)
                tips_section = <View> { visible: false }
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

        // Handle subject pill clicks - new layout
        for (i, tab_path) in [
            ids!(header_card.subject_row.tab0),
            ids!(header_card.subject_row.tab1),
            ids!(header_card.subject_row.tab2),
            ids!(header_card.subject_row.tab3),
            ids!(header_card.subject_row.tab4),
        ].iter().enumerate() {
            if self.view.view(*tab_path).finger_up(&actions).is_some() {
                if i < self.exercises.len() && i != self.selected_exercise_index {
                    self.selected_exercise_index = i;
                    self.update_exercise_tabs(cx);
                    self.load_sentences(cx);
                    self.view.redraw(cx);
                }
            }
        }

        // Handle next button click (compact layout)
        if self.view.view(ids!(main_card.audio_controls.next_btn)).finger_up(&actions).is_some() {
            if self.current_sentence_index < self.sentences.len().saturating_sub(1) {
                self.current_sentence_index += 1;
                self.has_recorded = false;
                self.update_sentence_display(cx);
                self.update_button_states(cx);
                // Hide evaluation section for new sentence
                self.view.view(ids!(main_card.evaluation_section)).set_visible(cx, false);
                self.view.redraw(cx);
            } else if !self.sentences.is_empty() {
                // Fetch more sentences when reaching the end (like website)
                self.load_sentences(cx);
            }
        }

        // Handle record button click (compact layout)
        if self.view.view(ids!(main_card.audio_controls.record_btn)).finger_up(&actions).is_some() {
            self.is_recording = !self.is_recording;
            if !self.is_recording {
                // Stopped recording, show evaluation
                self.has_recorded = true;
                self.show_evaluation(cx);
            }
            self.update_record_button(cx);
            self.view.redraw(cx);
        }

        // Handle standard audio play button
        if self.view.view(ids!(main_card.audio_controls.standard_btn)).finger_up(&actions).is_some() {
            // TODO: Play standard/native audio
        }

        // Handle playback button
        if self.view.view(ids!(main_card.audio_controls.playback_btn)).finger_up(&actions).is_some() {
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
                    // Hide loading label in new layout
                    self.view.label(ids!(header_card.subject_row.loading_label))
                        .set_visible(cx, false);
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
                    self.view.label(ids!(header_card.subject_row.loading_label))
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
                    // Hide evaluation section for new sentences
                    self.view.view(ids!(main_card.evaluation_section)).set_visible(cx, false);
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

        // Show loading label in new layout
        self.view.label(ids!(header_card.subject_row.loading_label))
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
    fn load_sentences(&mut self, _cx: &mut Cx) {
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
        // Use subject_row paths for new layout
        let tab_paths = [
            ids!(header_card.subject_row.tab0),
            ids!(header_card.subject_row.tab1),
            ids!(header_card.subject_row.tab2),
            ids!(header_card.subject_row.tab3),
            ids!(header_card.subject_row.tab4),
        ];

        for (i, tab_path) in tab_paths.iter().enumerate() {
            let tab = self.view.view(*tab_path);

            if i < self.exercises.len() {
                let exercise = &self.exercises[i];
                tab.set_visible(cx, true);

                // Set label text - use pill_label for new SubjectPill
                tab.label(ids!(pill_label)).set_text(cx, &exercise.title_zh);

                // Update selected state
                let is_selected = i == self.selected_exercise_index;
                let selected_val = if is_selected { 1.0f64 } else { 0.0f64 };

                tab.apply_over(cx, live! {
                    draw_bg: { selected: (selected_val) }
                });
                tab.label(ids!(pill_label)).apply_over(cx, live! {
                    draw_text: { selected: (selected_val) }
                });
            } else {
                tab.set_visible(cx, false);
            }
        }
    }

    /// Update sentence display with current sentence
    fn update_sentence_display(&mut self, cx: &mut Cx) {
        if let Some(sentence) = self.sentences.get(self.current_sentence_index) {
            // Update main card sentence display
            self.view.label(ids!(main_card.sentence_section.sentence_en))
                .set_text(cx, &sentence.content_en);
            self.view.label(ids!(main_card.sentence_section.sentence_zh))
                .set_text(cx, &sentence.content_zh);

            // Show difficulty if available
            if let Some(difficulty) = &self.exercises.get(self.selected_exercise_index)
                .and_then(|e| e.difficulty_level.as_ref()) {
                self.view.label(ids!(main_card.sentence_section.difficulty_label))
                    .set_text(cx, &format!("难度: {}", difficulty));
                self.view.label(ids!(main_card.sentence_section.difficulty_label))
                    .set_visible(cx, true);
            } else {
                self.view.label(ids!(main_card.sentence_section.difficulty_label))
                    .set_visible(cx, false);
            }
        } else {
            self.view.label(ids!(main_card.sentence_section.sentence_en))
                .set_text(cx, "暂无练习内容");
            self.view.label(ids!(main_card.sentence_section.sentence_zh))
                .set_text(cx, "请选择一个场景开始练习");
            self.view.label(ids!(main_card.sentence_section.difficulty_label))
                .set_visible(cx, false);
        }
    }

    /// Update progress bar and label
    fn update_progress(&mut self, cx: &mut Cx) {
        let total = self.sentences.len();
        let current = if total > 0 { self.current_sentence_index + 1 } else { 0 };

        // Update progress label in new compact header
        self.view.label(ids!(header_card.title_row.progress_label))
            .set_text(cx, &format!("{}/{}", current, total));
    }

    /// Update button enabled/disabled states
    fn update_button_states(&mut self, cx: &mut Cx) {
        // Update progress display
        self.update_progress(cx);
    }

    /// Update record button appearance
    fn update_record_button(&mut self, cx: &mut Cx) {
        let recording_val = if self.is_recording { 1.0f64 } else { 0.0f64 };

        let text = if self.is_recording {
            "⏹ 停止"
        } else if self.has_recorded {
            "🎙 重录"
        } else {
            "🎙 录音"
        };

        // Update compact button in new layout
        self.view.label(ids!(main_card.audio_controls.record_btn.btn_label)).set_text(cx, text);
        self.view.view(ids!(main_card.audio_controls.record_btn)).apply_over(cx, live! {
            draw_bg: { recording: (recording_val) }
        });
    }

    /// Show evaluation with mock scores (matches website design)
    fn show_evaluation(&mut self, cx: &mut Cx) {
        // Show evaluation section
        self.view.view(ids!(main_card.evaluation_section)).set_visible(cx, true);

        // Generate mock scores (in real app, this would come from AI evaluation)
        let pronunciation = 85 + (self.current_sentence_index % 10) as i32;
        let fluency = 75 + ((self.current_sentence_index * 3) % 15) as i32;
        let intonation = 80 + ((self.current_sentence_index * 7) % 12) as i32;
        let total = (pronunciation + fluency + intonation) / 3;

        // Score color helper: 0.0 = red (<60), 0.5 = amber (60-79), 1.0 = green (>=80)
        let get_score_color = |score: i32| -> f64 {
            if score >= 80 { 1.0 }
            else if score >= 60 { 0.5 }
            else { 0.0 }
        };

        // Update score circle
        let total_score = total as f64 / 100.0;
        self.view.view(ids!(main_card.evaluation_section.score_content.score_circle))
            .apply_over(cx, live! { draw_bg: { score: (total_score) } });
        self.view.label(ids!(main_card.evaluation_section.score_content.score_circle.score_text))
            .set_text(cx, &format!("{}", total));
        self.view.label(ids!(main_card.evaluation_section.score_content.score_circle.score_text))
            .apply_over(cx, live! { draw_text: { score: (total_score) } });

        // Update pronunciation bar
        let pron_progress = pronunciation as f64 / 100.0;
        let pron_color = get_score_color(pronunciation);
        self.view.view(ids!(main_card.evaluation_section.score_content.score_bars.pronunciation_bar.bar_track))
            .apply_over(cx, live! { draw_bg: { progress: (pron_progress), score_color: (pron_color) } });
        self.view.label(ids!(main_card.evaluation_section.score_content.score_bars.pronunciation_bar.bar_value))
            .set_text(cx, &format!("{}", pronunciation));
        self.view.label(ids!(main_card.evaluation_section.score_content.score_bars.pronunciation_bar.bar_value))
            .apply_over(cx, live! { draw_text: { score_color: (pron_color) } });

        // Update fluency bar
        let fluency_progress = fluency as f64 / 100.0;
        let fluency_color = get_score_color(fluency);
        self.view.view(ids!(main_card.evaluation_section.score_content.score_bars.fluency_bar.bar_track))
            .apply_over(cx, live! { draw_bg: { progress: (fluency_progress), score_color: (fluency_color) } });
        self.view.label(ids!(main_card.evaluation_section.score_content.score_bars.fluency_bar.bar_value))
            .set_text(cx, &format!("{}", fluency));
        self.view.label(ids!(main_card.evaluation_section.score_content.score_bars.fluency_bar.bar_value))
            .apply_over(cx, live! { draw_text: { score_color: (fluency_color) } });

        // Update intonation bar
        let intonation_progress = intonation as f64 / 100.0;
        let intonation_color = get_score_color(intonation);
        self.view.view(ids!(main_card.evaluation_section.score_content.score_bars.intonation_bar.bar_track))
            .apply_over(cx, live! { draw_bg: { progress: (intonation_progress), score_color: (intonation_color) } });
        self.view.label(ids!(main_card.evaluation_section.score_content.score_bars.intonation_bar.bar_value))
            .set_text(cx, &format!("{}", intonation));
        self.view.label(ids!(main_card.evaluation_section.score_content.score_bars.intonation_bar.bar_value))
            .apply_over(cx, live! { draw_text: { score_color: (intonation_color) } });

        // Show mastery indicator if score >= 90
        self.view.view(ids!(main_card.evaluation_section.mastery_indicator))
            .set_visible(cx, total >= 90);
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
