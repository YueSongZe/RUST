# Mini KMS Admin Frontend

这是一个为Mini KMS Admin后端生成的最小化前端。

## 功能

- 用户登录
- 用户管理（查看和创建用户）
- 设备管理（查看设备）
- 密钥包管理（查看密钥包）
- 系统状态查看

## 运行步骤

1. 确保后端已启动并运行在 `http://localhost:3000`

2. 启动前端服务器（使用Python）：
   ```
   python -m http.server 8080
   ```

3. 在浏览器中打开 `http://localhost:8080`

## 注意事项

- 前端假设后端API在 `http://localhost:3000`
- 登录后token会存储在localStorage中
- 如果API调用失败，会自动登出