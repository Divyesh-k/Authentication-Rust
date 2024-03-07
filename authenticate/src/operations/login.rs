use axum::{extract::{Path, Request}, http::StatusCode, response::{IntoResponse, Response}, Extension, Json
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::entity::TokenClaims;


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
pub async fn login_user() -> Result<Response , StatusCode> {

    let secret = "topsecret";

    let header = Header::new(Algorithm::HS256);

    // let tokenClaims = TokenClaims{
    //     sub : "" . to_string()
    // }

    // let token = encode(&header , &payload , &EncodingKey::from_secret(secret.as_ref())).unwrap();

    // inserting the user into the database

    // println!("{}" , token);

    Err(StatusCode::OK)
}