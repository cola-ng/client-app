//! # Practice Screen
//!
//! Main practice session screen that hosts different practice views.

use makepad_widgets::*;

use super::session::{ReviewSessionManager, SessionAction};
use super::types::{ReviewType, SessionSummary};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::review::components::CardBase;
    use crate::screens::review::components::PrimaryButton;
    use crate::screens::review::components::SecondaryButton;

    use crate::screens::review::practice::pronunciation_view::PronunciationView;
    use crate::screens::review::practice::meaning_view::MeaningView;
    use crate::screens::review::practice::spelling_view::SpellingView;
    use crate::screens::review::practice::recognition_view::RecognitionView;
    use crate::screens::review::practice::phrase_view::PhraseView;
    use crate::screens::review::practice::score_display::ScoreDisplay;

    // Progress bar at top
    ProgressHeader = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 12
        padding: {left: 24, right: 24, top: 16, bottom: 16}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((WHITE), (SLATE_800), self.dark_mode);
            }
        }

        top_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 0.5}

            back_btn = <Button> {
                width: Fit, height: Fit
                padding: {left: 8, right: 12, top: 8, bottom: 8}
                text: "← 退出"
                draw_bg: {
                    instance dark_mode: 0.0
                    fn pixel(self) -> vec4 { return (TRANSPARENT); }
                }
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((SLATE_600), (SLATE_400), self.dark_mode);
                    }
                }
            }

            <View> { width: Fill }

            progress_text = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        progress_bar = <RoundedView> {
            width: Fill, height: 8
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance progress: 0.0
                instance border_radius: 4.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                    // Track
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 4.0);
                    let track = mix((SLATE_200), (SLATE_700), self.dark_mode);
                    sdf.fill(track);

                    // Fill
                    let fill_width = self.rect_size.x * self.progress;
                    if fill_width > 0. {
                        sdf.box(0., 0., fill_width, self.rect_size.y, 4.0);
                        sdf.fill((ACCENT_PRIMARY));
                    }

                    return sdf.result;
                }
            }
        }
    }

    // Loading state
    LoadingView = <View> {
        width: Fill, height: Fill
        align: {x: 0.5, y: 0.5}
        flow: Down
        spacing: 16

        loading_spinner = <View> {
            width: 48, height: 48
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance rotation: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let radius = c.x - 4.;

                    // Track
                    sdf.circle(c.x, c.y, radius);
                    let track = mix((SLATE_200), (SLATE_700), self.dark_mode);
                    sdf.stroke(track, 4.0);

                    // Spinner arc
                    let start = self.rotation;
                    sdf.move_to(c.x + cos(start) * radius, c.y + sin(start) * radius);
                    for i in 0..8 {
                        let t = (i as f32) / 8.;
                        let a = start + t * 1.57;
                        sdf.line_to(c.x + cos(a) * radius, c.y + sin(a) * radius);
                    }
                    sdf.stroke((ACCENT_PRIMARY), 4.0);

                    return sdf.result;
                }
            }
        }

        loading_text = <Label> {
            text: "加载中..."
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    pub PracticeScreen = {{PracticeScreen}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        progress_header = <ProgressHeader> {
            progress_text = { text: "1 / 10" }
        }

        content = <View> {
            width: Fill, height: Fill
            flow: Overlay
            padding: {left: 24, right: 24, top: 24, bottom: 24}

            loading_view = <LoadingView> {}

            practice_content = <PageFlip> {
                width: Fill, height: Fill
                visible: false
                active_page: pronunciation

                pronunciation = <PronunciationView> {}
                meaning = <MeaningView> {}
                spelling = <SpellingView> {}
                recognition = <RecognitionView> {}
                phrase = <PhraseView> {}
                score = <ScoreDisplay> {}
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum PracticeScreenAction {
    None,
    Exit,
    SessionCompleted(SessionSummary),
}

#[derive(Live, LiveHook, Widget)]
pub struct PracticeScreen {
    #[deref]
    view: View,
    #[rust]
    session_manager: ReviewSessionManager,
    #[rust]
    review_type: ReviewType,
    #[rust]
    loading_rotation: f64,
}

impl Widget for PracticeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Animate loading spinner
        if self.session_manager.is_loading() {
            if let Event::NextFrame(_) = event {
                self.loading_rotation += 0.1;
                self.view.view(ids!(content.loading_view.loading_spinner)).apply_over(
                    cx,
                    live! { draw_bg: { rotation: (self.loading_rotation) } },
                );
                self.view.redraw(cx);
            }
        }

        // Poll session manager
        let action = self.session_manager.poll();
        self.handle_session_action(cx, scope, action);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle exit button
        if self.view.button(ids!(progress_header.top_row.back_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, PracticeScreenAction::Exit);
        }

        // Handle practice view actions
        self.handle_practice_view_actions(cx, scope, actions);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl PracticeScreen {
    fn handle_session_action(&mut self, cx: &mut Cx, scope: &mut Scope, action: SessionAction) {
        match action {
            SessionAction::None => {}
            SessionAction::SessionStarted => {
                // Hide loading, show practice content
                self.view.view(ids!(content.loading_view)).set_visible(cx, false);
                self.view.page_flip(ids!(content.practice_content)).set_visible(cx, true);

                // Set the correct practice view
                let page_id = match self.review_type {
                    ReviewType::Pronunciation => live_id!(pronunciation),
                    ReviewType::Meaning => live_id!(meaning),
                    ReviewType::Spelling => live_id!(spelling),
                    ReviewType::Recognition => live_id!(recognition),
                    ReviewType::Phrase => live_id!(phrase),
                };
                self.view.page_flip(ids!(content.practice_content)).set_active_page(cx, page_id);

                // Update current item
                self.update_current_item(cx);
                self.update_progress(cx);
                self.view.redraw(cx);
            }
            SessionAction::ItemChanged => {
                self.update_current_item(cx);
                self.update_progress(cx);
                self.view.redraw(cx);
            }
            SessionAction::SessionCompleted(summary) => {
                // Show score display
                self.view.page_flip(ids!(content.practice_content)).set_active_page(cx, live_id!(score));
                // TODO: Pass summary to score display
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    PracticeScreenAction::SessionCompleted(summary),
                );
            }
            SessionAction::Error(e) => {
                eprintln!("Session error: {}", e);
                // TODO: Show error UI
            }
        }
    }

    fn handle_practice_view_actions(&mut self, cx: &mut Cx, scope: &mut Scope, actions: &[Action]) {
        // Handle actions from practice views
        for action in actions {
            // Check for submit actions from child views
            // These would be defined in each practice view module
        }
    }

    fn update_current_item(&mut self, cx: &mut Cx) {
        if let Some(item) = self.session_manager.current_item() {
            // Update the current practice view with the item data
            match self.review_type {
                ReviewType::Pronunciation => {
                    // Update pronunciation view
                }
                ReviewType::Meaning => {
                    // Update meaning view
                }
                ReviewType::Spelling => {
                    // Update spelling view
                }
                ReviewType::Recognition => {
                    // Update recognition view
                }
                ReviewType::Phrase => {
                    // Update phrase view
                }
            }
        }
    }

    fn update_progress(&mut self, cx: &mut Cx) {
        let current = self.session_manager.current_item_number();
        let total = self.session_manager.total_items();
        let progress = self.session_manager.progress();

        // Update progress text
        self.view.label(ids!(progress_header.top_row.progress_text))
            .set_text(cx, &format!("{} / {}", current, total));

        // Update progress bar
        self.view.view(ids!(progress_header.progress_bar)).apply_over(
            cx,
            live! { draw_bg: { progress: (progress) } },
        );
    }

    /// Submit result for current item
    pub fn submit_result(&mut self, cx: &mut Cx, scope: &mut Scope, correct: bool, user_answer: Option<String>, self_rating: Option<u8>) {
        let action = self.session_manager.submit_result(correct, user_answer, self_rating);
        self.handle_session_action(cx, scope, action);
    }
}

impl PracticeScreenRef {
    /// Start a practice session
    pub fn start_session(&self, cx: &mut Cx, review_type: ReviewType) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.review_type = review_type;
            inner.session_manager.start_session(review_type);

            // Show loading state
            inner.view.view(ids!(content.loading_view)).set_visible(cx, true);
            inner.view.page_flip(ids!(content.practice_content)).set_visible(cx, false);
            inner.view.redraw(cx);
        }
    }

    /// Reset the practice screen
    pub fn reset(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.session_manager.reset();
            inner.view.view(ids!(content.loading_view)).set_visible(cx, true);
            inner.view.page_flip(ids!(content.practice_content)).set_visible(cx, false);
            inner.view.redraw(cx);
        }
    }
}
