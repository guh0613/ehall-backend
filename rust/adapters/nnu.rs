use crate::{
    adapters::{LoginType, SchoolAdapter},
    error::{Error, Result},
    models::{AuthToken, UsrPwd},
    utils::cas::cas_login,
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, REFERER},
};

const CAS_SERVER_URL: &str = "https://authserver.nnu.edu.cn/authserver/login?service=https%3A%2F%2Fehall.nnu.edu.cn%2Flogin%3Fservice%3Dhttp%3A%2F%2Fehall.nnu.edu.cn%2Fywtb-portal%2Fstandard%2Findex.html%23%2FWorkBench%2Fworkbench";
const AUTH_HEADER: &str = "https://authserver.nnu.edu.cn/authserver/login?service=https%3A%2F%2Fehall.nnu.edu.cn%2Flogin%3Fservice%3Dhttp%3A%2F%2Fehall.nnu.edu.cn%2Fywtb-portal%2Fstandard%2Findex.html%23%2FWorkBench%2Fworkbench";
const EHALL_SERVER_URL: &str = "https://ehall.nnu.edu.cn";
const EHALL_APP_SERVER_URL: &str = "https://ehallapp.nnu.edu.cn";

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
        headers.insert(
            "Accept-Encoding",
            HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("zh-CN,zh;q=0.9"),
        );
        headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
        headers.insert(
            "Sec-Ch-Ua",
            HeaderValue::from_static("\"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""),
        );
        headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
        headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"macOS\""));
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
        headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
        headers.insert("Priority", HeaderValue::from_static("u=0, i"));
        headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.5993.90 Safari/537.36"));
        headers
    };
}

#[derive(Debug, Default)]
pub struct Adapter {
    // usrpwd: UsrPwd,
    ticket: Option<String>,
    castgc: Option<String>,
}

impl Adapter {
    pub fn new() -> Self {
        Self {
            ticket: None,
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
        // let header_map = HEADERS.clone();
        let header_map = HeaderMap::new();
        // header_map.insert(REFERER, HeaderValue::from_static(AUTH_HEADER));
        let (ticket, castgc) = cas_login(CAS_SERVER_URL, header_map, &u, &p).await?;
        self.ticket = Some(ticket);
        self.castgc = Some(castgc);
        Ok(())
    }
}
