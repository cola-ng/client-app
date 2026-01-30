//! # Profile Screen
//!
//! User profile management with avatar, info editing, and achievements.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Avatar display
    ProfileAvatar = <View> {
        width: 100, height: 100
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                sdf.circle(c.x, c.y, 48.);
                let bg = mix((ORANGE_200), (SLATE_600), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        avatar_text = <Label> {
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}
            text: "用"
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_BOLD>{ font_size: 36.0 }
                fn get_color(self) -> vec4 {
                    return mix((ACCENT_PRIMARY), (WHITE), self.dark_mode);
                }
            }
        }
    }

    // Section card
    ProfileSection = <RoundedView> {
        width: Fill, height: Fit
        padding: 20
        flow: Down
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            border_radius: 12.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }
    }

    // Info row
    InfoRow = <View> {
        width: Fill, height: Fit
        flow: Right
        align: {y: 0.5}
        padding: {top: 8, bottom: 8}

        label = <Label> {
            width: 100
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }

        value = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Achievement badge
    AchievementBadge = <View> {
        width: 80, height: Fit
        flow: Down
        align: {x: 0.5}
        spacing: 8

        icon = <View> {
            width: 56, height: 56
            align: {x: 0.5, y: 0.5}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                instance unlocked: 1.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    sdf.circle(c.x, c.y, 26.);

                    let locked_bg = mix((SLATE_200), (SLATE_700), self.dark_mode);
                    let unlocked_bg = (ACCENT_YELLOW);
                    sdf.fill(mix(locked_bg, unlocked_bg, self.unlocked));

                    return sdf.result;
                }
            }

            emoji = <Label> {
                draw_text: {
                    text_style: { font_size: 24.0 }
                }
            }
        }

        name = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_MEDIUM>{ font_size: 11.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Edit button
    EditButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 12, right: 12, top: 6, bottom: 6}
        text: "编辑"

        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                let bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                sdf.fill(bg);
                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 12.0 }
            fn get_color(self) -> vec4 {
                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
            }
        }
    }

    pub ProfileScreen = {{ProfileScreen}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        content_scroll = <ScrollYView> {
            width: Fill, height: Fill

            content = <View> {
                width: Fill, height: Fit
                flow: Down
                padding: 24
                spacing: 20

                // Header with title
                header = <View> {
                    width: Fill, height: Fit

                    title = <Label> {
                        text: "个人中心"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 24.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Profile card
                profile_card = <ProfileSection> {
                    flow: Down
                    spacing: 20
                    align: {x: 0.5}

                    avatar_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        align: {x: 0.5}
                        spacing: 12

                        avatar = <ProfileAvatar> {}

                        change_avatar_btn = <Button> {
                            width: Fit, height: Fit
                            padding: {left: 12, right: 12, top: 6, bottom: 6}
                            text: "更换头像"
                            draw_bg: {
                                fn pixel(self) -> vec4 { return (TRANSPARENT); }
                            }
                            draw_text: {
                                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                                color: (ACCENT_PRIMARY)
                            }
                        }
                    }

                    // User info
                    info_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 4

                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: {y: 0.5}

                            section_title = <Label> {
                                text: "基本信息"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            <View> { width: Fill }

                            edit_btn = <EditButton> {}
                        }

                        name_row = <InfoRow> {
                            label = { text: "用户名" }
                            value = { text: "用户12345" }
                        }

                        email_row = <InfoRow> {
                            label = { text: "邮箱" }
                            value = { text: "user@example.com" }
                        }

                        join_row = <InfoRow> {
                            label = { text: "注册时间" }
                            value = { text: "2024年1月15日" }
                        }
                    }
                }

                // Statistics card
                stats_card = <ProfileSection> {
                    section_title = <Label> {
                        text: "学习统计"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    stats_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 24
                        margin: {top: 8}

                        stat_1 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            align: {x: 0.5}
                            spacing: 4

                            value = <Label> {
                                text: "1,234"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                                    fn get_color(self) -> vec4 {
                                        return (ACCENT_PRIMARY);
                                    }
                                }
                            }
                            label = <Label> {
                                text: "已学单词"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }

                        stat_2 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            align: {x: 0.5}
                            spacing: 4

                            value = <Label> {
                                text: "42"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                                    fn get_color(self) -> vec4 {
                                        return (ACCENT_PRIMARY);
                                    }
                                }
                            }
                            label = <Label> {
                                text: "连续天数"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }

                        stat_3 = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            align: {x: 0.5}
                            spacing: 4

                            value = <Label> {
                                text: "86%"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                                    fn get_color(self) -> vec4 {
                                        return (ACCENT_PRIMARY);
                                    }
                                }
                            }
                            label = <Label> {
                                text: "正确率"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }
                    }
                }

                // Achievements card
                achievements_card = <ProfileSection> {
                    section_title = <Label> {
                        text: "成就徽章"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    badges_row = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        spacing: 16
                        margin: {top: 8}

                        badge_1 = <AchievementBadge> {
                            icon = { emoji = { text: "🌟" } }
                            name = { text: "初学者" }
                        }

                        badge_2 = <AchievementBadge> {
                            icon = { emoji = { text: "🔥" } }
                            name = { text: "连续7天" }
                        }

                        badge_3 = <AchievementBadge> {
                            icon = { emoji = { text: "📚" } }
                            name = { text: "100单词" }
                        }

                        badge_4 = <AchievementBadge> {
                            icon = {
                                draw_bg: { unlocked: 0.0 }
                                emoji = { text: "🏆" }
                            }
                            name = { text: "大师" }
                        }
                    }
                }

                // Logout button
                logout_section = <View> {
                    width: Fill, height: Fit
                    align: {x: 0.5}
                    margin: {top: 12}

                    logout_btn = <Button> {
                        width: Fill, height: Fit
                        padding: {top: 14, bottom: 14}
                        align: {x: 0.5}
                        text: "退出登录"

                        draw_bg: {
                            instance dark_mode: 0.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                let bg = mix(#fee2e2, #7f1d1d, self.dark_mode);
                                sdf.fill(bg);
                                return sdf.result;
                            }
                        }

                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                            fn get_color(self) -> vec4 {
                                return mix(#b91c1c, #f87171, self.dark_mode);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ProfileScreenAction {
    None,
    EditProfile,
    ChangeAvatar,
    Logout,
}

#[derive(Live, LiveHook, Widget)]
pub struct ProfileScreen {
    #[deref]
    view: View,
}

impl Widget for ProfileScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle edit button
        if self.view.button(ids!(content_scroll.content.profile_card.info_section.edit_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ProfileScreenAction::EditProfile);
        }

        // Handle change avatar
        if self.view.button(ids!(content_scroll.content.profile_card.avatar_section.change_avatar_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ProfileScreenAction::ChangeAvatar);
        }

        // Handle logout
        if self.view.button(ids!(content_scroll.content.logout_section.logout_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ProfileScreenAction::Logout);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
