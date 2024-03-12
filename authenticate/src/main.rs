use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
pub mod operations;
pub mod utils;
pub mod entity;
pub mod middleware;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use utils::elastic_config::query_builder;
pub mod configuration;
use utils::elastic_config::search;

use crate::{middleware::{cors::cors_layer, jwt::verify_jwt}, operations::{login::*, register::register_user, services::{get_all_user, get_user}}, utils::{elastic_config::match_query_builder, logger}};

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: i32,
}

#[tokio::main]
async fn main() {
    // utils::db_config::get_client().await.unwrap();
    logger::startLogger();

    println!("logger is started");

    // Create a new Axum router
    let app = Router::new()
        // .with_state(Token("csrf-token".to_string()))
        .route("/health-check" , get(||async {"Server is restrating"}))
        .route("/login" , post(login_user))
        // .layer(from_fn(verify_jwt))
        .route("/register" , post(register_user))
        .route("/get-all-user", post(get_all_user))
        .route("/get/:id", get(get_user))
        .layer(ServiceBuilder::new().layer(cors_layer()));

    // Bind the server to address and port
    axum_server::bind("127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // delete("hii".to_string(), "1".to_string()).await;

    // let terms:Vec<String> = vec!["must".to_string() ,  "author:Aldous".to_string() , "should".to_string() , "author:Aldous".to_string()];
    // println!("{:#?}" , query_builder(terms.to_owned()));
    // let query = query_builder(terms);

    // let res = search("books".to_string(), query);
    // println!("{:#?}" , res.await);

    //match query builder
    // let terms:Vec<String> = vec!["author:Aldous".to_string() , "author:Aldous".to_string()];
    // let query = match_query_builder(terms.to_owned());
    // println!("{:#?}" , search("books".to_string(), query).await);

    
}   
