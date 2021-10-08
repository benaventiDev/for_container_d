use azure_sdk_cosmos::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use dotenv::dotenv;
#[path = "./tweet.rs"]
pub mod tweet;
use serde_json::json;


//#[tokio::main]
pub async fn cosmos_db_writer(comment: tweet::Tweet, hasHashs:bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = crate::config::Config::from_env().unwrap();
    let mut start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let mut _id : u128 = since_the_epoch.as_millis() + 58789;
    
    let account = config.cosmos.account; 
    let master_key = config.cosmos.key; 
    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let client = ClientBuilder::new(&account, authorization_token)?;
    let db_client = client.into_database_client(config.cosmos.db_name);
    let collection_client = db_client.into_collection_client(config.cosmos.container);

    let counter = _id.to_string();
    let customer: serde_json::Value;
    if hasHashs {
        customer = json!({
            "id" : counter,
            "usuario" : comment.username,
            "comentario" : comment.content,
            "upvotes" : comment.upvotes,
            "downvotes" : comment.downvotes,
            "fecha" : comment.fecha,
            "avatar" : comment.avatar,
            "hashtags": comment.hashtags,
            "pk": 1
        });
    }else{
        let hashs: Vec<String> = Vec::new();
        customer = json!({
            "id" : counter,
            "usuario" : comment.username,
            "comentario" : comment.content,
            "upvotes" : comment.upvotes,
            "downvotes" : comment.downvotes,
            "fecha" : comment.fecha,
            "avatar" : comment.avatar,
            "hashtags": hashs,
            "pk": 1
        });
    }


    _id = _id + 1;
    let response = collection_client.create_document()
            .with_partition_keys(PartitionKeys::new().push(1)?)    // let's pass the partition key
            .execute_with_document(&customer)                       // pass the document itself!
            .await?;


    /*
    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let client = ClientBuilder::new(&account, authorization_token)?;
    let db_client = client.into_database_client("ToDoDatabase");
    let collection_client = db_client.into_collection_client("ToDoList");

    let response = collection_client.create_document()
    .with_partition_keys(PartitionKeys::new().push(43)?)    // let's pass the partition key
    .execute_with_document(&customer)                       // pass the document itself!
    .await?;
    println!("{:?}", response);
    */
    Ok(())
}