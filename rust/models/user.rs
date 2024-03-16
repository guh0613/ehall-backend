use serde::{Deserialize, Serialize};

use super::UsrPwd;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    user_name: String,
    user_id: String,
    user_type: String,
    user_department: String,
    user_sex: String,
}

impl Info {
    pub fn new(user_name: String, user_id: String, user_type: String, user_department: String, user_sex: String) -> Self {
        Self {
            user_name,
            user_id,
            user_type,
            user_department,
            user_sex,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Score;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CourseTable;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExamSchedule;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Notification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginType {
    CasLogin(String),
    Password((String, String)),
}

impl LoginType {
    pub fn new_up(up: UsrPwd) -> Self {
        Self::Password((up.username, up.password))
    }
}
