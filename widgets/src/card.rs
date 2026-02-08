//! # Card Component
//!
//! Reusable card widget with header, content, and footer sections.
//!
//! ## Structure
//! - CardHeader: Title and optional actions
//! - CardContent: Main content area
//! - CardFooter: Bottom section for actions

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // CARD BASE - Foundation for all card variants
    // ==========================================================================
    pub Card = <RoundedView> {
        width: Fill, height: Fit
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance elevated: 0.0
            instance border_radius: 12.0

            fn get_color(self) -> vec4 {
                let light_bg = mix((WHITE), (SLATE_50), self.elevated);
                let dark_bg = mix((SLATE_800), (SLATE_700), self.elevated);
                return mix(light_bg, dark_bg, self.dark_mode);
            }
        }
    }

    // Card with border
    pub CardBordered = <Card> {
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                let light_bg = (WHITE);
                let dark_bg = (SLATE_800);
                let light_border = (SLATE_200);
                let dark_border = (SLATE_700);

                let bg = mix(light_bg, dark_bg, self.dark_mode);
                let border = mix(light_border, dark_border, self.dark_mode);

                sdf.fill(bg);
                sdf.stroke(border, 1.0);
                return sdf.result;
            }
        }
    }

    // Card with shadow effect
    pub CardElevated = <Card> {
        draw_bg: {
            instance dark_mode: 0.0
            instance border_radius: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow effect (subtle offset)
                sdf.box(2., 3., self.rect_size.x - 2., self.rect_size.y - 2., 12.0);
                let shadow = mix(vec4(0., 0., 0., 0.08), vec4(0., 0., 0., 0.2), self.dark_mode);
                sdf.fill(shadow);

                // Main card
                sdf.box(0., 0., self.rect_size.x - 2., self.rect_size.y - 2., 12.0);
                let light_bg = (WHITE);
                let dark_bg = (SLATE_800);
                let bg = mix(light_bg, dark_bg, self.dark_mode);
                sdf.fill(bg);

                return sdf.result;
            }
        }
    }

    // ==========================================================================
    // CARD HEADER - Title section with optional actions
    // ==========================================================================
    pub CardHeader = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {left: 20, right: 20, top: 16, bottom: 16}
        spacing: 12

        header_content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            card_title = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            card_description = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        header_actions = <View> {
            width: Fit, height: Fit
            flow: Right
            spacing: 8
        }
    }

    // Compact header without description
    pub CardHeaderCompact = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {left: 20, right: 20, top: 12, bottom: 12}
        spacing: 12

        card_title = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }

        <View> { width: Fill }

        header_actions = <View> {
            width: Fit, height: Fit
            flow: Right
            spacing: 8
        }
    }

    // ==========================================================================
    // CARD CONTENT - Main content area
    // ==========================================================================
    pub CardContent = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: {left: 20, right: 20, top: 0, bottom: 20}
    }

    // Content with top padding (for cards without header)
    pub CardContentPadded = <View> {
        width: Fill, height: Fit
        flow: Down
        padding: 20
    }

    // ==========================================================================
    // CARD FOOTER - Bottom section with actions
    // ==========================================================================
    pub CardFooter = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {left: 20, right: 20, top: 16, bottom: 16}
        spacing: 12
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Top border
                sdf.rect(0., 0., self.rect_size.x, 1.0);
                let border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                sdf.fill(border);
                // Background
                sdf.rect(0., 1., self.rect_size.x, self.rect_size.y - 1.);
                let bg = mix((SLATE_50), (SLATE_800), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }
    }

    // Footer with actions aligned right
    pub CardFooterActions = <CardFooter> {
        align: {x: 1.0, y: 0.5}
    }

    // ==========================================================================
    // CARD DIVIDER - Horizontal separator
    // ==========================================================================
    pub CardDivider = <View> {
        width: Fill, height: 1
        margin: {left: 20, right: 20}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_200), (SLATE_700), self.dark_mode);
            }
        }
    }

    // Full width divider
    pub CardDividerFull = <View> {
        width: Fill, height: 1
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((SLATE_200), (SLATE_700), self.dark_mode);
            }
        }
    }

    // ==========================================================================
    // COMPLETE CARD EXAMPLES
    // ==========================================================================

    // Basic card with all sections
    pub CardComplete = <Card> {
        header = <CardHeader> {}
        content = <CardContent> {}
        footer = <CardFooterActions> {}
    }

    // Simple card with just content
    pub CardSimple = <Card> {
        padding: 20
    }
}
