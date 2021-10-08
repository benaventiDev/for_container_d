//mod config;
/*
use cloud_pubsub::Client;
use dotenv::dotenv;
use serde_derive::Deserialize;
use std::sync::Arc;


#[derive(Deserialize)]
struct Config{
    topic: String,
    google_application_credentials: String,
    pubsub_subscription: String

}







pub async fn publish() {
    //dotenv().ok();
    //let config = crate::config::Config::from_env().unwrap();
    //let pubsub_subscription = "sopes1-sub";
    //Client::set_project(&mut slef, "sopes-p1");
    let parsed_env = envy::from_env::<Config>();
    if let Err(e) = parsed_env {
        eprintln!("ENV is not valid: {}", e);
        std::process::exit(1);
    }
    let config = parsed_env.unwrap();
    
    let pubsub = match Client::new(config.google_application_credentials).await {
         Err(e) => panic!("Failed to initialize pubsub: "),
         Ok(p) => p,
    };

    let topic = Arc::new(pubsub.topic(config.topic.clone()));
    match topic.clone().publish("Este es el mensaje de prueba").await{
        Err(e) => eprintln!("Failed sending message {}", e),
        Ok(p)=> println!(""),
    };

}*/