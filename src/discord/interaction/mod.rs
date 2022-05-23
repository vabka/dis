use crate::discord::rest::application_command::{
    ApplicationCommandOptionValue,
    ApplicationCommandType,
};
use crate::Snowflake;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::discord::Locale;

#[derive(Serialize, Debug)]
pub struct InteractionCallback {
    #[serde(rename = "type")]
    pub interaction_response_type: InteractionResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    Message(InteractionCallbackMessage),
}

#[derive(Serialize, Debug)]
pub struct InteractionCallbackMessage {
    // pub tts: Option<bool>,
    pub content: Option<String>,
    // pub embeds: Option<Box<[Embed]>>,
    // pub allowed_mentions: Option<AllowedMention>,
    // pub flags: Option<u64>,
    // pub components: Option<Box<[MessageComponent]>>,
    // pub attachments: Option<Box<[Attachment]>>
}

#[derive(Serialize)]
pub struct Embed {}

#[derive(Serialize)]
pub struct AllowedMention {}

impl InteractionCallback {
    pub fn pong() -> Self {
        InteractionCallback {
            interaction_response_type: InteractionResponseType::Pong,
            data: None,
        }
    }

    pub fn channel_message_with_source(message: InteractionCallbackMessage) -> Self {
        InteractionCallback {
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionCallbackData::Message(message)),
        }
    }
}

#[derive(Serialize_repr, Debug)]
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
    pub options: Option<Box<[ApplicationCommandInteractionDataOption]>>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    #[serde(rename = "type")]
    pub application_command_option_type: ApplicationCommandType,
    pub value: ApplicationCommandOptionValue,
}

#[derive(Deserialize, Debug)]
pub struct ResolvedData {}

#[derive(Deserialize, Debug)]
pub struct GuildMember {}

#[derive(Deserialize, Debug)]
pub struct User {}

#[derive(Deserialize, Debug)]
pub struct Message {}
