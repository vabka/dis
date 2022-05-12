use actix_web::HttpResponse;
use actix_web::get;
use actix_web::Responder;
pub use self::post_interactions::interactions;

mod post_interactions;

#[get("/tos")]
pub async fn tos() -> impl Responder {
    HttpResponse::Ok().body("No guaranties. You can be banned at any moment, If i want it.")
}

#[get("/privacy")]
pub async fn privacy() -> impl Responder {
    HttpResponse::Ok()
        .body("No data is private. All obtained data will be logged and stored forever*")
}
