# KatelyaTV Rust API 文档

## 认证相关

### 用户登录

**POST** `/api/auth/login`

请求体：
```json
{
  "username": "string",
  "password": "string"
}
```

响应（成功）：
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "username": "admin",
      "is_admin": true
    }
  },
  "message": null
}
```

响应（失败）：
```json
{
  "success": false,
  "data": null,
  "message": "Invalid credentials"
}
```

---

### 用户注册

**POST** `/api/auth/register`

请求体：
```json
{
  "username": "string",
  "password": "string",
  "confirm_password": "string"
}
```

响应：
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "username": "newuser",
      "is_admin": true
    }
  },
  "message": null
}
```

**注意**：
- 第一个注册的用户自动成为管理员
- 首个用户注册后，注册功能将自动关闭
- 如需再次开放注册，需管理员在设置中手动开启

---

### 获取当前用户信息

**GET** `/api/auth/me`

请求头：
```
Authorization: Bearer <token>
```

响应：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "username": "admin",
    "is_admin": true
  },
  "message": null
}
```

---

## 视频源管理

### 获取所有视频源

**GET** `/api/sources`

响应：
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "name": "资源站 1",
      "url": "https://example.com/api.php",
      "api_type": "cms",
      "is_active": true,
      "sort_order": 0,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "message": null
}
```

---

### 创建视频源

**POST** `/api/sources`

请求体：
```json
{
  "name": "新资源站",
  "url": "https://newsource.com/api.php",
  "api_type": "cms",
  "is_active": true,
  "sort_order": 10
}
```

响应：
```json
{
  "success": true,
  "data": {
    "id": 2,
    "name": "新资源站",
    "url": "https://newsource.com/api.php",
    "api_type": "cms",
    "is_active": true,
    "sort_order": 10,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "message": null
}
```

---

### 获取单个视频源

**GET** `/api/sources/:id`

响应：
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "资源站 1",
    "url": "https://example.com/api.php",
    "api_type": "cms",
    "is_active": true,
    "sort_order": 0,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "message": null
}
```

---

### 更新视频源

**PUT** `/api/sources/:id`

请求体：
```json
{
  "name": "更新后的名称",
  "url": "https://updated.com/api.php",
  "api_type": "cms",
  "is_active": true,
  "sort_order": 5
}
```

---

### 删除视频源

**DELETE** `/api/sources/:id`

响应：
```json
{
  "success": true,
  "data": null,
  "message": null
}
```

---

## TVBox 配置

### 获取 TVBox 配置（JSON 格式）

**GET** `/api/tvbox?format=json`

响应：
```json
{
  "sites": [
    {
      "key": "source_1",
      "name": "资源站 1",
      "type": 1,
      "api": "https://example.com/api.php",
      "searchable": 1,
      "changeable": 1
    }
  ],
  "parses": [
    {
      "name": "内置解析",
      "type": 0,
      "url": "http://localhost:3000/api/parse"
    }
  ],
  "flags": ["youku", "qq", "iqiyi"],
  "ads": [],
  "wallpaper": "https://picsum.photos/1920/1080"
}
```

---

### 获取 TVBox 配置（Base64 格式）

**GET** `/api/tvbox?format=base64`

响应：Base64 编码的 JSON 字符串

---

## 设置管理

### 设置注册开关（仅管理员）

**PUT** `/api/settings/register`

请求头：
```
Authorization: Bearer <admin_token>
```

请求体：
```json
{
  "enable": true
}
```

响应：
```json
{
  "success": true,
  "data": null,
  "message": null
}
```

---

## 错误响应格式

所有 API 错误统一返回格式：

```json
{
  "success": false,
  "data": null,
  "message": "错误描述信息"
}
```

常见 HTTP 状态码：
- `200 OK` - 请求成功
- `400 Bad Request` - 请求参数错误
- `401 Unauthorized` - 未授权，需要登录
- `403 Forbidden` - 禁止访问，权限不足
- `404 Not Found` - 资源不存在
- `500 Internal Server Error` - 服务器内部错误

---

## 认证说明

1. **Token 获取**：通过登录或注册接口获取 JWT token
2. **Token 使用**：在请求头中添加 `Authorization: Bearer <token>`
3. **Token 有效期**：默认 7 天，过期需重新登录
4. **Token 刷新**：当前版本不支持自动刷新，过期后需重新登录

---

## 使用示例

### cURL 示例

```bash
# 登录
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password123"}'

# 获取视频源列表（需要认证）
curl -X GET http://localhost:3000/api/sources \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"

# 添加视频源（需要管理员权限）
curl -X POST http://localhost:3000/api/sources \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{
    "name": "新资源站",
    "url": "https://newsource.com/api.php",
    "api_type": "cms",
    "is_active": true,
    "sort_order": 10
  }'
```

### JavaScript Fetch 示例

```javascript
// 登录
const loginResponse = await fetch('/api/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    username: 'admin',
    password: 'password123'
  })
});

const { data: { token } } = await loginResponse.json();

// 使用 token 访问受保护的 API
const sourcesResponse = await fetch('/api/sources', {
  headers: {
    'Authorization': `Bearer ${token}`
  }
});

const sources = await sourcesResponse.json();
console.log(sources);
```

---

## 待实现 API

以下 API 计划在未来版本中实现：

- `GET /api/search?q=<query>` - 搜索视频
- `GET /api/video/:id` - 获取视频详情
- `GET /api/video/:id/play` - 获取播放地址
- `POST /api/favorites` - 添加收藏
- `GET /api/favorites` - 获取收藏列表
- `DELETE /api/favorites/:id` - 删除收藏
- `GET /api/history` - 获取观看历史
- `DELETE /api/history/:id` - 删除历史记录
- `DELETE /api/history/clear` - 清空全部历史
- `POST /api/video/:id/progress` - 更新播放进度
- `GET /api/health` - 健康检查端点
