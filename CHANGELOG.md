# 更新日志 (Changelog)

所有重要的项目变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [未发布]

### Added - 新增
- 初始版本发布
- 用户认证系统（登录/注册）
- 首个用户自动管理员功能
- 注册开关控制（首个用户后自动关闭）
- 视频源管理（增删改查）
- TVBox 配置接口（JSON/Base64）
- SQLite 数据库支持
- 响应式 Web 界面
- 管理面板
- JWT Token 认证
- Argon2 密码哈希
- GitHub Actions 自动编译工作流
- Linux AMD64 Musl 静态编译

### Changed - 变更
- 无（初始版本）

### Deprecated - 弃用
- 无

### Removed - 移除
- 无

### Fixed - 修复
- 无

### Security - 安全
- 使用 Argon2id 进行密码哈希
- JWT Token 7 天有效期
- SQL 注入防护（参数化查询）
- XSS 防护（HTML 转义）

---

## [0.1.0] - 2024-XX-XX

### 说明
这是 KatelyaTV Rust 的第一个公开版本，使用 Rust 语言完整重构了原 KatelyaTV 项目的核心功能。

### 特性
- ✅ 单文件部署，无外部依赖
- ✅ 首个用户自动管理员
- ✅ 注册自动开关
- ✅ 完整的用户认证系统
- ✅ 视频源管理
- ✅ TVBox 兼容接口
- ✅ 响应式 Web 界面
- ✅ GitHub Actions 自动编译

### 待实现功能
- ⏳ 视频搜索 API 对接
- ⏳ 播放历史记录
- ⏳ 收藏功能
- ⏳ 播放进度同步
- ⏳ 片头片尾跳过
- ⏳ 豆瓣 API 集成

---

## 版本发布周期说明

### 版本号规则
- **主版本号 (Major)**: 不兼容的 API 变更或重大功能更新
- **次版本号 (Minor)**: 向后兼容的功能新增
- **修订号 (Patch)**: 向后兼容的问题修复

### 发布流程
1. 开发完成后在本地测试
2. 推送代码到 GitHub
3. 打标签并推送：`git tag v0.1.0 && git push origin v0.1.0`
4. GitHub Actions 自动编译并发布 Release
5. 更新本文档

### 自动化
- GitHub Actions 会在 tag 推送时自动：
  - 编译 Linux AMD64 Musl 二进制文件
  - 打包为 tar.gz
  - 计算 SHA256 校验和
  - 创建 GitHub Release
  - 附加编译产物

---

## 贡献者

感谢所有为这个项目做出贡献的人！

<!-- 未来可以添加贡献者列表 -->

---

## 支持

如有问题或建议，请：
1. 查看相关文档（README.md, DEPLOYMENT.md, API.md）
2. 在 GitHub Issues 中提交问题
3. 参与社区讨论

---

**最后更新**: 2024-XX-XX
