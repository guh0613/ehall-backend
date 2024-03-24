pub mod nnu;
pub mod nuaa;
pub mod ysu;

use crate::{
    error::Result,
    models::user::{CourseTable, ExamSchedule, Info, LoginType, Notification, Score},
};
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait SchoolAdapter: Debug {
    async fn login(&mut self, login: LoginType) -> Result<()>;

    async fn fetch_user_info(&mut self) -> Result<Info>;

    async fn fetch_scores(&mut self) -> Result<Vec<Score>> {
        unimplemented!()
    }

    async fn fetch_course_table(&mut self) -> Result<Vec<CourseTable>> {
        unimplemented!()
    }

    async fn fetch_exam_schedule(&mut self) -> Result<ExamSchedule> {
        unimplemented!()
    }

    async fn fetch_notifications(&mut self) -> Result<Vec<Notification>> {
        unimplemented!()
    }
}
