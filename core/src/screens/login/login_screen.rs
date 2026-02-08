//! # Login Screen
//!
//! User authentication screen with login/register tabs and OAuth options.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;
    use crate::screens::login::components::*;

    // Tab button for login/register
    AuthTab = <Button> {
        width: Fill, height: Fit
        padding: {top: 12, bottom: 12}
        align: {x: 0.5, y: 0.5}

        draw_bg: {
            instance dark_mode: 0.0
            instance active: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Bottom border for active tab
                if self.active > 0.5 {
                    sdf.rect(0., self.rect_size.y - 2., self.rect_size.x, 2.);
                    sdf.fill((ACCENT_PRIMARY));
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance dark_mode: 0.0
            instance active: 0.0
            text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
            fn get_color(self) -> vec4 {
                let inactive = mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                let active_color = (ACCENT_PRIMARY);
                return mix(inactive, active_color, self.active);
            }
        }
    }

    // Form field group
    FormField = <View> {
        width: Fill, height: Fit
        flow: Down
        spacing: 6

        label = <FormLabel> {}
        input = <FormInput> {}
        error = <FormError> { visible: false }
    }

    pub LoginScreen = {{LoginScreen}} {
        width: Fill, height: Fill
        flow: Down
        align: {x: 0.5, y: 0.5}
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            fn pixel(self) -> vec4 {
                return mix((DARK_BG), (DARK_BG_DARK), self.dark_mode);
            }
        }

        content = <View> {
            width: 400, height: Fit
            flow: Down
            padding: 32
            spacing: 24

            // Logo/title
            header = <View> {
                width: Fill, height: Fit
                flow: Down
                align: {x: 0.5}
                spacing: 8

                logo_text = <Label> {
                    text: "COLA"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_BOLD>{ font_size: 32.0 }
                        fn get_color(self) -> vec4 {
                            return (ACCENT_PRIMARY);
                        }
                    }
                }

                subtitle = <Label> {
                    text: "智能英语学习助手"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 14.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }

            // Auth card
            auth_card = <RoundedView> {
                width: Fill, height: Fit
                flow: Down
                show_bg: true
                draw_bg: {
                    instance dark_mode: 0.0
                    instance border_radius: 12.0
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);
                        let bg = mix((WHITE), (SLATE_800), self.dark_mode);
                        sdf.fill(bg);
                        return sdf.result;
                    }
                }

                // Tabs
                tabs = <View> {
                    width: Fill, height: Fit
                    flow: Right

                    login_tab = <AuthTab> {
                        text: "登录"
                        draw_bg: { active: 1.0 }
                        draw_text: { active: 1.0 }
                    }

                    register_tab = <AuthTab> {
                        text: "注册"
                    }
                }

                // Divider
                <View> {
                    width: Fill, height: 1
                    show_bg: true
                    draw_bg: {
                        instance dark_mode: 0.0
                        fn pixel(self) -> vec4 {
                            return mix((DIVIDER), (DIVIDER_DARK), self.dark_mode);
                        }
                    }
                }

                // Form content
                form_content = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    padding: 24
                    spacing: 16

                    // Login form
                    login_form = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 16

                        email_field = <FormField> {
                            label = { text: "邮箱" }
                            input = { text: "请输入邮箱地址" }
                        }

                        password_field = <FormField> {
                            label = { text: "密码" }
                            input = { text: "请输入密码" }
                        }

                        forgot_password = <View> {
                            width: Fill, height: Fit
                            align: {x: 1.0}

                            link = <Label> {
                                text: "忘记密码？"
                                draw_text: {
                                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                                    color: (ACCENT_PRIMARY)
                                }
                            }
                        }

                        login_btn = <FormButton> {
                            text: "登录"
                        }
                    }

                    // Register form (hidden by default)
                    register_form = <View> {
                        width: Fill, height: Fit
                        visible: false
                        flow: Down
                        spacing: 16

                        name_field = <FormField> {
                            label = { text: "用户名" }
                            input = { text: "请输入用户名" }
                        }

                        email_field = <FormField> {
                            label = { text: "邮箱" }
                            input = { text: "请输入邮箱地址" }
                        }

                        password_field = <FormField> {
                            label = { text: "密码" }
                            input = { text: "请输入密码（至少6位）" }
                        }

                        confirm_password_field = <FormField> {
                            label = { text: "确认密码" }
                            input = { text: "请再次输入密码" }
                        }

                        register_btn = <FormButton> {
                            text: "注册"
                        }
                    }

                    // OAuth section
                    oauth_section = <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: 16

                        <DividerWithText> {}

                        oauth_buttons = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 12

                            google_btn = <OAuthButton> { text: "使用 Google 账号继续" }
                            wechat_btn = <OAuthButton> { text: "使用微信登录" }
                        }
                    }
                }
            }

            // Terms
            terms = <View> {
                width: Fill, height: Fit
                align: {x: 0.5}

                terms_text = <Label> {
                    text: "登录即表示同意用户协议和隐私政策"
                    draw_text: {
                        instance dark_mode: 0.0
                        text_style: <FONT_REGULAR>{ font_size: 12.0 }
                        fn get_color(self) -> vec4 {
                            return mix((TEXT_MUTED), (TEXT_SECONDARY_DARK), self.dark_mode);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum LoginScreenAction {
    None,
    Login { email: String, password: String },
    Register { name: String, email: String, password: String },
    ForgotPassword,
    OAuthGoogle,
    OAuthWeChat,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AuthTab {
    #[default]
    Login,
    Register,
}

#[derive(Live, LiveHook, Widget)]
pub struct LoginScreen {
    #[deref]
    view: View,
    #[rust]
    current_tab: AuthTab,
}

impl Widget for LoginScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle tab switching
        if self.view.button(ids!(content.auth_card.tabs.login_tab)).clicked(actions) {
            self.switch_tab(cx, AuthTab::Login);
        }
        if self.view.button(ids!(content.auth_card.tabs.register_tab)).clicked(actions) {
            self.switch_tab(cx, AuthTab::Register);
        }

        // Handle login button
        if self.view.button(ids!(content.auth_card.form_content.login_form.login_btn)).clicked(actions) {
            let email = self.view.text_input(ids!(content.auth_card.form_content.login_form.email_field.input)).text();
            let password = self.view.text_input(ids!(content.auth_card.form_content.login_form.password_field.input)).text();
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                LoginScreenAction::Login { email, password },
            );
        }

        // Handle register button
        if self.view.button(ids!(content.auth_card.form_content.register_form.register_btn)).clicked(actions) {
            let name = self.view.text_input(ids!(content.auth_card.form_content.register_form.name_field.input)).text();
            let email = self.view.text_input(ids!(content.auth_card.form_content.register_form.email_field.input)).text();
            let password = self.view.text_input(ids!(content.auth_card.form_content.register_form.password_field.input)).text();
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                LoginScreenAction::Register { name, email, password },
            );
        }

        // Handle OAuth buttons
        if self.view.button(ids!(content.auth_card.form_content.oauth_section.oauth_buttons.google_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, LoginScreenAction::OAuthGoogle);
        }
        if self.view.button(ids!(content.auth_card.form_content.oauth_section.oauth_buttons.wechat_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, LoginScreenAction::OAuthWeChat);
        }

        // Handle forgot password link click
        self.handle_forgot_password(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl LoginScreen {
    fn switch_tab(&mut self, cx: &mut Cx, tab: AuthTab) {
        self.current_tab = tab;

        let (login_active, register_active) = match tab {
            AuthTab::Login => (1.0, 0.0),
            AuthTab::Register => (0.0, 1.0),
        };

        // Update tab styles
        self.view.button(ids!(content.auth_card.tabs.login_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (login_active) }
                draw_text: { active: (login_active) }
            },
        );
        self.view.button(ids!(content.auth_card.tabs.register_tab)).apply_over(
            cx,
            live! {
                draw_bg: { active: (register_active) }
                draw_text: { active: (register_active) }
            },
        );

        // Toggle form visibility
        self.view.view(ids!(content.auth_card.form_content.login_form))
            .set_visible(cx, tab == AuthTab::Login);
        self.view.view(ids!(content.auth_card.form_content.register_form))
            .set_visible(cx, tab == AuthTab::Register);

        self.view.redraw(cx);
    }

    fn handle_forgot_password(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let forgot_link = self.view.view(ids!(content.auth_card.form_content.login_form.forgot_password));
        if let Hit::FingerUp(_) = event.hits(cx, forgot_link.area()) {
            cx.widget_action(self.widget_uid(), &scope.path, LoginScreenAction::ForgotPassword);
        }
    }
}
