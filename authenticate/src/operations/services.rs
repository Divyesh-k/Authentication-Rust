use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}, Json};

use crate::{entity::Message, utils::db_config::{get_all_data, get_data}};

pub async fn get_all_user() -> Result<Response , StatusCode>{
    match get_all_data("U-".to_string() , "".to_string() , 1000).await {
        Ok(data) => {
            Ok(
                Json(Message {
                    statuscode: 200,
                    message: "All user data".to_string(),
                    data,
                }).into_response()
            )
        },
        Err(err) => {
            eprintln!("Error in getting all user: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}  

pub async fn get_user(Path(id) : Path<String>) -> Result<Response , StatusCode>{
    match get_data(id).await {
        Ok(data) => {
            Ok(
                Json(Message {
                    statuscode: 200,
                    message: "User data".to_string(),
                    data,
                }).into_response()
            )
        },
        Err(err) => {
            eprintln!("Error in getting user: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}