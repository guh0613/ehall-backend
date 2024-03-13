use serde::{Deserialize, Serialize};

use super::UsrPwd;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Info;

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
