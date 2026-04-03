# KatelyaTV Rust 快速开始指南

## 5 分钟快速部署

### 步骤 1：下载二进制文件

```bash
# 创建目录
mkdir -p /opt/katelyatv-rust
cd /opt/katelyatv-rust

# 从 GitHub Releases 下载（替换为实际版本 URL）
wget https://github.com/yourusername/katelyatv-rust/releases/latest/download/katelyatv-rust-linux-amd64-musl.tar.gz

# 解压
tar -xzf katelyatv-rust-linux-amd64-musl.tar.gz
```

### 步骤 2：运行

```bash
# 直接运行
./katelyatv-rust

# 或使用后台运行
nohup ./katelyatv-rust &
```

### 步骤 3：访问

打开浏览器访问：`http://localhost:3000`

### 步骤 4：注册管理员账号

1. 点击页面右上角的"注册"按钮
2. 填写用户名和密码
3. 点击"注册"
4. **重要**：第一个注册的账号自动成为管理员
5. 注册完成后，注册功能将自动关闭

### 步骤 5：添加视频源

1. 使用刚注册的账号登录
2. 点击左侧菜单的"管理面板"
3. 在"视频源管理"区域点击"添加视频源"
4. 填写视频源信息：
   - 名称：例如"资源站 1"
   - API 地址：CMS 接口地址
   - 类型：cms
   - 状态：启用
5. 点击"保存"

### 步骤 6：开始使用

- 回到首页浏览推荐内容
- 使用搜索功能查找影视
- 点击视频卡片开始播放

---

## 自定义配置

### 修改端口

```bash
./katelyatv-rust --port 8080
```

### 修改数据库路径

```bash
./katelyatv-rust --database-url /data/katelyatv.db
```

### 查看所有选项

```bash
./katelyatv-rust --help
```

输出：
```
KatelyaTV Rust - 自托管影视聚合播放器

Usage: katelyatv-rust [OPTIONS]

Options:
      --host <HOST>              服务器监听地址 [default: 0.0.0.0]
  -p, --port <PORT>              服务器端口 [default: 3000]
      --database-url <DATABASE>  数据库文件路径 [default: ./katelyatv.db]
      --jwt-secret <SECRET>      JWT 密钥（不设置则自动生成）
      --enable-register          是否启用注册（默认 false）
      --admin-password <PASSWORD> 管理员密码（用于重置）
  -h, --help                     显示帮助信息
```

---

## 重新开启注册（管理员操作）

如果注册已关闭，需要添加新用户：

### 方法 1：通过管理面板

1. 使用管理员账号登录
2. 进入"管理面板"
3. 在"用户管理"区域开启"开放注册"开关
4. 新用户即可注册
5. 记得关闭开关

### 方法 2：通过命令行参数重启

```bash
# 停止当前服务
pkill katelyatv-rust

# 带参数重启
./katelyatv-rust --enable-register
```

---

## 常见问题

### Q: 忘记管理员密码怎么办？

A: 目前需要手动删除数据库重新注册：
```bash
rm katelyatv.db
./katelyatv-rust
# 重新注册第一个账号
```

### Q: 如何备份数据？

A: 复制 SQLite 数据库文件即可：
```bash
cp katelyatv.db katelyatv.db.backup
```

### Q: 如何在后台运行？

A: 使用 nohup 或 systemd：
```bash
# 使用 nohup
nohup ./katelyatv-rust > katelyatv.log 2>&1 &

# 或使用 systemd（参考 DEPLOYMENT.md）
```

### Q: 为什么注册不了第二个用户？

A: 首个用户注册后系统自动关闭注册以保护安全。管理员可在管理面板手动开启。

### Q: 支持 ARM 架构吗？

A: 当前版本仅编译 AMD64 架构。如需 ARM 版本，请自行编译：
```bash
rustup target add aarch64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl
```

### Q: 如何查看日志？

A: 设置 RUST_LOG 环境变量：
```bash
RUST_LOG=debug ./katelyatv-rust
```

---

## 下一步

- 📖 阅读 [README.md](README.md) 了解完整功能
- 🔧 查看 [DEPLOYMENT.md](DEPLOYMENT.md) 学习生产环境部署
- 📡 参考 [API.md](API.md) 开发第三方客户端
- 🏗️ 查看 [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) 了解项目结构

---

## 获取帮助

如有问题，请：
1. 查看本文档
2. 查阅 [DEPLOYMENT.md](DEPLOYMENT.md) 中的故障排查章节
3. 在 GitHub 提交 Issue

祝你使用愉快！🎉
