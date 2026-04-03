use sqlx::{sqlite::SqlitePool, Pool};
use tracing::info;

/// 初始化数据库连接池并创建表
pub async fn init_db(database_url: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    info!("Initializing database connection to {}", database_url);
    
    // 创建连接池
    let pool = SqlitePool::connect(database_url).await?;
    
    // 创建表结构
    create_tables(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(pool)
}

async fn create_tables(pool: &Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    // 用户表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL DEFAULT FALSE,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 视频源表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS video_sources (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            api_type TEXT NOT NULL DEFAULT 'cms',
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 播放历史表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS play_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            video_id TEXT NOT NULL,
            video_title TEXT NOT NULL,
            video_url TEXT NOT NULL,
            source_id INTEGER NOT NULL,
            progress INTEGER NOT NULL DEFAULT 0,
            duration INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (source_id) REFERENCES video_sources(id),
            UNIQUE(user_id, video_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 收藏表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS favorites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            video_id TEXT NOT NULL,
            video_title TEXT NOT NULL,
            video_url TEXT NOT NULL,
            source_id INTEGER NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (source_id) REFERENCES video_sources(id),
            UNIQUE(user_id, video_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 系统设置表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            description TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 初始化默认设置：注册开关（首个用户注册后自动关闭）
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO settings (key, value, description) 
        VALUES ('enable_register', 'false', '是否允许用户注册')
        "#,
    )
    .execute(pool)
    .await?;

    info!("Database tables created successfully");
    Ok(())
}

/// 检查是否是第一个用户（用于管理员自动授权）
pub async fn is_first_user(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(count.0 == 0)
}

/// 获取注册开关状态
pub async fn get_register_setting(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT value FROM settings WHERE key = 'enable_register'",
    )
    .fetch_optional(pool)
    .await?;
    
    match result {
        Some((value,)) => Ok(value == "true"),
        None => Ok(false), // 默认关闭注册
    }
}

/// 设置注册开关
pub async fn set_register_setting(pool: &SqlitePool, enable: bool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO settings (key, value, description, updated_at) 
        VALUES ('enable_register', ?, '是否允许用户注册', CURRENT_TIMESTAMP)
        "#,
    )
    .bind(if enable { "true" } else { "false" })
    .execute(pool)
    .await?;
    Ok(())
}
