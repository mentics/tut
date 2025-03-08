use mongodb::{Client, Collection, options::ClientOptions};
use serde::Serialize;
use std::env;

pub async fn db(db_name: &str) -> mongodb::Database {
    // let mongo_uri = env::var("MONGODB_URI").expect("Required MONGODB_URI environment variable was not found");
    let atlas_password = env::var("ATLAS_PASSWORD")
        .expect("Required ATLAS_PASSWORD environment variable was not found");
    let mongo_uri = format!(
        "mongodb+srv://jshellman:{atlas_password}@journey.jseec.mongodb.net/?retryWrites=true&w=majority&appName=Journey"
    );
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database(db_name)
}

pub fn collection<T: Serialize + Send + Sync>(
    database: &mongodb::Database,
    collection_name: &str,
) -> Collection<T> {
    database.collection(collection_name)
}

pub async fn insert<T: Serialize + Send + Sync>(collection: &Collection<T>, document: T) {
    let insert_result = collection.insert_one(document).await.unwrap();
    println!("Inserted document with id: {:?}", insert_result.inserted_id);
}

pub async fn insert_many<T: Serialize + Send + Sync>(
    collection: &Collection<T>,
    docs: impl IntoIterator<Item = T>,
) {
    let insert_result = collection.insert_many(docs).await.unwrap();
    println!(
        "Inserted documents with ids: {:?}",
        insert_result.inserted_ids
    );
}
