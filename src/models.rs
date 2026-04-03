use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 用户模型
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 视频源模型
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct VideoSource {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub api_type: String, // cms, custom 等
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 播放历史模型
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PlayHistory {
    pub id: i64,
    pub user_id: i64,
    pub video_id: String,
    pub video_title: String,
    pub video_url: String,
    pub source_id: i64,
    pub progress: i64, // 播放进度（秒）
    pub duration: i64, // 总时长（秒）
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 收藏模型
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Favorite {
    pub id: i64,
    pub user_id: i64,
    pub video_id: String,
    pub video_title: String,
    pub video_url: String,
    pub source_id: i64,
    pub created_at: DateTime<Utc>,
}

/// API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.to_string()),
        }
    }
}

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

/// Token 响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            is_admin: user.is_admin,
        }
    }
}

/// 视频源请求
#[derive(Debug, Deserialize)]
pub struct VideoSourceRequest {
    pub name: String,
    pub url: String,
    pub api_type: String,
    pub is_active: bool,
    pub sort_order: i32,
}

/// TVBox 配置响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TvBoxConfig {
    pub sites: Vec<TvBoxSite>,
    pub parses: Vec<TvBoxParse>,
    pub flags: Vec<String>,
    pub ads: Vec<String>,
    pub wallpaper: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvBoxSite {
    pub key: String,
    pub name: String,
    pub r#type: i32,
    pub api: String,
    pub searchable: i32,
    pub changeable: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvBoxParse {
    pub name: String,
    pub r#type: i32,
    pub url: String,
}
