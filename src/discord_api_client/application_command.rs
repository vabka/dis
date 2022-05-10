use std::collections::HashMap;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::snowflake::Snowflake;

use super::{permissions::Permissions, ChannelType};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandType(u8);
impl ApplicationCommandType {
    pub const SUB_COMMAND: Self = Self(1);
    pub const SUB_COMMAND_GROUP: Self = Self(2);
    pub const STRING: Self = Self(3);
    pub const INTEGER: Self = Self(4);
    pub const BOOLEAN: Self = Self(5);
    pub const USER: Self = Self(6);
    pub const CHANNEL: Self = Self(7);
    pub const ROLE: Self = Self(8);
    pub const MENTIONABLE: Self = Self(9);
    pub const NUMBER: Self = Self(10);
    pub const ATTACHMENT: Self = Self(11);
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: ApplicationCommandOptionChoiceValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i64),
    Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub command_type: Option<ApplicationCommandType>,
    pub application_id: Snowflake,
    pub guild_id: Option<Snowflake>,

    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,

    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,

    pub options: Option<Box<[ApplicationCommandOption]>>,

    pub default_member_permissions: Option<Permissions>,
    pub dm_permission: Option<bool>,
    pub version: Snowflake,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub command_type: ApplicationCommandType,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
    pub required: Option<bool>,
    pub choices: Option<Box<[ApplicationCommandOptionChoice]>>,
    pub options: Option<Box<[ApplicationCommandOption]>>,
    pub channel_types: Option<Box<[ChannelType]>>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub autocomplete: Option<bool>,
}
