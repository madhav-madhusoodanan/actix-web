use actix_web::{Scope, get, post, web,  HttpResponse, Responder};
use serde::Deserialize;
use crate::db::User::{ addUsers, getUsers };
#[derive(Deserialize)]
struct Body {
    name: String,
    password: String
}


#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello this is designers!")
}


#[post("/new")]
async fn new(body: web::Json<Body>) -> impl Responder {

    // verify the user using the bearer
    // verify is object details are valid
    // return all the objects by fetching from database

    let name = &body.name;
    let password = &body.password;
    match addUsers(name, password).await {
        Ok(_) =>  HttpResponse::Ok().body("That was smooth :)"),
        Err(_) =>     HttpResponse::Ok().body("That was NOT smooth :(")
    }
}

// #[get("/{id}")]
// async fn designer(params: web::Path<String>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Hello this is designer id:{} !", params.0))
// }

#[get("/users")]
async fn dummy() -> impl Responder {

    let users = match getUsers().await {
        Ok(users) => users,
        Err(_) => panic!("it failed"),
    };

    let mut result = String::new();
    for user in users {
        result.push_str(&format!("designer {} and password {} \n", user.name(), user.password()));
    }
    HttpResponse::Ok().body(result)
}


// this function could be located in a different module
pub fn index() -> Scope{ 
    web::scope("/designers")
           .service(root)
           .service(dummy)
           .service(new)
}
