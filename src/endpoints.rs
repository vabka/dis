use actix_web::{get, post, HttpResponse, Responder};

#[post("/api/interactions")]
pub async fn interactions() -> impl Responder {
    HttpResponse::NotImplemented().body("Sorry")
}

#[get("/tos")]
pub async fn tos() -> impl Responder {
    HttpResponse::Ok().body("No guaranties. You can be banned at any moment, If i want it.")
}

#[get("/privacy-policy")]
pub async fn privacy() -> impl Responder {
    HttpResponse::Ok()
        .body("No data is private. All obtained data will be logged and stored forever*")
}
