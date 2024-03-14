use crate::{
    error::{Error, Result},
    utils::crypto::{aes_cbc_encrypt_url, random_vec},
};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn cas_login(cas_url: &str, username: &str, password: &str) -> Result<reqwest::Response> {
    let client = Client::new();

    let auth_response = client.get(cas_url).send().await.map_err(|_| Error::LoginFail)?;

    let pattern = r#"<input type="hidden" id="pwdEncryptSalt" value="(.+?)" /><input type="hidden" id="execution" name="execution" value="(.+?)" />"#;
    let re = Regex::new(pattern).unwrap();
    let haystack = auth_response.text().await.unwrap();
    let captures = re.captures(&haystack).ok_or(Error::LoginFail)?;

    let salt = captures.get(1).unwrap().as_str();
    let execution = captures.get(2).unwrap().as_str();
    let mut padded_password = random_vec(64);
    padded_password.extend(password.as_bytes());

    let encrypted_password = aes_cbc_encrypt_url(&padded_password, salt.as_bytes());
    let submit_data = CasLoginRequest::new(username, &encrypted_password, execution);

    let submit_response = client.post(cas_url).form(&submit_data).send()
        .await
        .map_err(|_| Error::LoginFail)?;
    Ok(submit_response)
}

#[derive(Serialize, Deserialize)]
pub struct CasLoginRequest<'a> {
    username: &'a str,
    password: &'a str,
    captcha: &'a str,
    _eventId: &'a str,
    cllt: &'a str,
    dllt: &'a str,
    lt: &'a str,
    execution: &'a str,
}

impl<'a> CasLoginRequest<'a> {
    fn new(username: &'a str, password: &'a str, execution: &'a str) -> Self {
        Self {
            username,
            password,
            captcha: "",
            _eventId: "submit",
            cllt: "userNameLogin",
            dllt: "generalLogin",
            lt: "",
            execution,
        }
    }
}
