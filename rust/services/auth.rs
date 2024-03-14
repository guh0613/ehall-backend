use axum::{
    body::{Body, HttpBody}, extract::Request, http::{header, Extensions}, middleware::Next, response::Response
};
use axum_auth::AuthBasic;

use crate::{
    error::{Error, Result},
    models::AuthToken,
};

pub async fn require_auth(
    AuthBasic((_id, _)): AuthBasic,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    Ok(next.run(req).await)
}
