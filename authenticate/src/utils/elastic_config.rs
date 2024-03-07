use elasticsearch::{
    http::{response::Response, transport::Transport}, CreateParts, DeleteParts, Elasticsearch, Error, IndexParts, Search, SearchParts, UpdateParts
};
use serde_json::{json, Value};

pub async fn get_client() -> Result<Elasticsearch, String> {
    let transport = Transport::single_node("http://localhost:9200").map_err(|e| e.to_string())?;
    let client = Elasticsearch::new(transport);
    Ok(client)
}

pub async fn search(index: String, query: Value) -> Result<Vec<Value>, String> {
    match get_client().await {
        Ok(client) => {
            let response = client
                .search(SearchParts::Index(&[index.as_str()]))
                .body(query)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            let body: Value = response
                .json::<Value>()
                .await
                .map_err(|err| err.to_string())?;

            let hits = body["hits"]["hits"]
                .as_array()
                .ok_or_else(|| "Error in unwarping hits".to_string())?
                .to_vec();
            Ok(hits)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create(index : String , id : String , query : Value) -> Result<() , String>{
    match get_client().await {
        Ok(client) => {
            client.create(CreateParts::IndexId(index.as_str() , id.as_str()))
                .body(query)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn delete(index : String , id : String) -> Result<() , String> {
    match get_client().await {
        Ok(client) => {
            client.delete(DeleteParts::IndexId(index.as_str() , id.as_str()))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn put(index : String , id : String , query : Value) -> Result<() , String> {
    match get_client().await {
        Ok(client) => {
            client.update(UpdateParts::IndexId(index.as_str(), id.as_str()))
                .body(query)
                .send()
                .await
                .map_err(|err| err.to_string())?;
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}