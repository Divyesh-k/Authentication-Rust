use serde::{Deserialize, Serialize};

#[derive(Debug , Serialize , Deserialize , Clone)]
pub struct User{
    pub username: String,
    pub password: String,
    pub email: String,
    pub authentication : Option<String>,
    pub jwt : Option<String>
}

#[derive(Debug , Serialize , Deserialize)]
pub struct Message<T>{
    pub statuscode: u16,
    pub message: String,
    pub data : T
}

#[derive(Debug , Serialize , Deserialize)]
pub struct TokenClaims {
    pub sub : String,
    pub iat : usize,
    pub exp : usize
}

#[derive(Debug , Serialize , Deserialize , Clone)]
pub struct Token(pub String);


