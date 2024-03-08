use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
pub mod operations;
pub mod utils;
use elasticsearch::{http::response, SearchParts};
use middleware::{cors::cors_layer, csrf::verify_csrf};
use operations::{login::login_user, register::register_user};
pub mod entity;
pub mod middleware;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower::ServiceBuilder;
use utils::{elastic_config::{delete, get_client}, logger};

use crate::{
    middleware::jwt::verify_jwt,
    operations::services::{get_all_user, get_user},
    utils::{db_config::delete_range, elastic_config::create},
};
pub mod configuration;

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: i32,
}

#[tokio::main]
async fn main() {
    // // utils::db_config::get_client().await.unwrap();
    // logger::startLogger();

    // println!("logger is started");

    // // Create a new Axum router
    // let app = Router::new()
    //     // .with_state(Token("csrf-token".to_string()))
    //     .route("/health-check" , get(||async {"Server is restrating"}))
    //     .route("/login" , post(login_user))
    //     .layer(from_fn(verify_jwt))
    //     .route("/register" , post(register_user))
    //     .route("/get-all-user", post(get_all_user))
    //     .route("/get/:id", get(get_user))
    //     .layer(ServiceBuilder::new().layer(cors_layer()));

    // // Bind the server to address and port
    // axum_server::bind("127.0.0.1:5000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // delete("hii".to_string(), "1".to_string()).await;

    
}   
