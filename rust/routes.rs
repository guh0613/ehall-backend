use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::models::{LoginRequest, LoginResponse};

pub async fn cas_login_handler(
    Path(school_name): Path<String>,
    login_data: Json<LoginRequest>,
) -> Json<LoginResponse> {
    todo!()
}

// MARKER: static routers

pub fn route_static() -> Router {
    Router::new().nest_service("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn routes_hello() -> Router {
    Router::new().route("/hello", get(handler_hello))
}

pub async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("Hello <strong>World!!!</strong>")
}
