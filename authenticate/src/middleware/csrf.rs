use axum::{body::{to_bytes, Body, HttpBody}, extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use crate::{entity::{Token, User}, utils::db_config::get_data};

pub async fn verify_csrf(mut request : Request , next : Next) -> Result<Response, StatusCode> {
    let csrf_token = request.headers().get("csrf-token").ok_or_else(|| StatusCode::UNAUTHORIZED)?;
    let token = csrf_token.to_str().map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?.to_owned();
    request.extensions_mut().insert(Token(token));
    Ok(next.run(request).await)
}

