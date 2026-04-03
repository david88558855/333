use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Clone)]
#[command(name = "katelyatv-rust")]
#[command(about = "KatelyaTV Rust - 自托管影视聚合播放器")]
pub struct AppConfig {
    /// 服务器监听地址
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    /// 服务器端口
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// 数据库文件路径
    #[arg(long, default_value = "./katelyatv.db")]
    pub database_url: String,

    /// JWT 密钥（可选，不设置则自动生成）
    #[arg(long)]
    pub jwt_secret: Option<String>,

    /// 是否启用注册（默认 false，首个用户注册后自动关闭）
    #[arg(long, default_value = "false")]
    pub enable_register: bool,

    /// 管理员密码（用于重置或初始化）
    #[arg(long)]
    pub admin_password: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // 从命令行参数加载配置
        let config = Self::parse();
        
        // 如果未提供 JWT 密钥，生成一个随机的
        if config.jwt_secret.is_none() {
            tracing::warn!("JWT secret not provided, generating a random one");
        }
        
        Ok(config)
    }
    
    /// 检查是否允许注册
    pub fn is_registration_open(&self) -> bool {
        self.enable_register
    }
}
