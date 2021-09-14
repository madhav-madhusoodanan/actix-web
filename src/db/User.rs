use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use bson::oid::ObjectId;
use mongodb::{Client, options::ClientOptions, error::Error, bson::doc, options::FindOptions};
use crate::config::index::url;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
}
impl User {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

pub async fn getUsers() -> Result<Vec<User>, Error> {

    let client_options: ClientOptions = ClientOptions::parse(url).await?;
    let client: Client = Client::with_options(client_options)?;
    
    let db = client.database("rust");
    let user = db.collection::<User>("user");

    let cursor = user.find(None, None).await?;

    let objects: Vec<_> = cursor.try_collect().await?;
    Ok(objects)
}

pub async fn addUsers(name: &String, password: &String) -> Result<(), Error> {

    let client_options: ClientOptions = ClientOptions::parse(url).await?;
    let client: Client = Client::with_options(client_options)?;
    
    let db = client.database("rust");
    let user = db.collection::<User>("user");

    user.insert_one(User {name: name.to_string(), password: password.to_string()}, None).await?;
    Ok(())
}