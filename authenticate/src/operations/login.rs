use axum::{
    extract::{Path, Query, Request},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::json;

use crate::{
    entity::{Message, TokenClaims, User},
    utils::elastic_config::{query_builder, search},
};

//check user login csrf token
// pub async fn login_user(Extension(token) : Extension<Token> , Json(user): Json<User>) -> Result<Response, StatusCode> {
//         println!("{:?}" , token);
//         let id : String = format!("U-{}" , token.0.split("U-").into_iter().collect::<Vec<&str>>()[1].to_string());
//         let user_data = get_data(id).await.map_err(|_| StatusCode::UNAUTHORIZED)?;
//         let user_data : User = serde_json::from_str(&user_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//         if user_data.password != user.password || user_data.username != user.username{
//             return Err(StatusCode::UNAUTHORIZED);
//         }
//         let resp = Json(Message {
//             statuscode: 200,
//             message: "User login successfully".to_string(),
//             data: user_data,
//         }).into_response();

//         Ok(resp)
// }

//check login with jwt token
pub async fn login_user(payload: Json<User>) -> Result<Response, StatusCode> {
    let query = query_builder(vec![
        "must".to_string(),
        format!("email:{}", payload.email),
        format!("password:{}", payload.password),
    ]);

    let vec_of_users = search("user".to_string(), query).await.map_err(|err| {
        dbg!(err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("{:#?}", vec_of_users);

    if vec_of_users.is_empty() {
        return Ok(Json(Message {
            statuscode: 401,
            message: "Invlid cradantials".to_string(),
            data: json!({}),
        })
        .into_response());
    }

    let user = vec_of_users
        .get(0)
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    let user = user
        .as_object()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = user.get("_id").unwrap().to_string();

    let secret = "topsecret";

    let header = Header::new(Algorithm::HS256);

    let claims = TokenClaims {
        sub: id,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + std::time::Duration::from_secs(3600)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("jwt", token.to_owned())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();

    let mut response = Json(Message {
        statuscode: 200,
        message: "User login successfully".to_string(),
        data: json!({ "token": token }),
    }).into_response();

    response.headers_mut().insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}
