use crate::{
    error::{Error, Result},
    utils::crypto::{aes_cbc_encrypt_url, random_vec},
};
use regex::Regex;
use reqwest::{header::HeaderMap, redirect::Policy, Client, Url};
use serde::{Deserialize, Serialize};

// return (ticket, castgc)
pub async fn cas_login(
    cas_url: &str,
    header_map: HeaderMap,
    username: &str,
    password: &str,
) -> Result<(String, String)> {
    let custom = Policy::custom(|attempt| {
        if attempt.url().query_pairs().any(|(k, _)| k == "ticket") {
            attempt.stop()
        } else {
            attempt.follow()
        }
    });
    let client = Client::builder()
        .cookie_store(true)
        .default_headers(header_map)
        .redirect(custom)
        .build()
        .unwrap();

    let auth_response = client
        .get(cas_url)
        .send()
        .await
        .map_err(|_| Error::LoginFail)?;

    let pattern = r#"<input type="hidden" id="pwdEncryptSalt" value="(.+?)" /><input type="hidden" id="execution" name="execution" value="(.+?)" />"#;
    let re = Regex::new(pattern).unwrap();
    let haystack = auth_response.text().await.unwrap();
    let captures = re.captures(&haystack).ok_or(Error::LoginFail)?;

    let (salt, execution) = (
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str(),
    );

    let mut padded_password = random_vec(64);
    padded_password.extend(password.as_bytes());

    let encrypted_password = aes_cbc_encrypt_url(&padded_password, &salt.as_bytes()[..16]);
    let submit_data = CasLoginRequest::new(username, &encrypted_password, execution);

    let submit_request = client.post(cas_url).form(&submit_data).build().unwrap();

    let submit_response = client
        .execute(submit_request)
        .await
        .map_err(|_| Error::LoginFail)?;

    let Some(location) = submit_response.headers().get("location") else {
        return Err(Error::LoginFail);
    };

    let url = Url::parse(location.to_str().unwrap()).unwrap();
    let ticket = url
        .query_pairs()
        .find(|(k, _)| k == "ticket")
        .map(|(_, v)| v)
        .ok_or(Error::LoginFail)?;

    let client_clone = client.clone();
    let url_clone = url.clone();
    tokio::spawn(async move {
        client_clone
            .get(url_clone)
            .send()
            .await
            .map_err(|_| Error::LoginFail)
            .expect("should be ok");
    });

    let cookie_value = submit_response
        .cookies()
        .find(|cookie| cookie.name() == "CASTGC")
        .map(|cookie| cookie.value().to_string())
        .ok_or(Error::LoginFail)?;
    Ok((ticket.into(), cookie_value))
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

// #[cfg(test)]
// mod test {
//     #[test]
// }
