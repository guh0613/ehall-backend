pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UsrPwd {
    username: String,
    password: String,
}

pub type LoginRequest = UsrPwd;

impl UsrPwd {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub type AuthToken = String;

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    status: String,
    message: String,
    auth_token: AuthToken,
}

impl LoginResponse {
    pub fn new(status: String, message: String, auth_token: AuthToken) -> Self {
        Self {
            status,
            message,
            auth_token,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum School {
    NanjingNormalUniversity,
    YanShanUniversity,
}

impl From<&str> for School {
    fn from(value: &str) -> Self {
        match value {
            "nnu" => School::NanjingNormalUniversity,
            "ysu" => School::YanShanUniversity,
            _ => unreachable!(),
        }
    }
}
