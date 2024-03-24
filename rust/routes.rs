use axum::{
    extract::{FromRef, Path, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};

use axum_auth::AuthBearer;

use crate::{
    error::Result,
    models::{LoginRequest, LoginResponse, ModelController, School, UserInfoResponse},
};

// MARKER: AppState
#[derive(Clone, FromRef)]
pub struct AppState {
    mc: ModelController,
}

pub async fn cas_login_handler(
    Path(school_name): Path<String>,
    State(app): State<AppState>,
    Json(login_data): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let school = School::try_from(school_name)?;
    app.mc
        .create_user(login_data, school)
        .await
        .map(|t| Json(LoginResponse::new_ok(t)))
}

pub async fn user_info_handler(
    AuthBearer(t): AuthBearer,
    State(app): State<AppState>,
) -> Result<Json<UserInfoResponse>> {
    app.mc
        .user_info(&t)
        .await
        .map(|t| Json(UserInfoResponse::new_ok(t)))
}

// MARKER: static routers
pub fn routes_login(app: AppState) -> Router {
    Router::new()
        .route("/:school_name/cas_login", post(cas_login_handler))
        .with_state(app)
}

pub fn routes_user_info(app: AppState) -> Router {
    Router::new()
        .route("/:school_name/user/info", get(user_info_handler))
        .with_state(app)
}

// MARKER: misc
impl AppState {
    pub async fn new() -> Self {
        Self {
            mc: ModelController::new().await.unwrap(),
        }
    }
}

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
