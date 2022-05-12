use actix_web::{get, post, web::{self}, HttpResponse, Responder, HttpRequest, HttpMessage, Error};

use log::info;
use serde::{self, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    discord_api_client::application_command::{ApplicationCommandType, Locale},
    snowflake::Snowflake,
};

#[post("/interactions")]
pub async fn interactions(
    interaction: web::Json<Interaction>,
) -> actix_web::Result<impl Responder> {
    info!("Interaction received! {:#?}", interaction);
    return match interaction.interaction_type {
        InteractionType::Ping => Ok(web::Json(InteractionResponse::pong())),
        _ => todo!("Not covered")
    };
}

#[get("/tos")]
pub async fn tos() -> impl Responder {
    HttpResponse::Ok().body("No guaranties. You can be banned at any moment, If i want it.")
}

#[get("/privacy")]
pub async fn privacy() -> impl Responder {
    HttpResponse::Ok()
        .body("No data is private. All obtained data will be logged and stored forever*")
}

#[derive(Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub interaction_response_type: InteractionResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<()>,
}

impl InteractionResponse {
    pub fn pong() -> Self {
        InteractionResponse {
            interaction_response_type: InteractionResponseType::Pong,
            data: None,
        }
    }
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}

#[derive(Deserialize, Debug)]
pub struct Interaction {
    pub application_id: Snowflake,
    pub id: Snowflake,
    pub token: String,

    #[serde(rename = "type")]
    pub interaction_type: InteractionType,

    pub user: Option<User>,

    pub data: Option<InteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub message: Option<Message>,
    pub locale: Option<Locale>,
    pub guild_locale: Option<Locale>,
    pub version: u8,
}

#[derive(Deserialize_repr, Debug, Eq, PartialEq)]
#[repr(u8)]
#[non_exhaustive]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Deserialize, Debug)]
pub struct InteractionData {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub command_type: ApplicationCommandType,
    pub resolved: Option<ResolvedData>,
    pub options: Option<ApplicationCommandInteractionDataOption>,
    pub guild_id: Option<Snowflake>,
    pub custom_id: Option<String>,
    pub values: Option<Box<[SelectOptionValue]>>,
    pub target_id: Option<Snowflake>,
    pub components: Option<Box<[MessageComponent]>>,
}

#[derive(Deserialize, Debug)]
pub struct MessageComponent {}

#[derive(Deserialize, Debug)]
pub struct SelectOptionValue {}

#[derive(Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption {}

#[derive(Deserialize, Debug)]
pub struct ResolvedData {}

#[derive(Deserialize, Debug)]
pub struct GuildMember {}

#[derive(Deserialize, Debug)]
pub struct User {}

#[derive(Deserialize, Debug)]
pub struct Message {}
