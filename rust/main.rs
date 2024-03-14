use axum::{
    middleware,
    response::Response,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use ehall_backend::routes::{route_static, routes_hello, routes_login};
use ehall_backend::{
    error::{Error, Result},
    routes::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app = AppState::new().await;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_login(app.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(route_static());
    // .route("/:school_name/cas_login", post(cas_login_handler));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8070").await.unwrap();
    axum::serve(listner, routes_all).await.unwrap();

    Ok(())
}

// MARKER: response mapper

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}
