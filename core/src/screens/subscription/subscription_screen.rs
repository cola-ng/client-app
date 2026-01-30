//! # Subscription Screen
//!
//! Subscription plans display with wallet balance and purchase options.

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use colang_widgets::theme::*;

    // Plan card component
    PlanCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 24
        flow: Down
        spacing: 16
        show_bg: true
        draw_bg: {
            instance dark_mode: 0.0
            instance featured: 0.0
            border_radius: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 12.0);

                let normal_bg = mix((WHITE), (SLATE_800), self.dark_mode);
                let featured_bg = mix((ORANGE_50), #7c2d12, self.dark_mode);
                let bg = mix(normal_bg, featured_bg, self.featured);
                sdf.fill(bg);

                // Border
                let normal_border = mix((SLATE_200), (SLATE_700), self.dark_mode);
                let featured_border = (ACCENT_PRIMARY);
                let border = mix(normal_border, featured_border, self.featured);
                sdf.stroke(border, mix(1.0, 2.0, self.featured));

                return sdf.result;
            }
        }

        plan_header = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 4

            plan_name = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 18.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }

            plan_desc = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        price_row = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {y: 1.0}
            spacing: 4

            price = <Label> {
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 32.0 }
                    fn get_color(self) -> vec4 {
                        return (ACCENT_PRIMARY);
                    }
                }
            }

            period = <Label> {
                margin: {bottom: 6}
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 14.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        features = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: 8
        }

        subscribe_btn = <Button> {
            width: Fill, height: Fit
            padding: {top: 12, bottom: 12}
            align: {x: 0.5}

            draw_bg: {
                instance dark_mode: 0.0
                instance featured: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);

                    let normal_bg = mix((SLATE_100), (SLATE_700), self.dark_mode);
                    let featured_bg = (ACCENT_PRIMARY);
                    sdf.fill(mix(normal_bg, featured_bg, self.featured));

                    return sdf.result;
                }
            }

            draw_text: {
                instance dark_mode: 0.0
                instance featured: 0.0
                text_style: <FONT_SEMIBOLD>{ font_size: 14.0 }
                fn get_color(self) -> vec4 {
                    let normal = mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    return mix(normal, (WHITE), self.featured);
                }
            }
        }
    }

    // Feature item
    FeatureItem = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 8
        align: {y: 0.5}

        check = <Label> {
            text: "✓"
            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                color: (ACCENT_GREEN)
            }
        }

        text = <Label> {
            draw_text: {
                instance dark_mode: 0.0
                text_style: <FONT_REGULAR>{ font_size: 13.0 }
                fn get_color(self) -> vec4 {
                    return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                }
            }
        }
    }

    // Wallet card
    WalletCard = <RoundedView> {
        width: Fill, height: Fit
        padding: 20
        flow: Right
        align: {y: 0.5}
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

        wallet_icon = <View> {
            width: 48, height: 48
            align: {x: 0.5, y: 0.5}
            show_bg: true
            draw_bg: {
                instance dark_mode: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 10.0);
                    let bg = mix((ORANGE_100), (SLATE_700), self.dark_mode);
                    sdf.fill(bg);
                    return sdf.result;
                }
            }

            emoji = <Label> {
                text: "💰"
                draw_text: {
                    text_style: { font_size: 24.0 }
                }
            }
        }

        wallet_info = <View> {
            width: Fill, height: Fit
            flow: Down
            margin: {left: 16}
            spacing: 2

            label = <Label> {
                text: "账户余额"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_REGULAR>{ font_size: 13.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_SECONDARY), (TEXT_SECONDARY_DARK), self.dark_mode);
                    }
                }
            }

            balance = <Label> {
                text: "¥0.00"
                draw_text: {
                    instance dark_mode: 0.0
                    text_style: <FONT_BOLD>{ font_size: 24.0 }
                    fn get_color(self) -> vec4 {
                        return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                    }
                }
            }
        }

        recharge_btn = <Button> {
            width: Fit, height: Fit
            padding: {left: 16, right: 16, top: 10, bottom: 10}
            text: "充值"

            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 6.0);
                    sdf.fill((ACCENT_PRIMARY));
                    return sdf.result;
                }
            }

            draw_text: {
                text_style: <FONT_MEDIUM>{ font_size: 13.0 }
                color: (WHITE)
            }
        }
    }

    pub SubscriptionScreen = {{SubscriptionScreen}} {
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

                // Header
                header = <View> {
                    width: Fill, height: Fit

                    title = <Label> {
                        text: "订阅管理"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_BOLD>{ font_size: 24.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }
                }

                // Wallet section
                wallet_section = <WalletCard> {}

                // Current plan info
                current_plan = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    section_title = <Label> {
                        text: "当前订阅"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    current_info = <RoundedView> {
                        width: Fill, height: Fit
                        padding: 16
                        flow: Right
                        align: {y: 0.5}
                        show_bg: true
                        draw_bg: {
                            instance dark_mode: 0.0
                            border_radius: 8.0
                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 8.0);
                                let bg = mix(#dcfce7, #14532d, self.dark_mode);
                                sdf.fill(bg);
                                return sdf.result;
                            }
                        }

                        status_icon = <Label> {
                            text: "✓"
                            draw_text: {
                                text_style: <FONT_BOLD>{ font_size: 16.0 }
                                color: (ACCENT_GREEN)
                            }
                        }

                        status_text = <Label> {
                            margin: {left: 8}
                            text: "免费版"
                            draw_text: {
                                instance dark_mode: 0.0
                                text_style: <FONT_MEDIUM>{ font_size: 14.0 }
                                fn get_color(self) -> vec4 {
                                    return mix(#166534, #4ade80, self.dark_mode);
                                }
                            }
                        }
                    }
                }

                // Plans section
                plans_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 12

                    section_title = <Label> {
                        text: "升级计划"
                        draw_text: {
                            instance dark_mode: 0.0
                            text_style: <FONT_SEMIBOLD>{ font_size: 16.0 }
                            fn get_color(self) -> vec4 {
                                return mix((TEXT_PRIMARY), (TEXT_PRIMARY_DARK), self.dark_mode);
                            }
                        }
                    }

                    // Free plan
                    free_plan = <PlanCard> {
                        plan_header = {
                            plan_name = { text: "免费版" }
                            plan_desc = { text: "基础学习功能" }
                        }
                        price_row = {
                            price = { text: "¥0" }
                            period = { text: "/永久" }
                        }
                        features = {
                            <FeatureItem> { text = { text: "每日20个单词" } }
                            <FeatureItem> { text = { text: "基础复习功能" } }
                            <FeatureItem> { text = { text: "学习统计" } }
                        }
                        subscribe_btn = { text: "当前计划" }
                    }

                    // Pro plan
                    pro_plan = <PlanCard> {
                        draw_bg: { featured: 1.0 }

                        plan_header = {
                            plan_name = { text: "专业版" }
                            plan_desc = { text: "解锁全部高级功能" }
                        }
                        price_row = {
                            price = { text: "¥29" }
                            period = { text: "/月" }
                        }
                        features = {
                            <FeatureItem> { text = { text: "无限单词学习" } }
                            <FeatureItem> { text = { text: "AI发音评估" } }
                            <FeatureItem> { text = { text: "智能复习计划" } }
                            <FeatureItem> { text = { text: "详细学习报告" } }
                            <FeatureItem> { text = { text: "离线学习" } }
                        }
                        subscribe_btn = {
                            text: "升级专业版"
                            draw_bg: { featured: 1.0 }
                            draw_text: { featured: 1.0 }
                        }
                    }

                    // Annual plan
                    annual_plan = <PlanCard> {
                        plan_header = {
                            plan_name = { text: "年度会员" }
                            plan_desc = { text: "最优惠的选择" }
                        }
                        price_row = {
                            price = { text: "¥199" }
                            period = { text: "/年 (省¥149)" }
                        }
                        features = {
                            <FeatureItem> { text = { text: "包含专业版全部功能" } }
                            <FeatureItem> { text = { text: "优先客服支持" } }
                            <FeatureItem> { text = { text: "专属学习社区" } }
                        }
                        subscribe_btn = { text: "订阅年度会员" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum SubscriptionScreenAction {
    None,
    Recharge,
    Subscribe { plan: String },
}

#[derive(Live, LiveHook, Widget)]
pub struct SubscriptionScreen {
    #[deref]
    view: View,
}

impl Widget for SubscriptionScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        let actions = match event {
            Event::Actions(actions) => actions.as_slice(),
            _ => return,
        };

        // Handle recharge button
        if self.view.button(ids!(content_scroll.content.wallet_section.recharge_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, SubscriptionScreenAction::Recharge);
        }

        // Handle plan subscribe buttons
        if self.view.button(ids!(content_scroll.content.plans_section.pro_plan.subscribe_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SubscriptionScreenAction::Subscribe { plan: "pro".to_string() },
            );
        }

        if self.view.button(ids!(content_scroll.content.plans_section.annual_plan.subscribe_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                SubscriptionScreenAction::Subscribe { plan: "annual".to_string() },
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
