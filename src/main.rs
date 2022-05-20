#![allow(dead_code)]
#![warn(unused_imports)]

use std::env;
use std::sync::{Arc};
use tokio::sync::{RwLock};
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

use crate::discord_authorization::DiscordAuthorization;
use discord::snowflake::Snowflake;
use crate::discord::DiscordBotApiClient;
use crate::domain::declare_commands;
use crate::domain::store::Storage;

use crate::endpoints::{interactions, privacy, tos};
use crate::endpoints::interaction_pipeline::{BotContext, EchoCommandHandler, GetCommandHandler, InteractionPipeline, LsCommandHandler, PingInteractionHandler, SetCommandHandler};

mod discord;
mod discord_authorization;
mod domain;
mod endpoints;
// mod typed_interaction;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    env_logger::init();
    let config = load_config();
    let public_key = config.public_key;
    let store = Storage::new(config.storage_path.as_str(), None)?;

    let client = DiscordBotApiClient::new(config.token.as_str(), config.base_url.as_str(), config.bot_url.as_str(), "0.1", config.app_id);
    // declare_commands(&client).await?;
    let bot_context = BotContext::new(store, client);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bot_context.clone()))
            .app_data(web::Data::new(InteractionPipeline::new(vec![
                Box::new(PingInteractionHandler),
                Box::new(EchoCommandHandler),
                Box::new(SetCommandHandler),
                Box::new(LsCommandHandler),
                Box::new(GetCommandHandler),
            ])))
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
    pub storage_path: String,
}

fn load_config() -> Config {
    Config {
        token: env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN"),
        socket_addr: {
            let listen_addr = env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = env::var("PORT")
                .map(|str| str.parse::<u16>().expect("valid port"))
                .unwrap_or(8080);
            (listen_addr, port)
        },
        intent_bits: {
            let intent_str = env::var("PERMISSIONS_INTEGER").expect("PERMISSIONS_INTEGER");
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
        storage_path: env::var("STORAGE_PATH").expect("Path to file db"),
    }
}

fn parse_hex(text: String) -> anyhow::Result<ed25519_dalek::PublicKey> {
    let byte_vec = hex::decode(text)?;
    let public_key = ed25519_dalek::PublicKey::from_bytes(&byte_vec)?;
    Ok(public_key)
}
