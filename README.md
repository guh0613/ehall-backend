# ehall-backend

## 概述
ehall-backend是一个纯api式的服务端，使用Flask框架运行。该服务端可向不同学校的一站式事务大厅后端服务器获取各类数据。

## API端点

### CAS 登录

用于向学校后端cas服务器进行登录请求，保持登录状态以及获取登录token。

- **请求URL**:
  `/api/<school_name>/cas_login`

- **请求方法**:
  `POST`

#### URL参数

- `school_name`: 学校名称，可详见学校支持列表。

#### 请求体（json）

  ```json
  {
      "username": "114514",
      "password": "password"
  }
  ```

#### 成功响应

- **代码**：200
- **响应示例**：
  ```json
  {
      "status": "OK",
      "message": "Login successful",
      "auth_token": "TGT-114514-xxxxxx"
  }
  ```


### 用户信息

用于获取用户信息。

- **请求URL**:
  `/api/<school_name>/user/info`

- **请求方法**:
  `GET`

#### URL参数

- `school_name`: 学校名称。

#### 请求头

- `Authorization`: `auth_token`

#### 成功响应

- **代码**：200
- **响应示例**：
  ```json
  {
      "status": "OK",
      "message": "User info retrieved successfully",
      "data": {
          "userName": "李田所",
          "userId": "114514",
          "userType": "学生",
          "userDepartment": "计算机科学与技术学院",
          "userSex": "男"
      }
  }
  ```

## 特别响应
部分特殊错误可能需要客户端进行特定处理，其响应会有特定的状态码及`status`字段。

### 认证过期
请求头中的`auth_token`无效或过期。

- **代码**：401
- **响应示例**:
    ```json
    {
        "status": "invalid",
        "message": "Failed to get user info.auth_token is probably invalid"
    }
    ```
  
### 意外错误
产生了意外错误，且可能是后端服务器产生的问题，建议客户端进行重试。

- **代码**：402
- **响应示例**:
    ```json
    {
        "status": "retry",
        "message": "Unexpected behavior happened.Please try again"
    }
    ```
