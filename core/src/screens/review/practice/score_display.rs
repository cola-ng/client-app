//! # Score Display
//!
//! Shows the results summary after completing a practice session.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;
    use crate::screens::review::components::SecondaryButton;

    // Large score circle
    ScoreCircleLarge = <View> {
        width: 180, height: 180
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance score: 0.85

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Background track
                sdf.circle(c.x, c.y, 80.);
                let track = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.stroke(track, 12.0);

                // Score arc
                let start = -1.5707963267; // -90 degrees
                sdf.move_to(c.x, c.y - 80.);
                for i in 0..64 {
                    let t = (i as f32) / 64.;
                    if t <= self.score {
                        let a = start + t * 6.28318530718;
                        sdf.line_to(c.x + cos(a) * 80., c.y + sin(a) * 80.);
                    }
                }

                // Color based on score
                let low_color = (ACCENT_RED);
                let mid_color = (ACCENT_YELLOW);
                let high_color = (ACCENT_GREEN);
                let color = mix(low_color, mix(mid_color, high_color, step(0.6, self.score)), step(0.4, self.score));
                sdf.stroke(color, 12.0);

                return sdf.result;
            }
        }

        score_container = <View> {
            width: Fill, height: Fill
            flow: Down
            align: {x: 0.5, y: 0.5}
            spacing: 4

            score_value = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 48.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            score_label = <Label> {
                text: "分"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }
    }

    // Stat item row
    StatRow = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {top: 12, bottom: 12}

        stat_label = <Label> {
            width: Fill
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }

        stat_value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Divider
    StatDivider = <View> {
        width: Fill, height: 1
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
            }
        }
    }

    pub ScoreDisplay = {{ScoreDisplay}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5}
        spacing: 24
        padding: 24

        // Header
        header = <View> {
            width: Fill, height: Fit
            flow: Down
            align: {x: 0.5}
            spacing: 8

            title = <Label> {
                text: "练习完成！"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            subtitle = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Score card
        score_card = <CardBase> {
            width: Fill, height: Fit
            padding: 32
            flow: Down
            align: {x: 0.5}
            spacing: 24

            score_circle = <ScoreCircleLarge> {}

            feedback_text = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        // Stats card
        stats_card = <CardBase> {
            width: Fill, height: Fit
            padding: 20
            flow: Down

            stats_title = <Label> {
                text: "统计数据"
                margin: {bottom: 12}
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            total_row = <StatRow> {
                stat_label = { text: "总题数" }
                stat_value = { text: "10" }
            }
            <StatDivider> {}

            correct_row = <StatRow> {
                stat_label = { text: "正确" }
                stat_value = { text: "8" }
            }
            <StatDivider> {}

            wrong_row = <StatRow> {
                stat_label = { text: "错误" }
                stat_value = { text: "2" }
            }
            <StatDivider> {}

            accuracy_row = <StatRow> {
                stat_label = { text: "正确率" }
                stat_value = { text: "80%" }
            }
            <StatDivider> {}

            time_row = <StatRow> {
                stat_label = { text: "用时" }
                stat_value = { text: "3:25" }
            }
            <StatDivider> {}

            avg_time_row = <StatRow> {
                stat_label = { text: "平均每题" }
                stat_value = { text: "20.5秒" }
            }
        }

        // Action buttons
        action_row = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 12

            review_mistakes_btn = <SecondaryButton> {
                width: Fill
                text: "复习错题"
            }

            continue_btn = <PrimaryButton> {
                width: Fill
                text: "继续练习"
            }
        }

        // Back to home button
        back_btn = <View> {
            width: Fill, height: Fit
            align: {x: 0.5}

            back_label = <Label> {
                text: "返回首页"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((ACCENT_PRIMARY), (ACCENT_ORANGE_DARK), self.dark_mode);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ScoreDisplayAction {
    None,
    ReviewMistakes,
    ContinuePractice,
    BackToHome,
}

#[derive(Live, LiveHook, Widget)]
pub struct ScoreDisplay {
    #[deref]
    view: View,
}

impl Widget for ScoreDisplay {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle review mistakes button
        if self.view.button(ids!(action_row.review_mistakes_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ScoreDisplayAction::ReviewMistakes);
        }

        // Handle continue button
        if self.view.button(ids!(action_row.continue_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ScoreDisplayAction::ContinuePractice);
        }

        // Handle back button click
        self.handle_back_button(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ScoreDisplay {
    fn handle_back_button(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let back_btn = self.view.view(ids!(back_btn));

        if let Hit::FingerUp(_) = event.hits(cx, back_btn.area()) {
            cx.widget_action(self.widget_uid(), &scope.path, ScoreDisplayAction::BackToHome);
        }
    }
}

/// Session summary data for display
pub struct SessionSummaryDisplay {
    pub total_items: usize,
    pub correct_count: usize,
    pub wrong_count: usize,
    pub accuracy: f64,
    pub total_time_ms: u64,
    pub avg_time_ms: u64,
    pub score: u8,
}

impl ScoreDisplayRef {
    pub fn set_summary(&self, cx: &mut Cx, summary: &SessionSummaryDisplay, practice_type: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            // Set subtitle with practice type
            inner.view.label(ids!(header.subtitle))
                .set_text(cx, &format!("{}练习", practice_type));

            // Set score circle
            let score_normalized = summary.score as f64 / 100.0;
            inner.view.view(ids!(score_card.score_circle)).apply_over(
                cx,
                live! { draw_bg: { score: (score_normalized) } },
            );
            inner.view.label(ids!(score_card.score_circle.score_container.score_value))
                .set_text(cx, &format!("{}", summary.score));

            // Set feedback text based on score
            let feedback = if summary.score >= 90 {
                "太棒了！继续保持！"
            } else if summary.score >= 70 {
                "做得不错！还有提升空间"
            } else if summary.score >= 50 {
                "继续努力，你可以做得更好"
            } else {
                "多加练习，相信你会进步的"
            };
            inner.view.label(ids!(score_card.feedback_text)).set_text(cx, feedback);

            // Set stats
            inner.view.label(ids!(stats_card.total_row.stat_value))
                .set_text(cx, &format!("{}", summary.total_items));
            inner.view.label(ids!(stats_card.correct_row.stat_value))
                .set_text(cx, &format!("{}", summary.correct_count));
            inner.view.label(ids!(stats_card.wrong_row.stat_value))
                .set_text(cx, &format!("{}", summary.wrong_count));
            inner.view.label(ids!(stats_card.accuracy_row.stat_value))
                .set_text(cx, &format!("{:.0}%", summary.accuracy * 100.0));

            // Format time
            let total_seconds = summary.total_time_ms / 1000;
            let minutes = total_seconds / 60;
            let seconds = total_seconds % 60;
            inner.view.label(ids!(stats_card.time_row.stat_value))
                .set_text(cx, &format!("{}:{:02}", minutes, seconds));

            let avg_seconds = summary.avg_time_ms as f64 / 1000.0;
            inner.view.label(ids!(stats_card.avg_time_row.stat_value))
                .set_text(cx, &format!("{:.1}秒", avg_seconds));

            // Show/hide review mistakes button based on wrong count
            inner.view.button(ids!(action_row.review_mistakes_btn))
                .set_visible(cx, summary.wrong_count > 0);

            inner.view.redraw(cx);
        }
    }
}
