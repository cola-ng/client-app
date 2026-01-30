//! # Avatar Component
//!
//! User avatar widget with image support and status indicators.
//!
//! ## Features
//! - Image or default placeholder
//! - Multiple sizes
//! - Status indicator (online, offline, busy, away)
//! - Initials fallback

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::*;

    // ==========================================================================
    // AVATAR BASE - Circle container with background
    // ==========================================================================
    pub Avatar = <View> {
        width: 40, height: 40
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Circle background
                sdf.circle(c.x, c.y, c.x - 1.);
                let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                sdf.fill(bg);

                return sdf.result;
            }
        }

        // Default user icon
        avatar_icon = <Icon> {
            draw_icon: {
                svg_file: dep("crate://makepad-widgets/resources/icons/user.svg")
                fn get_color(self) -> vec4 {
                    return (ACCENT_PRIMARY);
                }
            }
            icon_walk: {width: 20, height: 20}
        }
    }

    // Avatar with image
    pub AvatarImage = <View> {
        width: 40, height: 40
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x - 1.);
                sdf.fill((SLATE_200));
                return sdf.result;
            }
        }

        avatar_image = <Image> {
            width: Fill, height: Fill
            fit: Smallest
        }
    }

    // Avatar with initials
    pub AvatarInitials = <View> {
        width: 40, height: 40
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance hue: 0.6  // Hue for background color

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, c.x - 1.);

                // Generate color from hue
                let h = self.hue;
                let s = 0.7;
                let l = mix(0.85, 0.3, self.dark_mode);

                // HSL to RGB conversion
                let c_val = (1. - abs(2. * l - 1.)) * s;
                let x = c_val * (1. - abs(mod(h * 6., 2.) - 1.));
                let m = l - c_val / 2.;

                let h6 = h * 6.;
                let rgb = vec3(0.);
                if h6 < 1. {
                    let rgb = vec3(c_val, x, 0.);
                } else if h6 < 2. {
                    let rgb = vec3(x, c_val, 0.);
                } else if h6 < 3. {
                    let rgb = vec3(0., c_val, x);
                } else if h6 < 4. {
                    let rgb = vec3(0., x, c_val);
                } else if h6 < 5. {
                    let rgb = vec3(x, 0., c_val);
                } else {
                    let rgb = vec3(c_val, 0., x);
                }

                let color = vec4(rgb.x + m, rgb.y + m, rgb.z + m, 1.);
                sdf.fill(color);

                return sdf.result;
            }
        }

        initials = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_700), (SLATE_200), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // AVATAR SIZES
    // ==========================================================================
    pub AvatarSmall = <Avatar> {
        width: 28, height: 28
        avatar_icon = { icon_walk: {width: 14, height: 14} }
    }

    pub AvatarLarge = <Avatar> {
        width: 56, height: 56
        avatar_icon = { icon_walk: {width: 28, height: 28} }
    }

    pub AvatarXLarge = <Avatar> {
        width: 80, height: 80
        avatar_icon = { icon_walk: {width: 40, height: 40} }
    }

    // ==========================================================================
    // AVATAR WITH STATUS INDICATOR
    // ==========================================================================
    pub AvatarWithStatus = <View> {
        width: Fit, height: Fit
        flow: Overlay

        avatar = <Avatar> {}

        status_indicator = <View> {
            width: 14, height: 14
            abs_pos: vec2(28.0, 28.0)
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance status: 0.0  // 0=offline, 1=online, 2=busy, 3=away

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // White border ring
                    sdf.circle(c.x, c.y, c.x);
                    let border = mix((WHITE), (SLATE_800), self.dark_mode);
                    sdf.fill(border);

                    // Status dot
                    sdf.circle(c.x, c.y, c.x - 2.);

                    let offline = (SLATE_400);
                    let online = (ACCENT_GREEN);
                    let busy = (ACCENT_RED);
                    let away = (ACCENT_YELLOW);

                    let is_online = step(0.5, self.status) * (1. - step(1.5, self.status));
                    let is_busy = step(1.5, self.status) * (1. - step(2.5, self.status));
                    let is_away = step(2.5, self.status);

                    let color = offline;
                    let color = mix(color, online, is_online);
                    let color = mix(color, busy, is_busy);
                    let color = mix(color, away, is_away);

                    sdf.fill(color);

                    return sdf.result;
                }
            }
        }
    }

    // Large avatar with status
    pub AvatarWithStatusLarge = <AvatarWithStatus> {
        avatar = <AvatarLarge> {}
        status_indicator = {
            width: 18, height: 18
            abs_pos: vec2(42.0, 42.0)
        }
    }

    // ==========================================================================
    // AVATAR GROUP - Stacked avatars
    // ==========================================================================
    pub AvatarGroup = <View> {
        width: Fit, height: Fit
        flow: Right
        spacing: -12  // Overlap avatars
    }

    // Avatar for use in group (with border)
    pub AvatarGroupItem = <View> {
        width: 40, height: 40
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // White border
                sdf.circle(c.x, c.y, c.x);
                let border = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(border);

                // Inner circle
                sdf.circle(c.x, c.y, c.x - 2.);
                let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                sdf.fill(bg);

                return sdf.result;
            }
        }

        avatar_icon = <Icon> {
            draw_icon: {
                svg_file: dep("crate://makepad-widgets/resources/icons/user.svg")
                fn get_color(self) -> vec4 {
                    return (ACCENT_PRIMARY);
                }
            }
            icon_walk: {width: 18, height: 18}
        }
    }

    // Overflow indicator for avatar group
    pub AvatarGroupOverflow = <View> {
        width: 40, height: 40
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // White border
                sdf.circle(c.x, c.y, c.x);
                let border = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(border);

                // Inner circle
                sdf.circle(c.x, c.y, c.x - 2.);
                let bg = mix((SLATE_200), (SLATE_600), self.dark_mode);
                sdf.fill(bg);

                return sdf.result;
            }
        }

        overflow_text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((SLATE_600), (SLATE_300), self.dark_mode);
                }
            }
        }
    }

    // ==========================================================================
    // AVATAR WITH BADGE
    // ==========================================================================
    pub AvatarWithBadge = <View> {
        width: Fit, height: Fit
        flow: Overlay

        avatar = <Avatar> {}

        badge = <View> {
            width: 20, height: 20
            abs_pos: vec2(26.0, -4.0)
            align: {x: 0.5, y: 0.5}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // White border
                    sdf.circle(c.x, c.y, c.x);
                    let border = mix((WHITE), (SLATE_800), self.dark_mode);
                    sdf.fill(border);

                    // Badge background
                    sdf.circle(c.x, c.y, c.x - 1.5);
                    sdf.fill((ACCENT_RED));

                    return sdf.result;
                }
            }

            badge_text = <Label> {
                draw_text: {
                    text_style: <FONT_BOLD>{ font_size: 10.0 }
                    color: (WHITE)
                }
            }
        }
    }
}
