mod interactions;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web::{self}};
use actix_web::web::Json;

use serde::{self, Serialize};
use crate::discord::interactions::{Interaction, InteractionCallback, InteractionType};

pub use self::interactions::interactions;
#[get("/tos")]
pub async fn tos() -> impl Responder {
    HttpResponse::Ok().body("No guaranties. You can be banned at any moment, If i want it.")
}

#[get("/privacy")]
pub async fn privacy() -> impl Responder {
    HttpResponse::Ok()
        .body("No data is private. All obtained data will be logged and stored forever*")
}
