use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Header, Validation};

use crate::entity::User;

pub async fn verify_jwt(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("Authorization")
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

    let key = "topsecret";

    let validation = Validation::new(Algorithm::HS256);

    let user = decode::<User>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::from(validation),
    ).map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?.claims; //decoding the jwt token

    request.extensions_mut().insert(user); //inserting the user into the request extensions (state

    Ok(next.run(request).await)
}
