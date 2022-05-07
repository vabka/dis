use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/api/interactions")]
async fn interactions() -> impl Responder {
    HttpResponse::NotImplemented().body("Sorry")
}

#[get("/tos")]
async fn tos() -> impl Responder{
    HttpResponse::Ok().body("???")
}

#[get("/privacy-policy")]
async fn privacy() -> impl Responder {
    HttpResponse::Ok().body("No data is private. All obtained data will be logged and stored forever*")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = {
        let intent_str = env::var("PERMISSIONS_INTEGER").expect("intents number");
        let intent_bits: u64 = intent_str.parse::<u64>().expect("valid number");
        intent_bits
    };

    let socket_addr = {
        let listen_addr = env::var("LISTEN")
            .unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT")
            .map(|str| str.parse::<u16>().expect("valid port"))
            .unwrap_or(8080);
        (listen_addr, port)
    };

    HttpServer::new(|| {
        App::new()
            .service(tos)
            .service(privacy)
            .service(interactions)
    })
        .bind(socket_addr)?
        .run()
        .await
}