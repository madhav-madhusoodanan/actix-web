use actix_web::{Scope, post, get, web,  HttpResponse, Responder};
use mongodb::bson::doc;
use crate::config::index::key_function as gen;
// use hmac::{Hmac, NewMac};
// use sha2::Sha256;
// use futures::stream::TryStreamExt;

use serde::Deserialize;

#[derive(Deserialize)]
struct Body {
    username: String,
}


#[get("/")]
async fn root() -> impl Responder {

    // get and return all objects

    HttpResponse::Ok().body("Hello this is paintings!")
}

#[post("/new")]
async fn new() -> impl Responder {

    // verify the user using the bearer
    // verify is object details are valid
    // return all the objects by fetching from database

    HttpResponse::Ok().body(format!("Hello you just added an object!"))
}

#[get("/{id}")]
async fn painting(params: web::Path<String>) -> impl Responder {

    // get a specific object

    HttpResponse::Ok().body(format!("Hello this is paintings id:{:#?} !", params.get(..)))
}

#[post("/{id}")]
async fn order(params: web::Path<String>, body: web::Json<Body>) -> impl Responder {

    // first authenticate the user
    // then get the object
    // check if the order request is valid (might not be necessary)
    // then return the object (or) redirect to the objects page
    
    let username = &body.username;
    HttpResponse::Ok().body(format!("Hello this is paintings id: {:#?} and your name is {} !", params.get(..), username))
}

// this function could be located in a different module
pub fn index() -> Scope{ 
    web::scope("/paintings")
           .service(root)
           .service(painting)
           .service(order)
}
