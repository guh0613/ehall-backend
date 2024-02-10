# ehall-backend

## 概述
ehall-backend是一个纯api式的服务端，使用Flask框架运行。该服务端可向不同学校的一站式事务大厅后端服务器获取各类数据。

## API端点

### CAS 登录

用于向学校后端cas服务器进行登录请求，保持登录状态以及获取身份验证票据。

- **请求URL**:
  `/api/<school_name>/cas_login`

- **请求方法**:
  `POST`

#### URL参数

- `school_name`: 学校名称，可详见学校支持列表。

#### 请求体（json）

- 使用用户名和密码登录：
  ```json
  {
      "username": "114514",
      "password": "password"
  }
  ```

- 使用`CASTGC`登录：
  ```json
  {
      "CASTGC": "TGT-114514-xxxxxx"
  }
  ```

#### 成功响应

- **代码**：200
- **响应示例**：
  ```json
  {
      "status": "OK",
      "message": "Login successful",
      "CASTGC": "TGT-114514-xxxxxx",
      "MOD_AUTH_CAS": "MOD_AUTH_ST-114514-xxxxxx"
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

- `Authorization`: `MOD_AUTH_CAS`票据。

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

若请求提交的票据(`CASTGC`或`MOD_AUTH_CAS`)无效或过期，响应中的`status`会变为`invalid`。

- **代码**：401
- **响应示例**:
    ```json
    {
        "status": "invalid",
        "message": "Failed to login.CASTGC is probably invalid"
    }
    ```
