use axum::Error;
use axum::{extract::Request, response::Response};
use futures::Future;
use std::pin::Pin;
use tower::Service;

pub mod auth;
pub mod data;

#[derive(Clone)]
struct JsonContentType<T> {
    inner_handler: T,
}

// impl<T> Service<Request> for JsonContentType<T>
// where
//     T: Service<Request> + Clone + 'static,
// {
//     type Error = Error;
//     type Response = Response;
//     type Future = Pin<Box<dyn Future<Output = Result<Response, Error>>>>;
//
//     fn poll_ready(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         self.inner_handler.poll_ready(cx)
//     }
//
//     fn call(&mut self, request: Request) -> Self::Future {
//         let mut this = self.clone();
//
//         Box::pin(async move {
//             let mut response = this.inner_handler.call(request).await?;
//             response.set_header("Content-Type", "application/json");
//             Ok(response)
//         })
//     }
// }
