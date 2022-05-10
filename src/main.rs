use std::env;

use actix_web::{HttpServer, App, body::MessageBody};

use dotenv::dotenv;


use snowflake::Snowflake;

use crate::{endpoints::{tos, privacy, interactions}};

pub mod discord_api_client;
pub mod endpoints;
pub mod snowflake;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = load_config();
    // let client = DiscordBotApiClient::new(
    //     config.token.as_str(),
    //     &config.base_url.as_str(),
    //     &config.bot_url.as_str(),
    //     "0.1",
    //     config.app_id,
    // );
    // let options = vec![ApplicationCommandOption {
    //     command_type: ApplicationCommandType::STRING,
    //     name: "text".to_owned(),
    //     name_localizations: None,
    //     description: "text to echo".to_owned(),
    //     description_localizations: None,
    //     required: Some(true),
    //     choices: None,
    //     options: None,
    //     channel_types: None,
    //     min_value: None,
    //     max_value: None,
    //     autocomplete: Some(false),
    // }];
    // let application_command = ApplicationCommand {
    //     id: Snowflake::zero(),
    //     command_type: Some(ApplicationCommandType::SUB_COMMAND),
    //     application_id: config.app_id,
    //     guild_id: None,
    //     name: "echo".to_owned(),
    //     name_localizations: None,
    //     description: "echo command".to_owned(),
    //     description_localizations: None,
    //     options: Some(options.into_boxed_slice()),
    //     default_member_permissions: None,
    //     dm_permission: None,
    //     version: Snowflake::zero(),
    // };
    // let created_command = client
    //     .create_application_command(&application_command)
    //     .await?;
    // info!("Created command: {:#?}", created_command);

    HttpServer::new(|| {
        App::new()
            .service(tos)
            .service(privacy)
            .service(interactions)
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
        base_url: env::var("BASE_URL").unwrap_or("https://discord.com/api".to_owned()),
        app_id: env::var("CLID").expect("CLID").parse().expect("Valid CLID"),
        bot_url: env::var("URL").unwrap_or("TODO".to_owned()),
    }
}
