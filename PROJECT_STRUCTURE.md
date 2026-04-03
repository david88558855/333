# KatelyaTV Rust 项目结构

## 目录结构

```
katelyatv-rust/
├── .github/
│   └── workflows/
│       └── build.yml              # GitHub Actions 构建工作流
├── src/
│   ├── main.rs                    # 程序入口
│   ├── config.rs                  # 配置管理
│   ├── db.rs                      # 数据库初始化和工具函数
│   ├── models.rs                  # 数据模型定义
│   ├── handlers/
│   │   ├── mod.rs                 # Handlers 模块声明
│   │   ├── auth.rs                # 认证处理器（登录/注册）
│   │   ├── pages.rs               # 页面路由处理器
│   │   └── api.rs                 # API 路由处理器
│   └── templates/
│       └── mod.rs                 # 模板模块声明
├── templates/                     # Askama HTML 模板
│   ├── index.html                 # 首页
│   ├── login.html                 # 登录页
│   ├── register.html              # 注册页
│   ├── admin.html                 # 管理面板
│   ├── play.html                  # 播放页
│   ├── search.html                # 搜索页
│   ├── favorites.html             # 收藏页
│   ├── history.html               # 历史页
│   ├── config.html                # TVBox 配置页
│   └── about.html                 # 关于页
├── static/                        # 静态资源
│   ├── css/
│   │   └── style.css              # 主样式文件
│   └── js/
│       ├── app.js                 # 主应用逻辑
│       ├── auth.js                # 认证相关
│       ├── admin.js               # 管理面板逻辑
│       ├── player.js              # 播放器逻辑
│       ├── search.js              # 搜索逻辑
│       ├── config.js              # TVBox 配置逻辑
│       ├── favorites.js           # 收藏功能
│       └── history.js             # 历史记录功能
├── Cargo.toml                     # Rust 项目配置
├── Cross.toml                     # Cross 编译配置
├── build.sh                       # Linux 构建脚本
├── .gitignore                     # Git 忽略文件
├── config.example.json            # 配置文件示例
├── README.md                      # 项目说明文档
├── DEPLOYMENT.md                  # 部署指南
├── API.md                         # API 文档
└── PROJECT_STRUCTURE.md           # 项目结构说明（本文件）
```

## 核心模块说明

### 1. `src/main.rs` - 程序入口

- 初始化日志系统
- 加载配置
- 初始化数据库
- 设置路由
- 启动 HTTP 服务器

### 2. `src/config.rs` - 配置管理

使用 Clap 库解析命令行参数：
- `--host`: 监听地址（默认 0.0.0.0）
- `--port`: 端口号（默认 3000）
- `--database-url`: SQLite 数据库路径（默认 ./katelyatv.db）
- `--jwt-secret`: JWT 密钥（可选）
- `--enable-register`: 是否启用注册（默认 false）
- `--admin-password`: 管理员密码（用于重置）

### 3. `src/db.rs` - 数据库模块

- 数据库连接池初始化
- 表结构创建（users, video_sources, play_history, favorites, settings）
- 首个用户检测
- 注册开关控制

### 4. `src/models.rs` - 数据模型

定义所有数据结构：
- `User`: 用户模型
- `VideoSource`: 视频源模型
- `PlayHistory`: 播放历史模型
- `Favorite`: 收藏模型
- `ApiResponse<T>`: API 响应包装器
- 各种请求/响应结构体

### 5. `src/handlers/auth.rs` - 认证处理器

实现用户认证功能：
- `login()`: 用户登录
- `register()`: 用户注册（首个用户自动管理员，之后自动关闭注册）
- `get_current_user()`: 获取当前用户信息
- 密码哈希（Argon2）
- JWT Token 生成和验证

### 6. `src/handlers/pages.rs` - 页面路由

提供所有前端页面的 SSR 渲染：
- `/`: 首页
- `/login`: 登录页
- `/register`: 注册页
- `/admin`: 管理面板
- `/play`: 播放页
- `/search`: 搜索页
- `/favorites`: 收藏页
- `/history`: 历史页
- `/config`: TVBox 配置页
- `/about`: 关于页

### 7. `src/handlers/api.rs` - API 路由

提供 RESTful API：
- `/api/auth/*`: 认证相关
- `/api/sources/*`: 视频源管理
- `/api/tvbox`: TVBox 配置接口
- `/api/settings/*`: 系统设置

## 前端架构

### CSS 样式 (`static/css/style.css`)

- 响应式布局（桌面/平板/手机）
- 侧边栏导航
- 视频卡片网格
- 表单样式
- 模态框
- 开关切换组件

### JavaScript 模块

#### `auth.js` - 认证模块
- 登录表单处理
- 注册表单处理
- Token 存储和管理
- 退出登录

#### `app.js` - 主应用
- 登录状态检查
- 搜索功能
- 视频列表加载

#### `admin.js` - 管理面板
- 视频源 CRUD 操作
- 注册开关控制
- 模态框管理

#### `player.js` - 播放器
- 视频信息加载
- 播放源切换
- 剧集选择
- 收藏功能

#### `search.js` - 搜索
- 搜索执行
- 结果渲染
- 播放源过滤

#### `config.js` - TVBox 配置
- 配置 URL 生成
- 复制功能
- 格式切换

#### `favorites.js` - 收藏
- 收藏列表加载
- 收藏管理

#### `history.js` - 历史记录
- 历史记录加载
- 进度显示
- 继续观看

## 数据库设计

### users 表
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
)
```

### video_sources 表
```sql
CREATE TABLE video_sources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    api_type TEXT NOT NULL DEFAULT 'cms',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
)
```

### play_history 表
```sql
CREATE TABLE play_history (
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
```

### favorites 表
```sql
CREATE TABLE favorites (
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
```

### settings 表
```sql
CREATE TABLE settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
)
```

## 技术栈总览

| 类别 | 技术 | 版本 |
|------|------|------|
| **后端框架** | Axum | 0.7 |
| **异步运行时** | Tokio | 1.x |
| **数据库 ORM** | SQLx | 0.7 |
| **数据库** | SQLite | 3.x |
| **模板引擎** | Askama | 0.12 |
| **认证** | jwt-simple | 0.12 |
| **密码哈希** | Argon2 | 0.5 |
| **命令行解析** | Clap | 4.4 |
| **序列化** | Serde + Serde JSON | 1.0 |
| **HTTP 客户端** | Reqwest | 0.11 |
| **日志** | Tracing | 0.1 |
| **时间处理** | Chrono | 0.4 |
| **随机数** | Rand | 0.8 |
| **前端** | Vanilla JS | - |
| **CSS** | Custom CSS | - |

## 构建流程

### GitHub Actions 构建流程

1. **Checkout**: 拉取代码
2. **Install Rust**: 安装 Rust 工具链和 musl 目标
3. **Install musl-tools**: 安装 musl 开发库
4. **Cache**: 缓存 Cargo 依赖
5. **Build**: 编译 release 版本
6. **Verify**: 验证二进制文件
7. **Package**: 打包为 tar.gz
8. **Release**: 创建 GitHub Release（仅 tag 推送时）

### 本地构建流程

运行 `build.sh` 脚本：
1. 检查 Rust 和 musl 工具链
2. 添加 musl 目标（如未添加）
3. 清理之前的构建
4. 编译 release 版本
5. 验证二进制文件
6. 输出文件信息

## 安全特性

1. **密码哈希**: 使用 Argon2id 算法
2. **JWT 认证**: 基于 JWT 的无状态认证
3. **SQL 注入防护**: 使用参数化查询
4. **XSS 防护**: HTML 转义输出
5. **CSRF 防护**: Token 验证（待实现）
6. **权限控制**: 管理员权限隔离
7. **注册限制**: 首个用户后自动关闭注册

## 性能优化

1. **连接池**: SQLx 连接池复用数据库连接
2. **静态编译**: Musl 静态链接，无外部依赖
3. **LTO 优化**: Link Time Optimization 减少二进制大小
4. **代码剥离**: Strip 调试符号
5. **单二进制**: 无需运行时依赖
6. **异步 IO**: Tokio 异步运行时

## 扩展性

### 添加新 API 端点

1. 在 `src/handlers/api.rs` 添加处理函数
2. 在 `routes()` 函数中注册路由
3. 更新 `API.md` 文档

### 添加新页面

1. 在 `templates/` 创建 HTML 模板
2. 在 `src/handlers/pages.rs` 添加处理函数
3. 在 `routes()` 函数中注册路由
4. 在侧边栏添加导航链接

### 添加新表

1. 在 `src/db.rs` 的 `create_tables()` 中添加建表语句
2. 在 `src/models.rs` 中定义模型结构体
3. 实现 CRUD 操作

## 待完善功能

以下功能已在 UI 中预留，但后端 API 尚未完全实现：

- [ ] 视频搜索 API 对接 CMS
- [ ] 视频详情获取
- [ ] 视频播放地址解析
- [ ] 收藏功能完整实现
- [ ] 历史记录完整实现
- [ ] 播放进度同步
- [ ] 片头片尾跳过设置
- [ ] 豆瓣 API 集成
- [ ] 用户管理（多用户场景）
- [ ] 批量导入导出视频源
- [ ] 健康检查端点

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

MIT License
