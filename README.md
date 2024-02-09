# ehall-backend

## 概述
ehall-backend是一个纯api式的服务端，使用Flask框架运行。该服务端可向不同学校的一站式事务大厅后端服务器获取各类数据。

## API端点

### CAS 登录

用于向学校后端cas服务器进行登录请求，保持登录状态以及获取身份验证票据。

- **请求URL**:
  `/api/cas_login/<school_name>`

- **请求方法**:
  `POST`

#### URL参数

- `school_name`: 学校名称，可详见学校支持列表。

#### 请求体（选择一种格式）

- 使用用户名和密码登录：
  ```json
  {
      "username": "114514",
      "password": "password"
  }
  ```

- 使用`castgc`登录：
  ```json
  {
      "castgc": "TGT-114514-xxxxxx"
  }
  ```

#### 成功响应

- **代码**：200
- **响应示例**：
  ```json
  {
      "status": "success",
      "message": "Logged in successfully",
      "castgc": "TGT-114514-xxxxxx",(使用账号密码登录时)
      "mod_auth_cas": "MOD_AUTH_ST-114514-xxxxxx"
  }
  ```

#### 错误响应

- **代码**：400
- **内容（示例）**：
  ```json
  {
      "status": "error",
      "message": "Username and password are required"
  }
  ```
  ```json
  {
      "status": "error",
      "message": "No CAS URL found for {school_name}"
  }
  ```
  ```json
  {
      "status": "error",
      "message": "Failed to get password salt and execution"
  }
  ```
