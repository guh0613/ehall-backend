use regex::Regex;
use reqwest::Client;
use crate::utils::crypto::aes_cbc_encrypt_url;

// async fn cas_login(
//     cas_url: &str,
//     username: &str,
//     password: &str,
// ) -> Result<reqwest::Response, reqwest::Error> {
//     let client = Client::new();
//
//     let auth_response = client.get(cas_url).send().await?;
//
//     let pattern = (r#"<input type="hidden" id="pwdEncryptSalt" value="(.+?)" /><input type="hidden" id="execution" name="execution" value="(.+?)" />"#);
//     let re = regex::Regex::new(pattern).unwrap();
//     let captures = re.captures(&auth_response.text().await.unwrap());
//
//     if let Some(captures) = captures {
//         let salt = captures.get(1).unwrap().as_str();
//         let execution = captures.get(2).unwrap().as_str();
//
//         let encrypted_password =
//             aes_cbc_encrypt_url(&(random_string(64) + password).as_bytes(), salt.as_bytes());
//         let submit_data = [
//             ("username", username),
//             ("password", &encrypted_password),
//             ("captcha", ""),
//             ("_eventId", "submit"),
//             ("cllt", "userNameLogin"),
//             ("dllt", "generalLogin"),
//             ("lt", ""),
//             ("execution", execution),
//         ];
//
//         let submit_response = client.post(&cas_url).form(&submit_data).send().await?;
//         Ok(submit_response)
//     } else {
//         let res = reqwest::Response::new(reqwest::StatusCode::BAD_REQUEST);
//         Ok(res)
//     }
// }
