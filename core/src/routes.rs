//! Route definitions for Colang Desktop
//!
//! This module defines all application routes and provides utility functions
//! for route management.

use colang_widgets::router::Route;

/// All application route paths as constants
pub mod paths {
    pub const HOME: &str = "/";
    pub const CHAT: &str = "/chat";
    pub const REVIEW: &str = "/review";
    pub const REVIEW_DUE: &str = "/review/due";
    pub const REVIEW_STATS: &str = "/review/stats";
    pub const REVIEW_MISTAKES: &str = "/review/mistakes";
    pub const REVIEW_MASTERED: &str = "/review/mastered";
    pub const REVIEW_PRACTICE: &str = "/review/practice";
    pub const SCENES: &str = "/scenes";
    pub const READING: &str = "/reading";
    pub const DICTIONARY: &str = "/dictionary";
    pub const SETTINGS: &str = "/settings";
    pub const SETTINGS_GENERAL: &str = "/settings/general";
    pub const SETTINGS_AUDIO: &str = "/settings/audio";
    pub const SETTINGS_PROVIDERS: &str = "/settings/providers";
    pub const SETTINGS_ABOUT: &str = "/settings/about";
    // New pages
    pub const LOGIN: &str = "/login";
    pub const FAVORITES: &str = "/favorites";
    pub const PROFILE: &str = "/profile";
    pub const SUBSCRIPTION: &str = "/subscription";
    pub const FEEDBACK: &str = "/feedback";
}

/// Route identifiers matching screen widget ids
pub mod page_ids {
    use makepad_widgets::live_id;
    use makepad_widgets::LiveId;

    pub fn home_screen() -> LiveId {
        live_id!(home_screen)
    }
    pub fn chat_screen() -> LiveId {
        live_id!(chat_screen)
    }
    pub fn review_screen() -> LiveId {
        live_id!(review_screen)
    }
    pub fn scenes_screen() -> LiveId {
        live_id!(scenes_screen)
    }
    pub fn reading_screen() -> LiveId {
        live_id!(reading_screen)
    }
    pub fn dictionary_screen() -> LiveId {
        live_id!(dictionary_screen)
    }
    pub fn settings_screen() -> LiveId {
        live_id!(settings_screen)
    }
    // New pages
    pub fn login_screen() -> LiveId {
        live_id!(login_screen)
    }
    pub fn favorites_screen() -> LiveId {
        live_id!(favorites_screen)
    }
    pub fn profile_screen() -> LiveId {
        live_id!(profile_screen)
    }
    pub fn subscription_screen() -> LiveId {
        live_id!(subscription_screen)
    }
    pub fn feedback_screen() -> LiveId {
        live_id!(feedback_screen)
    }
}

/// Page metadata for header display
#[derive(Clone, Debug)]
pub struct PageMeta {
    pub icon: &'static str,
    pub title: &'static str,
}

/// Get page metadata for a given path
pub fn get_page_meta(path: &str) -> Option<PageMeta> {
    match path {
        paths::HOME => Some(PageMeta {
            icon: "🏠",
            title: "首页",
        }),
        paths::CHAT => Some(PageMeta {
            icon: "💬",
            title: "日常唠嗑",
        }),
        paths::REVIEW | paths::REVIEW_DUE | paths::REVIEW_STATS | paths::REVIEW_MISTAKES | paths::REVIEW_MASTERED | paths::REVIEW_PRACTICE => {
            Some(PageMeta {
                icon: "📚",
                title: "温故知新",
            })
        }
        paths::SCENES => Some(PageMeta {
            icon: "🎭",
            title: "角色扮演",
        }),
        paths::READING => Some(PageMeta {
            icon: "🎤",
            title: "大声跟读",
        }),
        paths::DICTIONARY => Some(PageMeta {
            icon: "📖",
            title: "词典翻译",
        }),
        paths::SETTINGS | paths::SETTINGS_GENERAL | paths::SETTINGS_AUDIO | paths::SETTINGS_PROVIDERS | paths::SETTINGS_ABOUT => {
            Some(PageMeta {
                icon: "⚙️",
                title: "设置",
            })
        }
        // New pages
        paths::LOGIN => Some(PageMeta {
            icon: "🔐",
            title: "登录",
        }),
        paths::FAVORITES => Some(PageMeta {
            icon: "⭐",
            title: "我的收藏",
        }),
        paths::PROFILE => Some(PageMeta {
            icon: "👤",
            title: "个人中心",
        }),
        paths::SUBSCRIPTION => Some(PageMeta {
            icon: "💳",
            title: "订阅管理",
        }),
        paths::FEEDBACK => Some(PageMeta {
            icon: "📬",
            title: "意见反馈",
        }),
        _ => None,
    }
}

/// Create all application routes
pub fn create_routes() -> Vec<Route> {
    vec![
        // Main routes
        Route::new(paths::HOME, page_ids::home_screen())
            .with_title("首页")
            .with_icon("🏠"),
        Route::new(paths::CHAT, page_ids::chat_screen())
            .with_title("日常唠嗑")
            .with_icon("💬"),
        Route::new(paths::REVIEW, page_ids::review_screen())
            .with_title("温故知新")
            .with_icon("📚"),
        Route::new(paths::REVIEW_DUE, page_ids::review_screen())
            .with_title("待复习")
            .with_icon("📚"),
        Route::new(paths::REVIEW_STATS, page_ids::review_screen())
            .with_title("学习统计")
            .with_icon("📊"),
        Route::new(paths::REVIEW_MISTAKES, page_ids::review_screen())
            .with_title("错题本")
            .with_icon("📝"),
        Route::new(paths::REVIEW_MASTERED, page_ids::review_screen())
            .with_title("已掌握")
            .with_icon("✅"),
        Route::new(paths::REVIEW_PRACTICE, page_ids::review_screen())
            .with_title("练习")
            .with_icon("✏️"),
        Route::new(paths::SCENES, page_ids::scenes_screen())
            .with_title("角色扮演")
            .with_icon("🎭"),
        Route::new(paths::READING, page_ids::reading_screen())
            .with_title("大声跟读")
            .with_icon("🎤"),
        Route::new(paths::DICTIONARY, page_ids::dictionary_screen())
            .with_title("词典翻译")
            .with_icon("📖"),
        Route::new(paths::SETTINGS, page_ids::settings_screen())
            .with_title("设置")
            .with_icon("⚙️"),
        Route::new(paths::SETTINGS_GENERAL, page_ids::settings_screen())
            .with_title("通用设置")
            .with_icon("⚙️"),
        Route::new(paths::SETTINGS_AUDIO, page_ids::settings_screen())
            .with_title("音频设置")
            .with_icon("🔊"),
        Route::new(paths::SETTINGS_PROVIDERS, page_ids::settings_screen())
            .with_title("服务商")
            .with_icon("🔌"),
        Route::new(paths::SETTINGS_ABOUT, page_ids::settings_screen())
            .with_title("关于")
            .with_icon("ℹ️"),
        // New pages
        Route::new(paths::LOGIN, page_ids::login_screen())
            .with_title("登录")
            .with_icon("🔐"),
        Route::new(paths::FAVORITES, page_ids::favorites_screen())
            .with_title("我的收藏")
            .with_icon("⭐"),
        Route::new(paths::PROFILE, page_ids::profile_screen())
            .with_title("个人中心")
            .with_icon("👤"),
        Route::new(paths::SUBSCRIPTION, page_ids::subscription_screen())
            .with_title("订阅管理")
            .with_icon("💳"),
        Route::new(paths::FEEDBACK, page_ids::feedback_screen())
            .with_title("意见反馈")
            .with_icon("📬"),
    ]
}

/// Sidebar selection enum matching routes
#[derive(Clone, PartialEq, Debug)]
pub enum SidebarRoute {
    Home,
    Chat,
    Review,
    Scenes,
    Reading,
    Dictionary,
    Favorites,
    Profile,
    Subscription,
    Feedback,
    Settings,
}

impl SidebarRoute {
    /// Get the route path for this sidebar item
    pub fn path(&self) -> &'static str {
        match self {
            SidebarRoute::Home => paths::HOME,
            SidebarRoute::Chat => paths::CHAT,
            SidebarRoute::Review => paths::REVIEW,
            SidebarRoute::Scenes => paths::SCENES,
            SidebarRoute::Reading => paths::READING,
            SidebarRoute::Dictionary => paths::DICTIONARY,
            SidebarRoute::Favorites => paths::FAVORITES,
            SidebarRoute::Profile => paths::PROFILE,
            SidebarRoute::Subscription => paths::SUBSCRIPTION,
            SidebarRoute::Feedback => paths::FEEDBACK,
            SidebarRoute::Settings => paths::SETTINGS,
        }
    }

    /// Get the sidebar route from a path
    pub fn from_path(path: &str) -> Option<Self> {
        match path {
            paths::HOME => Some(SidebarRoute::Home),
            p if p.starts_with("/chat") => Some(SidebarRoute::Chat),
            p if p.starts_with("/review") => Some(SidebarRoute::Review),
            p if p.starts_with("/scenes") => Some(SidebarRoute::Scenes),
            p if p.starts_with("/reading") => Some(SidebarRoute::Reading),
            p if p.starts_with("/dictionary") => Some(SidebarRoute::Dictionary),
            p if p.starts_with("/favorites") => Some(SidebarRoute::Favorites),
            p if p.starts_with("/profile") => Some(SidebarRoute::Profile),
            p if p.starts_with("/subscription") => Some(SidebarRoute::Subscription),
            p if p.starts_with("/feedback") => Some(SidebarRoute::Feedback),
            p if p.starts_with("/settings") => Some(SidebarRoute::Settings),
            _ => None,
        }
    }
}
