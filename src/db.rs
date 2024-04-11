use mongodb::{Client, Collection};
use crate::models::document::Document;

pub async fn get_database() -> Collection<Document> {
    let mongodb_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");
    let mongodb_name = std::env::var("MONGODB_NAME").expect("MONGODB_NAME must be set in .env");

    let client = Client::with_uri_str(&mongodb_uri).await.expect("Failed to connect to MongoDB");

    let collection: Collection<Document> = client.database(&mongodb_name).collection("documents");
    collection
}

