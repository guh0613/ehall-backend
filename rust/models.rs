pub mod user;

use std::sync::Arc;

use dashmap::DashMap;
use serde::{Deserialize, Serialize}; use tokio::sync::Mutex;
use uuid::Uuid;

use crate::adapters::{nnu, SchoolAdapter};
use crate::error::{Error, Result};

use self::user::{Info, LoginType, Score};

// MARKER: Model Controller
#[derive(Debug, Clone)]
pub struct ModelController {
    user_store: Arc<DashMap<AuthToken, Mutex<Box<dyn SchoolAdapter + Send>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            user_store: Arc::default(),
        })
    }

    pub async fn create_user(&self, p: UsrPwd, school: School) -> Result<AuthToken> {
        let adapter = match school {
            School::NanjingNormalUniversity => {
                let mut adapter = nnu::Adapter::new();
                let Ok(_) = adapter.login(LoginType::new_up(p)).await else {
                    return Result::Err(Error::LoginFail);
                };
                Box::new(adapter)
            }
            School::YanShanUniversity => {
                todo!()
            }
            _ => unimplemented!(),
        };

        // We use uuid to index the user, at least for the time being.
        let auth_token = Uuid::new_v4().to_string();
        self.user_store
            .insert(auth_token.clone(), Mutex::new(adapter));
        Ok(auth_token)
    }

    pub fn delete_user(&self, t: &AuthToken) -> Result<()> {
        // how to implement this in one line
        if self.user_store.remove(t).is_some() {
            Ok(())
        } else {
            Err(Error::DeleteUserFail)
        }
    }

    pub async fn is_valid_auth_token(&self, t: &AuthToken) -> bool {
        self.user_store.contains_key(t)
    }

    pub async fn user_info(&self, t: &AuthToken) -> Result<Info> {
        if let Some(v) = self.user_store.get_mut(t) {
            v.lock().await.fetch_user_info().await
        } else {
            Err(Error::FetchUserInfoFail)
        }
    }

    pub async fn user_score(&self, t: &AuthToken) -> Result<Vec<Score>> {
        if let Some(v) = self.user_store.get_mut(t) {
            v.lock().await.fetch_scores().await
        } else {
            Err(Error::FetchUserScoreFail)
        }
    }
}

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
    auth_token: Option<AuthToken>,
}

impl LoginResponse {
    pub fn new_ok(t: AuthToken) -> Self {
        Self {
            status: "Ok".to_owned(),
            message: "Login Successful".to_owned(),
            auth_token: Some(t),
        }
    }

    pub fn new_fail(m: String) -> Self {
        Self {
            status: "Failed".to_owned(),
            message: m,
            auth_token: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum School {
    NanjingNormalUniversity,
    YanShanUniversity,
    NanjingUniversityOfAeronauticsAndAstronautics,
}

impl TryFrom<&str> for School {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "nnu" => Ok(School::NanjingNormalUniversity),
            "ysu" => Ok(School::YanShanUniversity),
            "nuaa" => Ok(School::NanjingUniversityOfAeronauticsAndAstronautics),
            _ => Err(Error::SchoolNotSupported),
        }
    }
}

impl TryFrom<String> for School {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "nnu" => Ok(School::NanjingNormalUniversity),
            "ysu" => Ok(School::YanShanUniversity),
            "nuaa" => Ok(School::NanjingUniversityOfAeronauticsAndAstronautics),
            _ => Err(Error::SchoolNotSupported),
        }
    }
}
