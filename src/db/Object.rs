use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use mongodb::bson::{Document};
use mongodb::{Client, options::ClientOptions, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    title: String,
    author: String,
}


pub async fn getObjects() -> Result<Vec<Object>, Error> {

    const url: &str = "mongodb+srv://madhav:madhav@cluster0.tlg4k.mongodb.net/";
    let client_options: ClientOptions = ClientOptions::parse(url).await?;
    let mut client: Client = Client::with_options(client_options)?;
    
    let db = client.database("rust");
    let object = db.collection::<Document>("object");

    let objects: Vec<Object> = Vec::new();
    Ok(objects)
}