# KatelyaTV Rust

KatelyaTV 的 Rust实现版本 - 自托管影视聚合播放器

## 功能特性

### 核心功能
- ✅ **聚合搜索**：整合多个影视资源站，一键搜索全网内容
- ✅ **高清播放**：支持多种视频格式和播放源
- ✅ **智能跳过**：支持手动设置跳过片头片尾时间段
- ✅ **断点续播**：自动记录播放进度，跨设备同步观看位置
- ✅ **响应式设计**：完美适配手机、平板、电脑各种屏幕

### 用户系统
- ✅ **多用户支持**：独立的用户系统，每个用户独享个人数据
- ✅ **首个用户自动管理员**：第一个注册的用户自动成为管理员
- ✅ **注册开关控制**：首个用户注册后默认关闭注册，管理员可手动开启
- ✅ **收藏功能**：收藏喜欢的影视作品
- ✅ **播放历史**：自动记录观看历史，快速找回看过的内容

### 管理功能
- ✅ **视频源管理**：添加、编辑、删除视频源
- ✅ **注册控制**：管理员可手动开启/关闭注册
- ✅ **TVBox 兼容**：支持 TVBox 配置接口

### 技术特性
- ✅ **SQLite 数据库**：轻量级嵌入式数据库，无需额外服务
- ✅ **单文件部署**：编译后为单个静态二进制文件
- ✅ **端口可配置**：默认 3000，可通过参数修改
- ✅ **Linux AMD64 Musl**：完全静态编译，无外部依赖

## 快速开始

### 下载预编译二进制

从 [GitHub Releases](https://github.com/yourusername/katelyatv-rust/releases) 下载最新版本的 `katelyatv-rust-linux-amd64-musl.tar.gz`

### 运行

```bash
# 解压
tar -xzf katelyatv-rust-linux-amd64-musl.tar.gz

# 运行（默认端口 3000）
./katelyatv-rust

# 自定义端口
./katelyatv-rust --port 8080

# 自定义数据库路径
./katelyatv-rust --database-url ./data/katelyatv.db

# 查看所有选项
./katelyatv-rust --help
```

### 首次使用

1. 访问 `http://localhost:3000`
2. 点击"注册"创建第一个账号（自动成为管理员）
3. 注册完成后，注册功能将自动关闭
4. 登录后可在管理面板添加视频源

## 配置选项

```
--host <HOST>              服务器监听地址 [default: 0.0.0.0]
-p, --port <PORT>          服务器端口 [default: 3000]
--database-url <DATABASE>  数据库文件路径 [default: ./katelyatv.db]
--jwt-secret <SECRET>      JWT 密钥（不设置则自动生成）
--enable-register          启用注册（默认 false）
--admin-password <PASSWORD> 管理员密码（用于重置）
-h, --help                 显示帮助信息
```

## 自行编译

### 本地编译（Linux）

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 musl 工具链
sudo apt-get install musl-tools

# 克隆项目
git clone https://github.com/yourusername/katelyatv-rust.git
cd katelyatv-rust

# 编译 release 版本
cargo build --release --target x86_64-unknown-linux-musl

# 二进制文件位于
ls -lh target/x86_64-unknown-linux-musl/release/katelyatv-rust
```

### 使用 GitHub Actions 编译

项目已配置 GitHub Actions 工作流，推送 tag 即可自动编译：

```bash
# 打标签并推送
git tag v0.1.0
git push origin v0.1.0
```

Actions 会自动编译并创建 Release，附带编译好的二进制文件。

## API 接口

### 认证相关
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/register` - 用户注册
- `GET /api/auth/me` - 获取当前用户信息

### 视频源管理
- `GET /api/sources` - 获取所有视频源
- `POST /api/sources` - 创建视频源
- `GET /api/sources/:id` - 获取单个视频源
- `PUT /api/sources/:id` - 更新视频源
- `DELETE /api/sources/:id` - 删除视频源

### TVBox 配置
- `GET /api/tvbox?format=json` - 获取 JSON 格式配置
- `GET /api/tvbox?format=base64` - 获取 Base64 格式配置

### 设置管理
- `PUT /api/settings/register` - 设置注册开关（需管理员）

## 数据库结构

项目使用 SQLite 数据库，包含以下表：

- `users` - 用户表
- `video_sources` - 视频源表
- `play_history` - 播放历史表
- `favorites` - 收藏表
- `settings` - 系统设置表

## 安全说明

1. **JWT 密钥**：生产环境建议通过 `--jwt-secret` 参数设置固定的 JWT 密钥
2. **HTTPS**：建议在反向代理（如 Nginx）后运行，启用 HTTPS
3. **防火墙**：确保只开放必要的端口
4. **定期备份**：定期备份 SQLite 数据库文件

## Nginx 反向代理配置示例

```nginx
server {
    listen 443 ssl;
    server_name your-domain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Systemd 服务配置

创建 `/etc/systemd/system/katelyatv-rust.service`：

```ini
[Unit]
Description=KatelyaTV Rust Server
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/katelyatv-rust
ExecStart=/opt/katelyatv-rust/katelyatv-rust --port 3000
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable katelyatv-rust
sudo systemctl start katelyatv-rust
```

## 开发计划

- [ ] 视频播放功能完整实现
- [ ] CMS API 对接
- [ ] 片头片尾自动检测
- [ ] 豆瓣 API 集成
- [ ] 更多主题支持
- [ ] PWA 支持

## 技术栈

- **后端框架**: Axum
- **数据库**: SQLite (sqlx)
- **模板引擎**: Askama
- **认证**: JWT (jwt-simple) + Argon2 密码哈希
- **命令行解析**: Clap

## 许可证

MIT License

## 致谢

本项目灵感来源于 [KatelyaTV](https://github.com/haogege8888/KatelyaTV)
