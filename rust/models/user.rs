use serde::{Deserialize, Serialize};

use super::UsrPwd;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub user_name: String,
    pub user_id: String,
    pub user_type: String,
    pub user_department: String,
    pub user_sex: String,
}

impl Info {
    pub fn new(
        user_name: String,
        user_id: String,
        user_type: String,
        user_department: String,
        user_sex: String,
    ) -> Self {
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
pub struct Score {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AllRank {
    class: Rank,
    school: Rank,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rank {
    rank: i64,
    total_people_num: i64,
    high_score: i64,
    low_score: i64,
    average_score: i64,
    num_above90: Option<i64>,
    num_above80: Option<i64>,
    num_above70: Option<i64>,
    num_above60: Option<i64>,
    num_below60: Option<i64>,
}

impl Score {
    pub fn new() -> Self {
        Self {}
    }
}

impl AllRank {
    pub fn new(class: Rank, school: Rank) -> Self {
        Self { class, school }
    }
}

impl Rank {
    pub fn new(
        rank: i64,
        total_people_num: i64,
        high_score: i64,
        low_score: i64,
        average_score: i64,
        num_above90: Option<i64>,
        num_above80: Option<i64>,
        num_above70: Option<i64>,
        num_above60: Option<i64>,
        num_below60: Option<i64>,
    ) -> Self {
        Self {
            rank,
            total_people_num,
            high_score,
            low_score,
            average_score,
            num_above90,
            num_above80,
            num_above70,
            num_above60,
            num_below60,
        }
    }
}

impl CourseTable {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExamSchedule {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notification {
    pub fn new() -> Self {
        Self {}
    }
}

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
