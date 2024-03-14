use crate::{
    adapters::{LoginType, SchoolAdapter},
    error::{Error, Result},
    models::AuthToken,
};
use async_trait::async_trait;
use regex::Regex;
use reqwest::{header::HeaderMap, Client, Response};
use std::collections::HashMap;

const CAS_SERVER_URL: &str = "https://authserver.nnu.edu.cn/authserver/login?service=https%3A%2F%2Fehall.nnu.edu.cn%2Flogin%3Fservice%3Dhttp%3A%2F%2Fehall.nnu.edu.cn%2Fywtb-portal%2Fstandard%2Findex.html%23%2FWorkBench%2Fworkbench";
const EHALL_SERVER_URL: &str = "https://ehall.nnu.edu.cn";
const EHALL_APP_SERVER_URL: &str = "https://ehallapp.nnu.edu.cn";

#[derive(Debug, Default)]
pub struct Adapter {
    castgc: Option<String>,
}

impl Adapter {
    pub fn new() -> Self {
        Self {
            castgc: None,
        }
    }
}

#[async_trait]
impl SchoolAdapter for Adapter {
    async fn login(&mut self, login: LoginType) -> Result<()> {
        let LoginType::Password((u, p)) = login else {
            return Err(Error::LoginMethodNotSupported);
        };

        Ok(())
    }
}
