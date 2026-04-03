# KatelyaTV Rust v{{version}}

## 📦 下载

### Linux AMD64 (Musl 静态编译)

- **二进制文件**: [katelyatv-rust-linux-amd64-musl.tar.gz](链接)
- **SHA256**: [katelyatv-rust-linux-amd64-musl.tar.gz.sha256](链接)

### 验证下载

```bash
# 验证 SHA256
sha256sum -c katelyatv-rust-linux-amd64-musl.tar.gz.sha256

# 解压
tar -xzf katelyatv-rust-linux-amd64-musl.tar.gz

# 运行
./katelyatv-rust --help
```

---

## ✨ 版本亮点

<!-- 在此处添加本版本的主要更新内容 -->

---

## 🚀 快速开始

### 1. 解压文件

```bash
tar -xzf katelyatv-rust-linux-amd64-musl.tar.gz
```

### 2. 运行

```bash
./katelyatv-rust
```

### 3. 访问

打开浏览器访问 `http://localhost:3000`

### 4. 注册管理员

第一个注册的账号自动成为管理员，注册后系统将自动关闭注册功能。

---

## 📝 完整更新日志

<!-- 在此处添加详细的更新日志 -->

---

## 🔧 技术细节

- **编译目标**: x86_64-unknown-linux-musl
- **Rust 版本**: stable
- **编译优化**: LTO + strip
- **依赖**: 无（完全静态编译）

---

## 📚 文档

- [README.md](https://github.com/yourusername/katelyatv-rust/blob/main/README.md) - 项目介绍
- [QUICKSTART.md](https://github.com/yourusername/katelyatv-rust/blob/main/QUICKSTART.md) - 快速开始
- [DEPLOYMENT.md](https://github.com/yourusername/katelyatv-rust/blob/main/DEPLOYMENT.md) - 部署指南
- [API.md](https://github.com/yourusername/katelyatv-rust/blob/main/API.md) - API 文档

---

## ⚠️ 注意事项

1. **首次使用**：第一个注册用户自动成为管理员
2. **注册开关**：首个用户注册后自动关闭，需管理员手动开启
3. **端口配置**：默认 3000 端口，可通过 `--port` 参数修改
4. **数据库**：SQLite 文件默认在当前目录，建议定期备份

---

## 🐛 已知问题

<!-- 在此处列出已知的 bug 或限制 -->

---

## 🙏 致谢

感谢所有贡献者和使用者！

---

## 📄 许可证

MIT License
