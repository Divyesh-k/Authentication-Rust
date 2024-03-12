use std::{collections::HashMap, hash::Hash, string};

use elasticsearch::{
    http::{response::Response, transport::Transport}, CreateParts, DeleteParts, Elasticsearch, Error, IndexParts, Search, SearchParts, UpdateParts
};
use log::Record;
use serde_json::{json, Value};

//get the client
pub async fn get_client() -> Result<Elasticsearch, String> {
    let transport = Transport::single_node("http://localhost:9200").map_err(|e| e.to_string())?;
    let client = Elasticsearch::new(transport);
    Ok(client)
}

//search the document
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
                .ok_or_else(|| "Hits is not available:".to_string())?
                .to_vec();
            Ok(hits)
        }
        Err(e) => Err(e.to_string()),
    }
}

//create the document
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

//delete the document
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

//update the document
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

//boolean query builder
pub fn query_builder(terms: Vec<String>) -> Value {
    let bool_terms = vec!["should".to_string(), "must".to_string(), "must_not".to_string()];
    let mut query: HashMap<String, Vec<HashMap<String, Value>>> = HashMap::new();
    let mut current_bool_term = String::new(); 

    for i in 0..terms.len() {
        if bool_terms.contains(&terms[i]) {
            current_bool_term = terms[i].clone();
        } else {
            if !current_bool_term.is_empty() {

                //splitting the term into key value pair
                let fields = match terms[i].split_once(":") {
                    Some((key, value)) => (key.to_string(), value.to_string()),
                    None => continue,
                };

                //get the current term map
                let term_map = query.entry(current_bool_term.clone()).or_insert_with(Vec::new);

                //create a match statement
                let mut match_statement = HashMap::new();
                match_statement.insert("match".to_string(), json!({ fields.0: fields.1 }));

                //push the match statement to the term map
                term_map.push(match_statement);
            }
        }
    }
    //return the query
    json!({
        "query": {
            "bool": query
        }
    })
}

//match query builder
pub fn match_query_builder(terms: Vec<String>) -> Value {
    let mut query: HashMap<String, Value> = HashMap::new();
    for i in 0..terms.len() {
        let fields = match terms[i].split_once(":") {
            Some((key, value)) => (key.to_string(), value.to_string()),
            None => continue,
        };
        query.insert("match".to_string(), json!({ fields.0: fields.1 }));
    }
    json!({
        "query": query
    })
}


