use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{models::{ApiResponse, LoginRequest, RegisterRequest, TokenResponse, UserResponse}, AppState};

/// JWT 密钥（实际应用中应从配置读取）
const JWT_SECRET: &str = "katelyatv-rust-secret-key-change-in-production";

/// 密码哈希验证
pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).ok()?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// 生成 JWT token
pub fn generate_token(user_id: i64, username: &str, is_admin: bool) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims {
        user_id,
        username: username.to_string(),
        is_admin,
        exp: Some((chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as u64),
    };
    
    let token = JWT_SECRET.authenticate().claim(claims).to_token()?;
    Ok(token)
}

/// 验证 JWT token
pub fn verify_token(token: &str) -> Option<Claims> {
    JWT_SECRET.authenticate().verify_token(token, None).ok()
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub is_admin: bool,
    pub exp: Option<u64>,
}

/// 登录处理
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, StatusCode> {
    // 查找用户
    let user = sqlx::query_as::<_, crate::models::User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => {
            // 验证密码
            if !verify_password(&payload.password, &u.password_hash) {
                return Err(StatusCode::UNAUTHORIZED);
            }
            
            // 生成 token
            let token = generate_token(u.id, &u.username, u.is_admin)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Json(ApiResponse::success(TokenResponse {
                token,
                user: UserResponse::from(u),
            })))
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

/// 注册处理
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, (StatusCode, String)> {
    // 检查密码匹配
    if payload.password != payload.confirm_password {
        return Err((StatusCode::BAD_REQUEST, "Passwords do not match".to_string()));
    }
    
    // 检查用户名是否已存在
    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if existing.is_some() {
        return Err((StatusCode::BAD_REQUEST, "Username already exists".to_string()));
    }
    
    // 检查是否允许注册
    let is_first = crate::db::is_first_user(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let registration_open = is_first || state.config.is_registration_open();
    
    if !registration_open {
        return Err((StatusCode::FORBIDDEN, "Registration is currently closed".to_string()));
    }
    
    // 哈希密码
    let password_hash = hash_password(&payload.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // 插入用户（首个用户自动成为管理员）
    let is_admin = is_first;
    
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, ?)"
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .bind(is_admin)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    // 如果是首个用户，自动关闭注册
    if is_first {
        crate::db::set_register_setting(&state.db, false)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        tracing::info!("First user registered as admin, registration now closed");
    }
    
    // 生成 token
    let token = generate_token(result.last_insert_rowid(), &payload.username, is_admin)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(ApiResponse::success(TokenResponse {
        token,
        user: UserResponse {
            id: result.last_insert_rowid(),
            username: payload.username,
            is_admin,
        },
    })))
}

/// 获取当前用户信息
pub async fn get_current_user(
    State(state): State<AppState>,
    headers: http::HeaderMap,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // 从 header 中提取 token
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = auth_header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    
    // 验证 token
    let claims = verify_token(token).ok_or(StatusCode::UNAUTHORIZED)?;
    
    // 查询用户
    let user = sqlx::query_as::<_, crate::models::User>("SELECT * FROM users WHERE id = ?")
        .bind(claims.user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match user {
        Some(u) => Ok(Json(ApiResponse::success(UserResponse::from(u)))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
