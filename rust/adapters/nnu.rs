mod info;
mod rank;
mod score;

use crate::{
    adapters::{LoginType, SchoolAdapter},
    error::{Error, Result},
    models::{
        user::{Info, Score},
        AuthToken, UsrPwd,
    },
    utils::cas::cas_login,
};
use async_trait::async_trait;
use axum::{
    extract::Query,
    http::{header, response},
};
use futures::future::join_all;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE},
    Client, StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

const CAS_SERVER_URL: &str = "https://authserver.nnu.edu.cn/authserver/login?service=https%3A%2F%2Fehall.nnu.edu.cn%2Flogin%3Fservice%3Dhttp%3A%2F%2Fehall.nnu.edu.cn%2Fywtb-portal%2Fstandard%2Findex.html%23%2FWorkBench%2Fworkbench";
const AUTH_HEADER: &str = "https://authserver.nnu.edu.cn/authserver/login?service=https%3A%2F%2Fehall.nnu.edu.cn%2Flogin%3Fservice%3Dhttp%3A%2F%2Fehall.nnu.edu.cn%2Fywtb-portal%2Fstandard%2Findex.html%23%2FWorkBench%2Fworkbench";
const EHALL_SERVER_URL: &str = "https://ehall.nnu.edu.cn";
const EHALL_APP_SERVER_URL: &str = "https://ehallapp.nnu.edu.cn";
const USER_INFO_URL: &str =
    "https://ehall.nnu.edu.cn//jsonp/ywtb/info/getUserInfoAndSchoolInfo.json";
const WEU_COOKIE_URL: &str = "https://ehall.nnu.edu.cn/appShow?appId=4768574631264620";
const USER_SCORE_URL: &str = "https://ehall.nnu.edu.cn/jwapp/sys/cjcx/modules/cjcx/xscjcx.do";

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

    async fn get_info(&mut self) -> Result<Info> {
        #[derive(Serialize, Deserialize)]
        struct UserInfoResponse {
            data: Data,
        }

        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data {
            user_name: String,
            user_id: String,
            user_typename: String,
            user_department: String,
            user_sex: String,
        }

        let Some(ref castgc) = self.castgc else {
            return Err(Error::FetchUserInfoFail);
        };
        let mut header_map = HeaderMap::new();
        header_map.insert(
            COOKIE,
            HeaderValue::from_str(&format!("MOD_AUTH_CAS={}", castgc)).unwrap(),
        );
        let client = Client::builder()
            .cookie_store(true)
            .default_headers(header_map)
            .build()
            .unwrap();
        let res = client
            .get(USER_INFO_URL)
            .send()
            .await
            .map_err(|_| Error::FetchUserInfoFail)?;
        let data = res
            .json::<UserInfoResponse>()
            .await
            .map_err(|_| Error::FetchUserInfoFail)?
            .data;
        Ok(Info::new(
            data.user_name,
            data.user_id,
            data.user_typename,
            data.user_department,
            data.user_sex,
        ))
    }

    async fn refresh_ticket() {}

    async fn get_score(&mut self, _semester: &str, amount: &str) -> Result<Vec<Score>> {
        let Some(ref castgc) = self.castgc else {
            return Err(Error::FetchUserScoreFail);
        };

        let mut header_map = HeaderMap::new();
        header_map.insert(
            COOKIE,
            HeaderValue::from_str(&format!("MOD_AUTH_CAS={}", castgc)).unwrap(),
        );
        let client = Client::builder()
            .cookie_store(true)
            .default_headers(header_map)
            .build()
            .unwrap();
        if client.get(WEU_COOKIE_URL).send().await.unwrap().status() != StatusCode::OK {
            return Err(Error::FetchUserScoreFail);
        }

        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct UserScoreRequest {
            query_setting: Vec<QuerySetting>,
            #[serde(rename = "*order")]
            order: String,
            page_size: String,
            page_number: String,
        }

        #[derive(Serialize, Deserialize)]
        #[allow(non_snake_case)]
        struct QuerySetting {
            name: String,
            caption: String,
            linkOpt: String,
            builderlist: String,
            builder: String,
            value: String,
            value_display: String,
        }

        impl QuerySetting {
            pub fn new(
                name: String,
                caption: String,
                link_opt: String,
                builder_list: String,
                builder: String,
                value: String,
                value_display: String,
            ) -> Self {
                QuerySetting {
                    name,
                    caption,
                    linkOpt: link_opt,
                    builderlist: builder_list,
                    builder,
                    value,
                    value_display,
                }
            }
        }
        let query_request = UserScoreRequest {
            query_setting: vec![
                QuerySetting::new(
                    "SFYX".to_string(),
                    "是否有效".to_string(),
                    "AND".to_string(),
                    "cbl_m_List".to_string(),
                    "m_value_equal".to_string(),
                    "1".to_string(),
                    "是".to_string(),
                ),
                QuerySetting::new(
                    "SHOWMAXCJ".to_string(),
                    "显示最高成绩".to_string(),
                    "AND".to_string(),
                    "cbl_String".to_string(),
                    "equal".to_string(),
                    "0".to_string(),
                    "否".to_string(),
                ),
            ],
            order: "-XNXQDM, -KCH, -KXH".to_string(),
            page_size: amount.to_string(),
            page_number: "1".to_string(),
        };

        let res = client
            .post(USER_SCORE_URL)
            .json(&query_request)
            .send()
            .await
            .map_err(|_| Error::FetchUserScoreFail)?;
        // let json =
        // FIXME: insert user rank.
        Err(Error::FetchUserScoreFail)
    }

    async fn get_rank(
        &mut self,
        course_id: &str,
        class_id: &str,
        semester: &str,
    ) -> Result<Vec<Score>> {
        let Ok(info) = self.get_info().await else {
            return Err(Error::FetchUserRankFail);
        };
        let Some(ref castgc) = self.castgc else {
            return Err(Error::FetchUserRankFail);
        };
        let user_id = info.user_id;
        let mut header_map = HeaderMap::new();
        header_map.insert(
            COOKIE,
            HeaderValue::from_str(&format!("MOD_AUTH_CAS={}", castgc)).unwrap(),
        );
        let client = Client::builder()
            .default_headers(header_map)
            .cookie_store(true)
            .build()
            .unwrap();
        client
            .get(WEU_COOKIE_URL)
            .send()
            .await
            .map_err(|_| Error::FetchUserRankFail)?;

        #[derive(Serialize, Deserialize)]
        struct ScoreResponse {
            datas: Datas,
        }

        #[derive(Serialize, Deserialize)]
        struct Datas {
            jxbcjtjcx: Jxbcjtjcx,
            jxbcjfbcx: Jxbcjfbcx,
            jxbxspmcx: Jxbxspmcx,
        }

        #[derive(Serialize, Deserialize)]
        struct Jxbcjtjcx {
            rows: Vec<Row>,
        }

        #[derive(Serialize, Deserialize)]
        struct Jxbcjfbcx {
            rows: Vec<Row>,
        }

        #[derive(Serialize, Deserialize)]
        struct Jxbxspmcx {
            rows: Vec<Row>,
        }

        #[derive(Serialize, Deserialize)]
        struct Row {
            zgf: f64,
            zdf: f64,
            pjf: f64,
            djsl: Option<i32>,
            pm: i32,
            zrs: i32,
        }

        let tasks = [
        client.post(format!("{}/jwapp/sys/cjcx/modules/cjcx/jxbcjtjcx.do", EHALL_APP_SERVER_URL))
            .json(&json!({"JXBID": class_id, "XNXQDM": semester, "TJLX": "01"}))
            .send(),
        client.post(format!("{}/jwapp/sys/cjcx/modules/cjcx/jxbcjtjcx.do", EHALL_APP_SERVER_URL))
            .json(&json!({"JXBID": "*", "XNXQDM": semester, "KCH": course_id, "TJLX": "02"}))
            .send(),
        client.post(format!("{}/jwapp/sys/cjcx/modules/cjcx/jxbcjfbcx.do", EHALL_APP_SERVER_URL))
            .json(&json!({"JXBID": class_id, "XNXQDM": semester, "TJLX": "01", "*order": "+DJDM"}))
            .send(),
        client.post(format!("{}/jwapp/sys/cjcx/modules/cjcx/jxbcjfbcx.do", EHALL_APP_SERVER_URL))
            .json(&json!({"JXBID": "*", "XNXQDM": semester, "KCH": course_id, "TJLX": "02", "*order": "+DJDM"}))
            .send(),
        client.post(format!("{}/jwapp/sys/cjcx/modules/cjcx/jxbxspmcx.do", EHALL_APP_SERVER_URL))
            .json(&json!({"XH": user_id, "JXBID": class_id, "XNXQDM": semester, "TJLX": "01"}))
            .send(),
        client.post(format!("{}/jwapp/sys/cjcx/modules/cjcx/jxbxspmcx.do", EHALL_APP_SERVER_URL))
            .json(&json!({"XH": user_id, "JXBID": "*", "XNXQDM": semester, "KCH": course_id, "TJLX": "02"}))
            .send(),
    ];

        let res = join_all(tasks).await;

        let res = join_all(
            res
                .into_iter()
                .map(|res| async {
                    let score_res: ScoreResponse = res
                        .map_err(|_| Error::FetchUserRankFail)?
                        .json()
                        .await
                        .map_err(|_| Error::FetchUserRankFail)?;
                    Ok(score_res)
                })
                .collect::<Vec<_>>(),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        let class_score_response = &res[0].datas.jxbcjtjcx.rows[0];
        let school_score_response = &res[1].datas.jxbcjtjcx.rows[0];
        let class_people_response = &res[2].datas.jxbcjfbcx.rows[0];
        let school_people_response = &res[3].datas.jxbcjfbcx.rows[0];
        let class_rank_response = &res[4].datas.jxbxspmcx.rows[0];
        let school_rank_response = &res[5].datas.jxbxspmcx.rows[0];

        Err(Error::FetchUserRankFail)
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

    async fn fetch_user_info(&mut self) -> Result<Info> {
        self.get_info().await
    }

    async fn fetch_scores(&mut self) -> Result<Vec<Score>> {
        self.get_score("all", "64").await
    }
}
