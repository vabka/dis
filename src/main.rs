use std::env;

use actix_web::dev::Service;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpMessage, HttpServer};
use dotenv::dotenv;
use futures_util::FutureExt;

use crate::discord_authorization::DiscordAuthorization;
use reqwest::header::HeaderValue;
use snowflake::Snowflake;

use crate::endpoints::{interactions, privacy, tos};

mod discord_api_client;
mod discord_authorization;
mod endpoints;
mod snowflake;
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    env_logger::init();
    let config = load_config();
    let public_key = config.public_key;
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .service(privacy)
            .service(tos)
            .service(
                web::scope("/api")
                    .wrap(DiscordAuthorization::new(public_key))
                    .service(interactions),
            )
    })
    .bind(config.socket_addr)?
    .run()
    .await?;
    Ok(())
}

struct Config {
    pub token: String,
    pub socket_addr: (String, u16),
    pub intent_bits: u64,
    pub app_id: Snowflake,
    pub bot_url: String,
    pub base_url: String,
    pub public_key: ed25519_dalek::PublicKey,
}

fn load_config() -> Config {
    Config {
        token: env::var("DISCORD_TOKEN").expect("token"),
        socket_addr: {
            let listen_addr = env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1".to_string());
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
        base_url: env::var("BASE_URL").unwrap_or_else(|_| "https://discord.com/api".to_owned()),
        app_id: env::var("CLID").expect("CLID").parse().expect("Valid CLID"),
        bot_url: env::var("URL").unwrap_or_else(|_| "TODO".to_owned()),
        public_key: env::var("PUBLIC_KEY")
            .map_err(|err| anyhow::anyhow!(err))
            .and_then(parse_hex)
            .expect("Valid public PUBLIC_KEY"),
    }
}

fn parse_hex(text: String) -> anyhow::Result<ed25519_dalek::PublicKey> {
    let byte_vec = hex::decode(text)?;
    let public_key = ed25519_dalek::PublicKey::from_bytes(&byte_vec)?;
    Ok(public_key)
}
