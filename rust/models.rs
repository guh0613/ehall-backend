pub mod user;

use std::{collections::HashMap, sync::Arc};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::adapters::{nnu, SchoolAdapter};
use crate::error::{Error, Result};

use self::user::{Info, LoginType, Score};

// MARKER: Model Controller
#[derive(Debug, Clone)]
pub struct ModelController {
    user_store: Arc<DashMap<AuthToken, Box<dyn SchoolAdapter>>>,
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
        self.user_store.insert(auth_token.clone(), adapter);
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

    // pub async fn user_info(&self, t: &AuthToken) -> Result<Info> {
    //     if let Some(mut k) = self.user_store.get_mut(t) {
    //         k.fetch_user_info().await.map_err(|_| Error::FetchUserInfoFail)
    //     } else {
    //         Err(Error::InvalidAuthToken)
    //     }
    // }

    pub async fn is_valid_auth_token(&self, t: &AuthToken) -> bool {
        self.user_store.contains_key(t)
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
