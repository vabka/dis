use actix_web::{get, post, HttpResponse, Responder, web};
use log::info;
use serde::{self, Deserialize};
use serde_repr::Deserialize_repr;

use crate::{discord_api_client::application_command::{Locale, ApplicationCommandType}, snowflake::Snowflake};

#[post("/api/interactions")]
pub async fn interactions(interaction: web::Json<Interaction>) -> impl Responder {
    info!("Interaction received! {:#?}", interaction);
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

#[derive(Deserialize, Debug)]
pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub version: u8,
    pub message: Option<Message>,
    pub locale: Option<Locale>,
    pub guild_locale: Option<Locale>,
}

#[derive(Deserialize_repr, Debug)]
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
pub struct InteractionData{
    pub id: Snowflake,
    pub name: String,
    #[serde(rename="type")]
    pub command_type: ApplicationCommandType,
    pub resolved: Option<ResolvedData>,
    pub options: Option<ApplicationCommandInteractionDataOption>,
    pub guild_id: Option<Snowflake>,
    pub custom_id: Option<String>,
    pub values: Option<Box<[SelectOptionValue]>>,
    pub target_id: Option<Snowflake>,
    pub components: Option<Box<[MessageComponent]>>
}

#[derive(Deserialize, Debug)]
pub struct MessageComponent;
#[derive(Deserialize, Debug)]
pub struct SelectOptionValue;

#[derive(Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption;
#[derive(Deserialize, Debug)]
pub struct ResolvedData;
#[derive(Deserialize, Debug)]
pub struct GuildMember;

#[derive(Deserialize, Debug)]
pub struct User;

#[derive(Deserialize, Debug)]
pub struct Message;
