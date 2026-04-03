use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    handlers::auth,
    models::{ApiResponse, LoginRequest, RegisterRequest, VideoSource, VideoSourceRequest},
    AppState,
};

/// API 路由集合
pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        // 认证相关
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        .route("/auth/me", get(auth::get_current_user))
        // 视频源管理
        .route("/sources", get(list_sources))
        .route("/sources", post(create_source))
        .route("/sources/:id", get(get_source))
        .route("/sources/:id", put(update_source))
        .route("/sources/:id", delete(delete_source))
        // TVBox 配置
        .route("/tvbox", get(tvbox_config))
        // 设置管理
        .route("/settings/register", put(set_registration))
}

/// 列出所有视频源
async fn list_sources(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<VideoSource>>>, StatusCode> {
    let sources = sqlx::query_as::<_, VideoSource>("SELECT * FROM video_sources ORDER BY sort_order")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(ApiResponse::success(sources)))
}

/// 获取单个视频源
async fn get_source(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<VideoSource>>, StatusCode> {
    let source = sqlx::query_as::<_, VideoSource>("SELECT * FROM video_sources WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match source {
        Some(s) => Ok(Json(ApiResponse::success(s))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 创建视频源
async fn create_source(
    State(state): State<AppState>,
    Json(payload): Json<VideoSourceRequest>,
) -> Result<Json<ApiResponse<VideoSource>>, StatusCode> {
    let source = sqlx::query_as::<_, VideoSource>(
        r#"
        INSERT INTO video_sources (name, url, api_type, is_active, sort_order)
        VALUES (?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.url)
    .bind(&payload.api_type)
    .bind(payload.is_active)
    .bind(payload.sort_order)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(ApiResponse::success(source)))
}

/// 更新视频源
async fn update_source(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<VideoSourceRequest>,
) -> Result<Json<ApiResponse<VideoSource>>, StatusCode> {
    let source = sqlx::query_as::<_, VideoSource>(
        r#"
        UPDATE video_sources
        SET name = ?, url = ?, api_type = ?, is_active = ?, sort_order = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.url)
    .bind(&payload.api_type)
    .bind(payload.is_active)
    .bind(payload.sort_order)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match source {
        Some(s) => Ok(Json(ApiResponse::success(s))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 删除视频源
async fn delete_source(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let result = sqlx::query("DELETE FROM video_sources WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if result.rows_affected() > 0 {
        Ok(Json(ApiResponse::success(())))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// TVBox 配置接口
#[derive(Debug, Deserialize)]
struct TvBoxQuery {
    format: Option<String>,
}

#[derive(Serialize)]
struct TvBoxConfig {
    sites: Vec<TvBoxSite>,
    parses: Vec<TvBoxParse>,
    flags: Vec<String>,
    ads: Vec<String>,
    wallpaper: String,
}

#[derive(Serialize)]
struct TvBoxSite {
    key: String,
    name: String,
    #[serde(rename = "type")]
    type_: i32,
    api: String,
    searchable: i32,
    changeable: i32,
}

#[derive(Serialize)]
struct TvBoxParse {
    name: String,
    #[serde(rename = "type")]
    type_: i32,
    url: String,
}

async fn tvbox_config(
    State(state): State<AppState>,
    query: Query<TvBoxQuery>,
) -> Result<String, StatusCode> {
    let sources = sqlx::query_as::<_, VideoSource>("SELECT * FROM video_sources WHERE is_active = true")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let sites: Vec<TvBoxSite> = sources
        .iter()
        .map(|s| TvBoxSite {
            key: format!("source_{}", s.id),
            name: s.name.clone(),
            type_: if s.api_type == "cms" { 1 } else { 0 },
            api: s.url.clone(),
            searchable: 1,
            changeable: 1,
        })
        .collect();
    
    let config = TvBoxConfig {
        sites,
        parses: vec![TvBoxParse {
            name: "内置解析".to_string(),
            type_: 0,
            url: format!("{}/api/parse", state.config.host),
        }],
        flags: vec!["youku".to_string(), "qq".to_string(), "iqiyi".to_string()],
        ads: vec![],
        wallpaper: "https://picsum.photos/1920/1080".to_string(),
    };
    
    // 根据格式参数返回 JSON 或 Base64
    match query.format.as_deref() {
        Some("base64") => {
            let json = serde_json::to_string(&config).unwrap();
            Ok(base64_encode(&json))
        }
        _ => {
            let json = serde_json::to_string_pretty(&config).unwrap();
            Ok(json)
        }
    }
}

/// 简单的 Base64 编码（使用 base64 crate）
fn base64_encode(input: &str) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(input.as_bytes())
}

/// 设置注册开关（仅管理员）
async fn set_registration(
    State(state): State<AppState>,
    Json(payload): Json<SetRegistrationRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // TODO: 添加权限验证
    
    crate::db::set_register_setting(&state.db, payload.enable)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(ApiResponse::success(())))
}

#[derive(Deserialize)]
struct SetRegistrationRequest {
    enable: bool,
}

// 辅助函数：POST 路由宏
macro_rules! post {
    ($handler:expr) => {
        axum::routing::post($handler)
    };
}

use post;
