# KatelyaTV Rust 部署指南

## 快速部署

### 方法一：使用预编译二进制（推荐）

1. **下载二进制文件**
   ```bash
   # 从 GitHub Releases 下载最新版本
   wget https://github.com/yourusername/katelyatv-rust/releases/latest/download/katelyatv-rust-linux-amd64-musl.tar.gz
   
   # 解压
   tar -xzf katelyatv-rust-linux-amd64-musl.tar.gz
   ```

2. **运行**
   ```bash
   ./katelyatv-rust
   ```

3. **访问**
   
   打开浏览器访问 `http://localhost:3000`

### 方法二：使用 Docker

```dockerfile
FROM alpine:latest
COPY katelyatv-rust /app/katelyatv-rust
WORKDIR /app
EXPOSE 3000
CMD ["./katelyatv-rust", "--host", "0.0.0.0", "--port", "3000"]
```

构建并运行：
```bash
docker build -t katelyatv-rust .
docker run -d -p 3000:3000 -v $(pwd)/data:/app/data katelyatv-rust
```

### 方法三：Systemd 服务（生产环境推荐）

1. **创建服务文件**
   ```bash
   sudo nano /etc/systemd/system/katelyatv-rust.service
   ```

2. **添加以下内容**
   ```ini
   [Unit]
   Description=KatelyaTV Rust Server
   After=network.target
   
   [Service]
   Type=simple
   User=www-data
   Group=www-data
   WorkingDirectory=/opt/katelyatv-rust
   ExecStart=/opt/katelyatv-rust/katelyatv-rust --port 3000
   Restart=on-failure
   RestartSec=5s
   LimitNOFILE=65535
   
   # 安全加固
   NoNewPrivileges=true
   PrivateTmp=true
   ProtectSystem=strict
   ProtectHome=true
   ReadWritePaths=/opt/katelyatv-rust
   
   [Install]
   WantedBy=multi-user.target
   ```

3. **启动服务**
   ```bash
   sudo mkdir -p /opt/katelyatv-rust
   sudo cp katelyatv-rust /opt/katelyatv-rust/
   sudo chown -R www-data:www-data /opt/katelyatv-rust
   
   sudo systemctl daemon-reload
   sudo systemctl enable katelyatv-rust
   sudo systemctl start katelyatv-rust
   
   # 查看状态
   sudo systemctl status katelyatv-rust
   ```

## Nginx 反向代理配置

### 基础配置

```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    # 强制 HTTPS（可选）
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com;
    
    # SSL 证书配置
    ssl_certificate /etc/nginx/ssl/your-domain.crt;
    ssl_certificate_key /etc/nginx/ssl/your-domain.key;
    
    # SSL 优化
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;
    
    # 静态文件缓存
    location /static/ {
        proxy_pass http://127.0.0.1:3000/static/;
        expires 30d;
        add_header Cache-Control "public, immutable";
    }
    
    # 主应用
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket 支持（如果需要）
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        
        # 超时设置
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
}
```

### 启用配置

```bash
sudo ln -s /etc/nginx/sites-available/katelyatv /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

## 数据库备份

SQLite 数据库文件默认位于 `./katelyatv.db`，定期备份：

```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/backup/katelyatv"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR

# 复制数据库文件
cp /opt/katelyatv-rust/katelyatv.db $BACKUP_DIR/katelyatv_$DATE.db

# 压缩
tar -czf $BACKUP_DIR/katelyatv_$DATE.tar.gz $BACKUP_DIR/katelyatv_$DATE.db
rm $BACKUP_DIR/katelyatv_$DATE.db

# 保留最近 7 天的备份
find $BACKUP_DIR -name "katelyatv_*.tar.gz" -mtime +7 -delete

echo "Backup completed: katelyatv_$DATE.tar.gz"
```

添加到 crontab：
```bash
crontab -e
# 每天凌晨 2 点备份
0 2 * * * /opt/katelyatv-rust/backup.sh
```

## 性能优化

### 1. 调整文件描述符限制

```bash
# 编辑 /etc/security/limits.conf
* soft nofile 65535
* hard nofile 65535
```

### 2. 内核参数优化

```bash
# 编辑 /etc/sysctl.conf
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 65535
net.ipv4.ip_local_port_range = 1024 65535

# 应用
sudo sysctl -p
```

### 3. 使用 Jemalloc（可选）

```bash
# 安装 jemalloc
sudo apt-get install libjemalloc-dev

# 运行
LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libjemalloc.so ./katelyatv-rust
```

## 监控和日志

### 查看日志

```bash
# Systemd 服务日志
sudo journalctl -u katelyatv-rust -f

# 按级别过滤
sudo journalctl -u katelyatv-rust -f | grep ERROR
```

### 健康检查端点

添加健康检查 API（待实现）：
```bash
curl http://localhost:3000/api/health
```

## 故障排查

### 常见问题

1. **端口被占用**
   ```bash
   # 检查端口占用
   sudo lsof -i :3000
   
   # 修改端口
   ./katelyatv-rust --port 8080
   ```

2. **权限问题**
   ```bash
   # 确保正确的所有权
   sudo chown -R www-data:www-data /opt/katelyatv-rust
   ```

3. **数据库锁定**
   ```bash
   # 检查是否有其他进程使用数据库
   lsof katelyatv.db
   
   # 重启服务
   sudo systemctl restart katelyatv-rust
   ```

4. **内存不足**
   ```bash
   # 查看内存使用
   free -h
   
   # 限制最大连接数（在代码中配置）
   ```

## 升级流程

1. **停止服务**
   ```bash
   sudo systemctl stop katelyatv-rust
   ```

2. **备份数据**
   ```bash
   cp /opt/katelyatv-rust/katelyatv.db /backup/katelyatv.db.backup
   ```

3. **替换二进制文件**
   ```bash
   cp new-katelyatv-rust /opt/katelyatv-rust/katelyatv-rust
   ```

4. **启动服务**
   ```bash
   sudo systemctl start katelyatv-rust
   ```

5. **验证**
   ```bash
   curl http://localhost:3000
   ```

## 安全建议

1. ✅ 使用 HTTPS（通过 Nginx 反向代理）
2. ✅ 设置强密码的 JWT_SECRET
3. ✅ 定期更新系统和依赖
4. ✅ 限制数据库文件权限
5. ✅ 使用防火墙限制访问
6. ✅ 定期备份数据
7. ✅ 监控系统资源使用
8. ✅ 首个用户注册后自动关闭注册功能
