use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use askama::Template;

use crate::AppState;

/// 首页模板
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub is_logged_in: bool,
    pub username: Option<String>,
    pub is_admin: bool,
}

/// 登录页面模板
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;

/// 注册页面模板
#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub registration_open: bool,
}

/// 管理面板模板
#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminTemplate {
    pub username: String,
}

/// 播放页面模板
#[derive(Template)]
#[template(path = "play.html")]
pub struct PlayTemplate {
    pub video_title: String,
    pub video_url: String,
}

/// 首页路由
pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(index_handler))
        .route("/login", get(login_page))
        .route("/register", get(register_page))
        .route("/admin", get(admin_page))
        .route("/play", get(play_page))
        .route("/search", get(search_page))
        .route("/favorites", get(favorites_page))
        .route("/history", get(history_page))
        .route("/config", get(config_page))
        .route("/about", get(about_page))
}

async fn index_handler(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let template = IndexTemplate {
        is_logged_in: false,
        username: None,
        is_admin: false,
    };
    
    Ok(Html(template.render().unwrap_or_else(|e| {
        tracing::error!("Failed to render template: {}", e);
        "<h1>Error loading page</h1>".to_string()
    })))
}

async fn login_page() -> Result<Html<String>, StatusCode> {
    let template = LoginTemplate;
    Ok(Html(template.render().unwrap()))
}

async fn register_page(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let registration_open = state.config.is_registration_open();
    let template = RegisterTemplate { registration_open };
    Ok(Html(template.render().unwrap()))
}

async fn admin_page() -> Result<Html<String>, StatusCode> {
    let template = AdminTemplate {
        username: "admin".to_string(),
    };
    Ok(Html(template.render().unwrap()))
}

async fn play_page() -> Result<Html<String>, StatusCode> {
    let template = PlayTemplate {
        video_title: "Video Title".to_string(),
        video_url: "".to_string(),
    };
    Ok(Html(template.render().unwrap()))
}

async fn search_page() -> Result<Html<String>, StatusCode> {
    Ok(Html("<h1>Search Page</h1>".to_string()))
}

async fn favorites_page() -> Result<Html<String>, StatusCode> {
    Ok(Html("<h1>Favorites Page</h1>".to_string()))
}

async fn history_page() -> Result<Html<String>, StatusCode> {
    Ok(Html("<h1>History Page</h1>".to_string()))
}

async fn config_page() -> Result<Html<String>, StatusCode> {
    Ok(Html("<h1>TVBox Config Page</h1>".to_string()))
}

async fn about_page() -> Result<Html<String>, StatusCode> {
    Ok(Html("<h1>About Page</h1>".to_string()))
}
