use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use ehall_backend::{
    models::{LoginRequest, LoginResponse},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/:school_name/cas_login", post(cas_login_handler));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8070").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn cas_login_handler(
    Path(school_name): Path<String>,
    login_data: Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    unimplemented!()
}
