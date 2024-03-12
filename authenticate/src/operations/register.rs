use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::json;

use crate::{
    entity::{Message, User},
    utils::{db_config::put_data, elastic_config::create, logger},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

//register user with csrf token
// pub async fn register_user(Json(mut payload): Json<User>) -> Result<Response, StatusCode> {

//     //generate csrf token
//     let csrf_token: String = thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(16)
//         .map(char::from)
//         .collect();

//     //generate unique id
//     let id : String = thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(4)
//         .map(char::from)
//         .collect();

//     //making final unique id
//     let id: String = format!("U-{}", id);
//     println!("{}" , id);

//     //merging the csrf token with id
//     let csrf_token = format!("{}{}", csrf_token, id);

//     //assigning csrf token to the user
//     payload.csrf = Some(csrf_token.clone());

//     //inserting the user into the database
//     put_data(id , serde_json::to_string(&payload).unwrap()).await.map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;

//     //make response
//     let mut resp = Json(Message {
//         statuscode: 200,
//         message: "User registered successfully".to_string(),
//         data: "User".to_string(),
//     })
//     .into_response();

//     //set csrf token in the cookie
//     resp.headers_mut().insert(
//         header::SET_COOKIE,
//         HeaderValue::from_str(format!("csrf-token = {}", csrf_token).as_str()).unwrap(),
//     );
//     Ok(resp)
// }

//register user with jwt token
pub async fn register_user(Json(mut payload): Json<User>) -> Result<Response, StatusCode> {
    // generate unique id
    let id: String = format!(
        "U-{}",
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>()
    );

    payload.authentication = Some("Admin".to_string());

    //adding the data into the database
    put_data(
        id.to_owned(),
        serde_json::to_string(&payload).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("data added into the database");

    //adding data into elastic search
    let data = create("user".to_string(), id.to_owned() , json!(
        {
            "name" : payload.username,
            "email" : payload.email,
            "password" : payload.password,
            "authentication" : payload.authentication
        }
    )).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("data is added into the elastic{:#?}" , data);

    // make response
    let resp = Json(Message {
        statuscode: 200,
        message: "User registered successfully".to_string(),
        data: "User".to_string(),
    })
    .into_response();

    Ok(resp)
}
