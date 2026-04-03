mod db;
mod models;
mod handlers;
mod config;
mod templates;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::init_db;
use crate::handlers::{
    auth, pages, api
};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "katelyatv_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = AppConfig::load().expect("Failed to load configuration");
    
    let port = config.port;
    let host = config.host.clone();

    // 初始化数据库
    let pool = init_db(&config.database_url)
        .await
        .expect("Failed to initialize database");

    // 创建应用状态
    let app_state = AppState {
        config,
        db: pool,
    };

    // 构建路由
    let app = Router::new()
        // API 路由
        .nest("/api", api::routes(app_state.clone()))
        // 页面路由
        .merge(pages::routes(app_state.clone()))
        // 静态文件服务
        .nest_service("/static", ServeDir::new("static"));

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Starting KatelyaTV Rust server on http://{}:{}", host, port);
    tracing::info!("Admin registration is open for the first user");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: sqlx::SqlitePool,
}
