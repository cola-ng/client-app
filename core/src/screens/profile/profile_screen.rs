//! # Profile Screen
//!
//! User profile management with tabs for profile, achievements, and settings.
//! Based on website MePage.tsx.

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

    // Tab button for profile screen - matches website TabsTrigger style
    ProfileTabButton = <Button> {
        width: Fit, height: Fit
        padding: {left: 14, right: 14, top: 8, bottom: 8}
        margin: {right: 4}

        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                if self.active > 0.5 {
                    // Active: orange-100 bg
                    let light = vec4(1.0, 0.929, 0.835, 1.0); // orange-100
                    let dark = (SLATE_700);
                    sdf.fill(mix(light, dark, self.dark_mode));
                } else if self.hover > 0.5 {
                    // Hover: gray-100
                    let light = vec4(0.953, 0.961, 0.969, 1.0);
                    let dark = (SLATE_700);
                    sdf.fill(mix(light, dark, self.dark_mode));
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance active: 0.0
            text_style: <FONT_MEDIUM>{ font_size: 13.0 }
            fn get_color(self) -> vec4 {
                // Active: orange-800, Inactive: gray-600
                let light_inactive = vec4(0.298, 0.333, 0.388, 1.0); // gray-600
                let light_active = vec4(0.604, 0.176, 0.071, 1.0);   // orange-800
                let dark_inactive = (SLATE_400);
                let dark_active = vec4(1.0, 0.929, 0.835, 1.0); // orange-100

                let inactive = mix(light_inactive, dark_inactive, self.dark_mode);
                let active_color = mix(light_active, dark_active, self.dark_mode);
                return mix(inactive, active_color, self.active);
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
                // Gradient matching website: from-orange-50 via-amber-50 to-yellow-50
                let orange_50 = vec4(1.0, 0.969, 0.929, 1.0);
                let amber_50 = vec4(1.0, 0.984, 0.922, 1.0);
                let yellow_50 = vec4(0.996, 0.988, 0.910, 1.0);
                let dark_bg = vec4(0.067, 0.075, 0.102, 1.0);

                let t = self.pos.x + self.pos.y * 0.3;
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
                padding: 24
                spacing: 20

                // Profile header card with gradient banner
                profile_header_card = <RoundedView> {
                    width: Fill, height: Fit
                    flow: Down
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

                    // Gradient banner
                    banner = <View> {
                        width: Fill, height: 80
                        show_bg: true
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                // Gradient: orange-500 → amber-500 → orange-400
                                let orange_500 = vec4(0.976, 0.451, 0.086, 1.0);
                                let amber_500 = vec4(0.961, 0.620, 0.043, 1.0);
                                let orange_400 = vec4(0.984, 0.573, 0.235, 1.0);

                                let t = self.pos.x;
                                if t < 0.5 {
                                    return mix(orange_500, amber_500, t * 2.0);
                                } else {
                                    return mix(amber_500, orange_400, (t - 0.5) * 2.0);
                                }
                            }
                        }
                    }

                    // Avatar and info section
                    avatar_info_section = <View> {
                        width: Fill, height: Fit
                        flow: Right
                        padding: {left: 20, right: 20, top: 0, bottom: 16}
                        spacing: 16
                        align: {y: 1.0}
                        margin: {top: -40}

                        avatar = <ProfileAvatar> {}

                        user_info = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 2
                            padding: {top: 44}

                            user_name = <Label> {
                                text: "用户12345"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_BOLD>{ font_size: 18.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            user_email = <Label> {
                                text: "user@example.com"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_MUTED), (SLATE_500), self.dark_mode);
                                    }
                                }
                            }
                        }
                    }
                }

                // Tabs row with muted background container
                tabs_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    padding: {bottom: 8}

                    tabs_bg = <RoundedView> {
                        width: Fit, height: Fit
                        padding: 4
                        flow: Right
                        spacing: 0
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: 8.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let light = vec4(0.961, 0.953, 0.945, 1.0); // muted
                                let dark = (SLATE_800);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                sdf.fill(mix(light, dark, self.dark_mode));
                                return sdf.result;
                            }
                        }

                        profile_tab = <ProfileTabButton> {
                            text: "👤 个人资料"
                            draw_bg: { active: 1.0 }
                            draw_text: { active: 1.0 }
                        }

                        achievements_tab = <ProfileTabButton> {
                            text: "🏆 成就"
                        }

                        settings_tab = <ProfileTabButton> {
                            text: "⚙️ 设置"
                        }
                    }
                }

                // Profile tab content (visible by default)
                profile_tab_content = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 20

                    // Account info card
                    account_card = <ProfileSection> {
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: {y: 0.5}
                            spacing: 12

                            <View> {
                                width: 32, height: 32
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                        sdf.fill(#dbeafe); // blue-100
                                        return sdf.result;
                                    }
                                }
                                align: {x: 0.5, y: 0.5}

                                <Label> {
                                    text: "🛡️"
                                    draw_text: { text_style: { font_size: 16.0 } }
                                }
                            }

                            section_title = <Label> {
                                text: "账户信息"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }

                        email_row = <InfoRow> {
                            label = { text: "邮箱地址" }
                            value = { text: "user@example.com" }
                        }

                        phone_row = <InfoRow> {
                            label = { text: "手机号码" }
                            value = { text: "未设置" }
                        }

                        join_row = <InfoRow> {
                            label = { text: "注册时间" }
                            value = { text: "2024年1月15日" }
                        }
                    }

                    // Edit profile card
                    edit_card = <ProfileSection> {
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: {y: 0.5}
                            spacing: 12

                            <View> {
                                width: 32, height: 32
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                        sdf.fill((ORANGE_100));
                                        return sdf.result;
                                    }
                                }
                                align: {x: 0.5, y: 0.5}

                                <Label> {
                                    text: "👤"
                                    draw_text: { text_style: { font_size: 16.0 } }
                                }
                            }

                            <Label> {
                                text: "编辑资料"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }

                        name_input_row = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 4

                            <Label> {
                                text: "显示名称"
                                draw_text: {
                                    instance dark_mode: 0.0
                                    text_style: <FONT_MEDIUM>{ font_size: 12.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                                    }
                                }
                            }

                            name_input = <TextInput> {
                                width: Fill, height: Fit
                                padding: {left: 12, right: 12, top: 10, bottom: 10}
                                text: "输入您的姓名"

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
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                    fn get_color(self) -> vec4 {
                                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                    }
                                }
                            }
                        }

                        save_btn = <Button> {
                            width: Fill, height: Fit
                            padding: {top: 12, bottom: 12}
                            align: {x: 0.5}
                            text: "保存更改"

                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                    // Gradient from orange to amber
                                    let t = self.pos.x;
                                    let orange = vec4(0.976, 0.451, 0.086, 1.0);
                                    let amber = vec4(0.961, 0.620, 0.043, 1.0);
                                    sdf.fill(mix(orange, amber, t));
                                    return sdf.result;
                                }
                            }

                            draw_text: {
                                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                color: (WHITE)
                            }
                        }
                    }
                }

                // Achievements tab content (hidden by default)
                achievements_tab_content = <View> {
                    width: Fill, height: Fit
                    visible: false
                    flow: Down
                    spacing: 20

                    // Rank & XP Card with gradient
                    rank_card = <RoundedView> {
                        width: Fill, height: Fit
                        flow: Down
                        show_bg: true
                        draw_bg: {
                            border_radius: 12.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                                // Gradient: orange-500 → amber-500 → orange-400
                                let t = self.pos.x;
                                let orange_500 = vec4(0.976, 0.451, 0.086, 1.0);
                                let amber_500 = vec4(0.961, 0.620, 0.043, 1.0);
                                sdf.fill(mix(orange_500, amber_500, t));
                                return sdf.result;
                            }
                        }

                        rank_content = <View> {
                            width: Fill, height: Fit
                            padding: 20
                            flow: Right
                            align: {y: 0.5}
                            spacing: 16

                            rank_icon = <View> {
                                width: 56, height: 56
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.circle(self.rect_size.x * 0.5, self.rect_size.y * 0.5, 26.);
                                        sdf.fill((ACCENT_YELLOW));
                                        return sdf.result;
                                    }
                                }
                                align: {x: 0.5, y: 0.5}

                                <Label> {
                                    text: "🏆"
                                    draw_text: { text_style: { font_size: 24.0 } }
                                }
                            }

                            rank_info = <View> {
                                width: Fill, height: Fit
                                flow: Down
                                spacing: 2

                                rank_name = <Label> {
                                    text: "新手"
                                    draw_text: {
                                        text_style: <FONT_BOLD>{ font_size: 20.0 }
                                        color: (WHITE)
                                    }
                                }

                                rank_level = <Label> {
                                    text: "Lv.1"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                        color: #ffffff80
                                    }
                                }
                            }

                            xp_info = <View> {
                                width: Fit, height: Fit
                                flow: Down
                                align: {x: 1.0}
                                spacing: 2

                                xp_row = <View> {
                                    width: Fit, height: Fit
                                    flow: Right
                                    spacing: 4
                                    align: {y: 0.5}

                                    <Label> {
                                        text: "⚡"
                                        draw_text: { text_style: { font_size: 18.0 } }
                                    }

                                    xp_value = <Label> {
                                        text: "0"
                                        draw_text: {
                                            text_style: <FONT_BOLD>{ font_size: 20.0 }
                                            color: (WHITE)
                                        }
                                    }

                                    <Label> {
                                        text: "XP"
                                        draw_text: {
                                            text_style: <FONT_REGULAR>{ font_size: 14.0 }
                                            color: #ffffff80
                                        }
                                    }
                                }
                            }
                        }

                        // Stats row inside rank card
                        stats_row = <View> {
                            width: Fill, height: Fit
                            padding: 16
                            flow: Right
                            spacing: 12
                            show_bg: true
                            draw_bg: {
                                instance dark_mode: 0.0
                                fn pixel(self) -> vec4 {
                                    return mix((WHITE), (SLATE_800), self.dark_mode);
                                }
                            }

                            stat_streak = <RoundedView> {
                                width: Fill, height: Fit
                                padding: 12
                                flow: Down
                                align: {x: 0.5}
                                spacing: 4
                                show_bg: true
                                draw_bg: {
                                    border_radius: 12.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                                        sdf.fill((ORANGE_50));
                                        return sdf.result;
                                    }
                                }

                                <Label> {
                                    text: "🔥"
                                    draw_text: { text_style: { font_size: 20.0 } }
                                }
                                streak_value = <Label> {
                                    text: "0"
                                    draw_text: {
                                        text_style: <FONT_BOLD>{ font_size: 20.0 }
                                        color: (ACCENT_PRIMARY)
                                    }
                                }
                                <Label> {
                                    text: "连续天数"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: (TEXT_MUTED)
                                    }
                                }
                            }

                            stat_achievements = <RoundedView> {
                                width: Fill, height: Fit
                                padding: 12
                                flow: Down
                                align: {x: 0.5}
                                spacing: 4
                                show_bg: true
                                draw_bg: {
                                    border_radius: 12.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                                        sdf.fill(#dcfce7); // green-100
                                        return sdf.result;
                                    }
                                }

                                <Label> {
                                    text: "🏅"
                                    draw_text: { text_style: { font_size: 20.0 } }
                                }
                                achievements_value = <Label> {
                                    text: "0"
                                    draw_text: {
                                        text_style: <FONT_BOLD>{ font_size: 20.0 }
                                        color: (ACCENT_GREEN)
                                    }
                                }
                                <Label> {
                                    text: "已获成就"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: (TEXT_MUTED)
                                    }
                                }
                            }

                            stat_level = <RoundedView> {
                                width: Fill, height: Fit
                                padding: 12
                                flow: Down
                                align: {x: 0.5}
                                spacing: 4
                                show_bg: true
                                draw_bg: {
                                    border_radius: 12.0
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                                        sdf.fill(#dbeafe); // blue-100
                                        return sdf.result;
                                    }
                                }

                                <Label> {
                                    text: "🎯"
                                    draw_text: { text_style: { font_size: 20.0 } }
                                }
                                level_value = <Label> {
                                    text: "1"
                                    draw_text: {
                                        text_style: <FONT_BOLD>{ font_size: 20.0 }
                                        color: #3b82f6 // blue-500
                                    }
                                }
                                <Label> {
                                    text: "当前等级"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: (TEXT_MUTED)
                                    }
                                }
                            }
                        }
                    }

                    // Achievements badges card
                    badges_card = <ProfileSection> {
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: {y: 0.5}
                            spacing: 12

                            <View> {
                                width: 32, height: 32
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                        sdf.fill(#fef3c7); // amber-100
                                        return sdf.result;
                                    }
                                }
                                align: {x: 0.5, y: 0.5}

                                <Label> {
                                    text: "🏅"
                                    draw_text: { text_style: { font_size: 16.0 } }
                                }
                            }

                            <View> {
                                width: Fill, height: Fit
                                flow: Down

                                <Label> {
                                    text: "成就勋章"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                badges_count = <Label> {
                                    text: "已解锁 0/0 个成就"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: (TEXT_MUTED)
                                    }
                                }
                            }
                        }

                        badges_row = <View> {
                            width: Fill, height: Fit
                            flow: Right
                            spacing: 12
                            margin: {top: 12}

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
                }

                // Settings tab content (hidden by default)
                settings_tab_content = <View> {
                    width: Fill, height: Fit
                    visible: false
                    flow: Down
                    spacing: 20

                    // AI Provider settings card
                    ai_provider_card = <ProfileSection> {
                        <View> {
                            width: Fill, height: Fit
                            flow: Right
                            align: {y: 0.5}
                            spacing: 12

                            <View> {
                                width: 32, height: 32
                                show_bg: true
                                draw_bg: {
                                    fn pixel(self) -> vec4 {
                                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                        sdf.fill(#f3e8ff); // purple-100
                                        return sdf.result;
                                    }
                                }
                                align: {x: 0.5, y: 0.5}

                                <Label> {
                                    text: "🤖"
                                    draw_text: { text_style: { font_size: 16.0 } }
                                }
                            }

                            <View> {
                                width: Fill, height: Fit
                                flow: Down

                                <Label> {
                                    text: "AI 提供商设置"
                                    draw_text: {
                                        instance dark_mode: 0.0
                                        text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                                        fn get_color(self) -> vec4 {
                                            return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                                        }
                                    }
                                }

                                <Label> {
                                    text: "配置您的 AI 服务提供商"
                                    draw_text: {
                                        text_style: <FONT_REGULAR>{ font_size: 11.0 }
                                        color: (TEXT_MUTED)
                                    }
                                }
                            }
                        }

                        provider_hint = <Label> {
                            text: "AI 提供商设置功能即将上线"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                fn get_color(self) -> vec4 {
                                    return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                                }
                            }
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
    SaveProfile,
}

/// Active tab for profile screen
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ProfileTab {
    #[default]
    Profile,
    Achievements,
    Settings,
}

#[derive(Live, LiveHook, Widget)]
pub struct ProfileScreen {
    #[deref]
    view: View,
    #[rust]
    active_tab: ProfileTab,
}

impl Widget for ProfileScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle tab switching
        if self.view.button(ids!(content_scroll.content.tabs_row.tabs_bg.profile_tab)).clicked(actions) {
            self.switch_tab(cx, ProfileTab::Profile);
        }
        if self.view.button(ids!(content_scroll.content.tabs_row.tabs_bg.achievements_tab)).clicked(actions) {
            self.switch_tab(cx, ProfileTab::Achievements);
        }
        if self.view.button(ids!(content_scroll.content.tabs_row.tabs_bg.settings_tab)).clicked(actions) {
            self.switch_tab(cx, ProfileTab::Settings);
        }

        // Handle save profile button
        if self.view.button(ids!(content_scroll.content.profile_tab_content.edit_card.save_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, ProfileScreenAction::SaveProfile);
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

impl ProfileScreen {
    fn switch_tab(&mut self, cx: &mut Cx, tab: ProfileTab) {
        if self.active_tab == tab {
            return;
        }
        self.active_tab = tab;

        let (profile_active, achievements_active, settings_active) = match tab {
            ProfileTab::Profile => (1.0, 0.0, 0.0),
            ProfileTab::Achievements => (0.0, 1.0, 0.0),
            ProfileTab::Settings => (0.0, 0.0, 1.0),
        };

        // Update tab button styles
        self.view.button(ids!(content_scroll.content.tabs_row.tabs_bg.profile_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (profile_active) }
                draw_text: { active: (profile_active) }
            },
        );
        self.view.button(ids!(content_scroll.content.tabs_row.tabs_bg.achievements_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (achievements_active) }
                draw_text: { active: (achievements_active) }
            },
        );
        self.view.button(ids!(content_scroll.content.tabs_row.tabs_bg.settings_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (settings_active) }
                draw_text: { active: (settings_active) }
            },
        );

        // Toggle content visibility
        self.view.view(ids!(content_scroll.content.profile_tab_content))
            .set_visible(cx, tab == ProfileTab::Profile);
        self.view.view(ids!(content_scroll.content.achievements_tab_content))
            .set_visible(cx, tab == ProfileTab::Achievements);
        self.view.view(ids!(content_scroll.content.settings_tab_content))
            .set_visible(cx, tab == ProfileTab::Settings);

        self.view.redraw(cx);
    }
}
