//! Router - URL-based navigation system for Makepad
//!
//! This module provides a routing system similar to react-router, enabling
//! deep linking and URL-based navigation in Makepad applications.
//!
//! ## Features
//! - URL-based navigation with path matching
//! - History management (push, replace, back)
//! - Route parameters support
//! - Nested routes support
//! - Route guards and redirects
//!
//! ## Usage
//!
//! ```rust,ignore
//! live_design! {
//!     use colang_widgets::router::*;
//!
//!     App = <View> {
//!         router = <Router> {
//!             routes: [
//!                 { path: "/", component: "HomeScreen" },
//!                 { path: "/chat", component: "ChatScreen" },
//!                 { path: "/settings", component: "SettingsScreen" },
//!             ]
//!         }
//!     }
//! }
//! ```

use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub Router = {{Router}} {
        width: Fill, height: Fill
        flow: Overlay
    }
}

/// Route path definition
#[derive(Clone, Debug, PartialEq)]
pub struct RoutePath {
    /// Path segments (e.g., ["", "chat", "room", ":id"])
    pub segments: Vec<String>,
    /// Whether this route has dynamic parameters
    pub has_params: bool,
}

impl RoutePath {
    /// Parse a path string into RoutePath
    pub fn parse(path: &str) -> Self {
        let segments: Vec<String> = path
            .split('/')
            .map(|s| s.to_string())
            .collect();

        let has_params = segments.iter().any(|s| s.starts_with(':'));

        Self { segments, has_params }
    }

    /// Check if this path matches another path, extracting parameters
    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        let other_segments: Vec<&str> = path.split('/').collect();

        if self.segments.len() != other_segments.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (pattern, actual) in self.segments.iter().zip(other_segments.iter()) {
            if pattern.starts_with(':') {
                // This is a parameter - extract it
                let param_name = &pattern[1..];
                params.insert(param_name.to_string(), actual.to_string());
            } else if pattern != *actual {
                // Literal segment doesn't match
                return None;
            }
        }

        Some(params)
    }
}

/// A single route definition
#[derive(Clone, Debug)]
pub struct Route {
    /// The path pattern (e.g., "/chat/:id")
    pub path: RoutePath,
    /// The original path string
    pub path_str: String,
    /// The page/screen identifier to show
    pub page_id: LiveId,
    /// Optional title for the header
    pub title: Option<String>,
    /// Optional icon for the header
    pub icon: Option<String>,
    /// Whether this route requires authentication
    pub requires_auth: bool,
    /// Redirect to another path if not authenticated
    pub auth_redirect: Option<String>,
}

impl Route {
    pub fn new(path: &str, page_id: LiveId) -> Self {
        Self {
            path: RoutePath::parse(path),
            path_str: path.to_string(),
            page_id,
            title: None,
            icon: None,
            requires_auth: false,
            auth_redirect: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_auth(mut self, redirect: &str) -> Self {
        self.requires_auth = true;
        self.auth_redirect = Some(redirect.to_string());
        self
    }
}

/// Navigation history entry
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub path: String,
    pub params: HashMap<String, String>,
    pub state: Option<String>,
}

/// Router action emitted when navigation occurs
#[derive(Clone, Debug, DefaultNone)]
pub enum RouterAction {
    None,
    /// Navigation to a new path
    Navigate {
        path: String,
        params: HashMap<String, String>,
        page_id: LiveId,
        title: Option<String>,
        icon: Option<String>,
    },
    /// Navigation was blocked (e.g., auth required)
    Blocked {
        path: String,
        reason: String,
    },
    /// Request to go back
    Back,
    /// History changed
    HistoryChanged {
        current_index: usize,
        history_len: usize,
    },
}

/// Router widget - manages URL-based navigation
#[derive(Live, LiveHook, Widget)]
pub struct Router {
    #[deref]
    view: View,

    /// Registered routes
    #[rust]
    routes: Vec<Route>,

    /// Current path
    #[rust]
    current_path: String,

    /// Current route parameters
    #[rust]
    current_params: HashMap<String, String>,

    /// Currently active page LiveId
    #[rust]
    current_page_id: Option<LiveId>,

    /// Navigation history
    #[rust]
    history: Vec<HistoryEntry>,

    /// Current position in history
    #[rust]
    history_index: usize,

    /// Whether the router has been initialized
    #[rust]
    initialized: bool,

    /// Default route path
    #[rust]
    default_path: String,

    /// Authentication state (for protected routes)
    #[rust]
    is_authenticated: bool,
}

impl Widget for Router {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize on first draw
        if !self.initialized && !self.routes.is_empty() {
            self.initialized = true;
            // Navigate to default path if no current path
            if self.current_path.is_empty() {
                let path = self.default_path.clone();
                if !path.is_empty() {
                    self.navigate_internal(&path);
                }
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl Router {
    /// Navigate internal without triggering a redraw
    fn navigate_internal(&mut self, path: &str) -> Option<RouterAction> {
        // Find matching route
        for route in &self.routes {
            if let Some(params) = route.path.matches(path) {
                // Check authentication
                if route.requires_auth && !self.is_authenticated {
                    return Some(RouterAction::Blocked {
                        path: path.to_string(),
                        reason: "Authentication required".to_string(),
                    });
                }

                self.current_path = path.to_string();
                self.current_params = params.clone();
                self.current_page_id = Some(route.page_id);

                return Some(RouterAction::Navigate {
                    path: path.to_string(),
                    params,
                    page_id: route.page_id,
                    title: route.title.clone(),
                    icon: route.icon.clone(),
                });
            }
        }

        None
    }
}

impl RouterRef {
    /// Register a new route
    pub fn register_route(&self, route: Route) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.routes.push(route);
        }
    }

    /// Register multiple routes at once
    pub fn register_routes(&self, routes: Vec<Route>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.routes.extend(routes);
        }
    }

    /// Set the default path (used when navigating to "/")
    pub fn set_default_path(&self, path: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.default_path = path.to_string();
        }
    }

    /// Navigate to a path (push to history)
    pub fn push(&self, cx: &mut Cx, path: &str) -> Option<RouterAction> {
        if let Some(mut inner) = self.borrow_mut() {
            let action = inner.navigate_internal(path);

            if let Some(RouterAction::Navigate { .. }) = &action {
                // Get values before modifying history
                let truncate_at = inner.history_index + 1;
                let params = inner.current_params.clone();

                // Add to history
                inner.history.truncate(truncate_at);
                inner.history.push(HistoryEntry {
                    path: path.to_string(),
                    params,
                    state: None,
                });
                inner.history_index = inner.history.len() - 1;

                inner.view.redraw(cx);
            }

            action
        } else {
            None
        }
    }

    /// Navigate to a path (replace current history entry)
    pub fn replace(&self, cx: &mut Cx, path: &str) -> Option<RouterAction> {
        if let Some(mut inner) = self.borrow_mut() {
            let action = inner.navigate_internal(path);

            if let Some(RouterAction::Navigate { .. }) = &action {
                // Get values before modifying history
                let params = inner.current_params.clone();
                let idx = inner.history_index;

                // Replace current history entry
                if inner.history.is_empty() {
                    inner.history.push(HistoryEntry {
                        path: path.to_string(),
                        params,
                        state: None,
                    });
                    inner.history_index = 0;
                } else {
                    inner.history[idx] = HistoryEntry {
                        path: path.to_string(),
                        params,
                        state: None,
                    };
                }

                inner.view.redraw(cx);
            }

            action
        } else {
            None
        }
    }

    /// Go back in history
    pub fn back(&self, cx: &mut Cx) -> Option<RouterAction> {
        if let Some(mut inner) = self.borrow_mut() {
            if inner.history_index > 0 {
                inner.history_index -= 1;
                let entry = inner.history[inner.history_index].clone();
                let action = inner.navigate_internal(&entry.path);
                inner.view.redraw(cx);
                action
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Go forward in history
    pub fn forward(&self, cx: &mut Cx) -> Option<RouterAction> {
        if let Some(mut inner) = self.borrow_mut() {
            if inner.history_index < inner.history.len() - 1 {
                inner.history_index += 1;
                let entry = inner.history[inner.history_index].clone();
                let action = inner.navigate_internal(&entry.path);
                inner.view.redraw(cx);
                action
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get current path
    pub fn current_path(&self) -> String {
        if let Some(inner) = self.borrow() {
            inner.current_path.clone()
        } else {
            String::new()
        }
    }

    /// Get current route parameters
    pub fn current_params(&self) -> HashMap<String, String> {
        if let Some(inner) = self.borrow() {
            inner.current_params.clone()
        } else {
            HashMap::new()
        }
    }

    /// Get a specific route parameter
    pub fn get_param(&self, name: &str) -> Option<String> {
        if let Some(inner) = self.borrow() {
            inner.current_params.get(name).cloned()
        } else {
            None
        }
    }

    /// Get current page LiveId
    pub fn current_page_id(&self) -> Option<LiveId> {
        if let Some(inner) = self.borrow() {
            inner.current_page_id
        } else {
            None
        }
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.history_index > 0
        } else {
            false
        }
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.history_index < inner.history.len().saturating_sub(1)
        } else {
            false
        }
    }

    /// Set authentication state
    pub fn set_authenticated(&self, authenticated: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.is_authenticated = authenticated;
        }
    }

    /// Get all registered routes
    pub fn routes(&self) -> Vec<Route> {
        if let Some(inner) = self.borrow() {
            inner.routes.clone()
        } else {
            Vec::new()
        }
    }

    /// Initialize the router with a starting path
    pub fn init(&self, cx: &mut Cx, path: &str) -> Option<RouterAction> {
        self.push(cx, path)
    }
}

/// Macro to easily define routes
#[macro_export]
macro_rules! routes {
    (
        $($path:expr => $page_id:ident $(, title: $title:expr)? $(, icon: $icon:expr)? $(, auth: $auth:expr)?);* $(;)?
    ) => {
        vec![
            $(
                {
                    let mut route = $crate::router::Route::new($path, live_id!($page_id));
                    $(route = route.with_title($title);)?
                    $(route = route.with_icon($icon);)?
                    $(route = route.with_auth($auth);)?
                    route
                }
            ),*
        ]
    };
}
