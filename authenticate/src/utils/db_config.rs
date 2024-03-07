use std::{thread, time::Duration};

use tikv_client::{Key, RawClient};
use tokio::sync::OnceCell;

//intialize the client
static CLIENT : OnceCell<RawClient> = OnceCell::const_new();

//getting the client with OnceCell
pub async fn get_client() -> Result<RawClient, String>{
    match CLIENT.get() {
        Some(client) => {
            println!("db is connected");
            Ok(client.to_owned())
        },
        None => {
            match RawClient::new(vec!["127.0.0.1:2379"]).await {
                Ok(client) => {
                    CLIENT.get_or_init(|| async {client}).await;
                    println!("db is connected");
                    Ok(CLIENT.get().unwrap().to_owned())
                },
                Err(err) => Err(format!("Error in getting Client : {}" , err.to_string())),
            }
        }
    }
}

// pub async fn get_client() -> Result<RawClient, String> {
//     loop {
//         thread::sleep(Duration::from_secs(5));
//         match RawClient::new(vec!["127.0.0.1:2379"]).await {
//             Ok(client) => {
//                 println!("db connected");
//                 return Ok(client)
//             },
//             Err(err) => {
//                 thread::sleep(Duration::from_secs(5));
//                 eprintln!("Failed to connect to database: {}. Retrying...", err)
//             }
//         }
//     }
// }

pub async fn put_data(key : String , value : String) -> Result<bool , String> {
    match get_client().await {
        Ok(client) => {
            let _ = client.put(key , value).await;
            Ok(true)
        },
        Err(err) => Err(format!("Error in getting Client: {}" , err.to_string())),
    }
}

pub async fn get_data(key : String) -> Result<String , String>{
    match get_client().await {
        Ok(client) => {
            match client.get(key).await {
                Ok(vec_of_buffer) => {
                    match vec_of_buffer {
                        Some(data) => {
                            match String::from_utf8(data) {
                                Ok(final_str) => Ok(final_str),
                                Err(err) => Err(err.to_string()),
                            }
                        },
                        None => Err("internal server error".to_string()),
                    }
                },
                Err(err) => Err(format!("Error in getting the data {}" , err.to_string())),
            }
        },
        Err(err) => Err(format!("Error in getting the Client {}" , err.to_string())),
    }
}

pub async fn delete_data(key : String) -> Result<bool , String> {
    match get_client().await {
        Ok(client) => {
            let _ = client.delete(key).await;
            Ok(true)
        },
        Err(err) => Err(format!("Error in getting Client: {}" , err.to_string())),
    }
}

pub async fn get_all_data(start : String , end : String , limit:u32) -> Result<Vec<String>, String> {
    match get_client().await {
        Ok(client) => {
            match client.scan(Key::from(start)..Key::from(end) , limit).await {
                Ok(vec_of_kv) => {
                    let mut vec_of_tuple = Vec::new();
                    for kv in vec_of_kv{
                        match String::from_utf8(kv.1) {
                            Ok(value) =>  {
                                println!("{:?}" , value);
                                vec_of_tuple.push(value)
                            },
                            Err(err) => return Err(err.to_string()),
                        }
                    }
                    Ok( vec_of_tuple )
                },
                Err(err) => Err(err.to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    }
}



pub async fn delete_range(start : String , end : String) -> Result<bool , String> {
    match get_client().await {
        Ok(client) => {
            let _ = client.delete_range(Key::from(start)..Key::from(end)).await;
            Ok(true)
        },
        Err(err) => Err(format!("Error in getting Client: {}" , err.to_string())),
    }
}