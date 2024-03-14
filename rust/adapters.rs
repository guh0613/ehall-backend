pub mod nnu;
pub mod nuaa;
pub mod ysu;

use crate::{
    error::{Error, Result},
    models::user::{CourseTable, ExamSchedule, Info, LoginType, Notification, Score},
};
use async_trait::async_trait;
use axum::{body::Body, extract::Request, response::Response};
use futures::future::BoxFuture;
use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{BoxError, Service};

#[async_trait]
pub trait SchoolAdapter: Debug {
    async fn login(&mut self, login: LoginType) -> Result<()> {
        unimplemented!()
    }

    async fn fetch_user_info(&mut self) -> Result<Info> {
        unimplemented!()
    }

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

struct SchoolService<T: SchoolAdapter> {
    adapter: T,
}

// impl<T, B> Service<Request<B>> for SchoolService<T>
// where
//     T: SchoolAdapter + Clone + Send + 'static,
//     B: Send + 'static,
// {
//     type Response = Response;
//     type Error = String;
//     type Future = BoxFuture<'static, Result<Self::Response>>;
//
//     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), String>> {
//         Poll::Ready(Ok(()))
//     }
//
//     fn call(&mut self, request: Request<B>) -> Self::Future {
//         let adapter = self.adapter.clone();
//         let response = async move {
//             // Some shits
//
//             Ok(Response::new(Body::from("Hello from SchoolAdapter")))
//         };
//         Box::pin(response)
//     }
// }
