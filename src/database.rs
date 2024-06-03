use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn get_mongo_client() -> Client {
    // Get the MongoDB connection string from an environment variable
    let mongo_uri =
        env::var("MONGO_URI").expect("You must set the MONGO_URI environment variable!");

    // Parse the connection string into an options struct
    let mut client_options = ClientOptions::parse(&mongo_uri).await.unwrap();

    // Get a reference to the database specified in the connection string
    client_options.app_name = Some("RustBackend".to_string());

    // Create the MongoDB client
    Client::with_options(client_options).unwrap()
}
