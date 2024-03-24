use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{middleware, response::Response, Router};
use ehall_backend::routes::{route_static, routes_hello, routes_login, routes_user_info};
use ehall_backend::{error::Result, routes::AppState};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();
    info!("Starting server...");

    let app = AppState::new().await;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_login(app.clone()))
        .merge(routes_user_info(app.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(route_static())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 8070;
    let listner = tokio::net::TcpListener::bind(SocketAddr::new(ip, port))
        .await
        .unwrap();
    info!("Server started at {}:{}", ip, port);
    axum::serve(listner, routes_all).await.unwrap();

    Ok(())
}

// MARKER: response mapper

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}
