use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use endpoints::index;

use crate::configuration::BotConfig;
use crate::discord_authorization::DiscordAuthorization;
use crate::domain::store::Storage;
use discord::Snowflake;

use crate::endpoints::{interactions, privacy, tos};
use domain::bot::BotContext;
use domain::interaction_handlers::*;
use domain::interaction_pipeline::InteractionPipeline;

mod configuration;
mod discord;
mod discord_authorization;
mod domain;
mod endpoints;
// mod typed_interaction;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    env_logger::init();
    let config = BotConfig::load_env()?;
    let public_key = config.public_key;
    let store = Storage::new(config.storage_path.as_str(), None)?;

    let client = discord::rest::DiscordBotApiClient::new(
        config.token.as_str(),
        config.base_url.as_str(),
        config.bot_url.as_str(),
        "0.1",
        config.app_id,
    );
    // declare_commands(&client).await?;
    let bot_context = BotContext::new(store, client);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bot_context.clone()))
            .app_data(web::Data::new(InteractionPipeline::<BotContext>::new(
                vec![
                    Box::new(PingInteractionHandler),
                    Box::new(InteractionCommandInteractionHandler::from(
                        EchoCommandHandler,
                    )),
                    Box::new(InteractionCommandInteractionHandler::from(
                        SetCommandHandler,
                    )),
                    Box::new(InteractionCommandInteractionHandler::from(LsCommandHandler)),
                    Box::new(InteractionCommandInteractionHandler::from(
                        GetCommandHandler,
                    )),
                ],
            )))
            .wrap(middleware::Compress::default())
            .service(privacy)
            .service(tos)
            .service(index)
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
