use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[post("/api/interactions")]
async fn interactions() -> impl Responder {
    HttpResponse::NotImplemented().body("Sorry")
}

#[get("/tos")]
async fn tos() -> impl Responder {
    HttpResponse::Ok().body("No guaranties. You can be banned at any moment, If i want it.")
}

#[get("/privacy-policy")]
async fn privacy() -> impl Responder {
    HttpResponse::Ok()
        .body("No data is private. All obtained data will be logged and stored forever*")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = load_config();
    HttpServer::new(|| {
        App::new()
            .service(tos)
            .service(privacy)
            .service(interactions)
    })
    .bind(config.socket_addr)?
    .run()
    .await
}

struct Config {
    pub token: String,
    pub socket_addr: (String, u16),
    pub intent_bits: u64,
}
fn load_config() -> Config {
    Config {
        token: env::var("DISCORD_TOKEN").expect("token"),
        socket_addr: {
            let listen_addr = env::var("LISTEN").unwrap_or("127.0.0.1".to_string());
            let port = env::var("PORT")
                .map(|str| str.parse::<u16>().expect("valid port"))
                .unwrap_or(8080);
            (listen_addr, port)
        },
        intent_bits: {
            let intent_str = env::var("PERMISSIONS_INTEGER").expect("intents number");
            let intent_bits: u64 = intent_str.parse::<u64>().expect("valid number");
            intent_bits
        },
    }
}
