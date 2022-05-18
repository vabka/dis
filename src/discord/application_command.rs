use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::snowflake::Snowflake;

use super::{permissions::Permissions, ChannelType};

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ApplicationCommandType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub name_localizations: Option<HashMap<Locale, String>>,

    pub value: ApplicationCommandOptionValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandOptionValue {
    Str(String),
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
    pub name_localizations: Option<HashMap<Locale, String>>,

    pub description: String,
    pub description_localizations: Option<HashMap<Locale, String>>,

    pub options: Option<Box<[ApplicationCommandOption]>>,

    pub default_member_permissions: Option<Permissions>,
    pub dm_permission: Option<bool>,
    pub version: Snowflake,
}

pub struct ApplicationCommandBuilder<'builder> {
    id: Snowflake,
    command_type: Option<ApplicationCommandType>,
    application_id: Snowflake,
    guild_id: Option<Snowflake>,
    name: &'builder str,
    name_localizations: Option<HashMap<Locale, String>>,
    description: &'builder str,
    description_localizations: Option<HashMap<Locale, String>>,
    options: Option<Vec<ApplicationCommandOption>>,
    default_member_permissions: Option<Permissions>,
    dm_permission: Option<bool>,
    version: Snowflake,
}

impl<'builder> ApplicationCommandBuilder<'builder> {
    pub fn for_application(name: &'builder str, application_id: Snowflake) -> ApplicationCommandBuilder<'builder> {
        ApplicationCommandBuilder {
            id: Snowflake::default(),
            command_type: None,
            application_id,
            guild_id: None,
            name,
            name_localizations: None,
            description: "",
            description_localizations: None,
            options: None,
            default_member_permissions: None,
            dm_permission: None,
            version: Snowflake::default(),
        }
    }
    pub fn with_option(mut self, option: ApplicationCommandOption) -> Self {
        if let Some(options) = &mut self.options {
            options.push(option);
        } else {
            self.options = Some(vec![option])
        }
        self
    }
    pub fn with_description(self, description: &'builder str) -> ApplicationCommandBuilder<'builder> {
        ApplicationCommandBuilder {
            description,
            ..self
        }
    }
    pub fn build(self) -> ApplicationCommand {
        ApplicationCommand {
            id: self.id,
            command_type: self.command_type,
            application_id: self.application_id,
            guild_id: self.guild_id,
            name: self.name.to_string(),
            name_localizations: self.name_localizations,
            description: self.description.to_string(),
            description_localizations: self.description_localizations,
            options: self.options.map(Vec::into_boxed_slice),
            default_member_permissions: self.default_member_permissions,
            dm_permission: self.dm_permission,
            version: self.version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub command_type: ApplicationCommandType,
    pub name: String,
    pub name_localizations: Option<HashMap<Locale, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<Locale, String>>,
    pub required: Option<bool>,
    pub choices: Option<Box<[ApplicationCommandOptionChoice]>>,
    pub options: Option<Box<[ApplicationCommandOption]>>,
    pub channel_types: Option<Box<[ChannelType]>>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub autocomplete: Option<bool>,
}

pub struct ApplicationCommandOptionBuilder<'builder> {
    command_type: ApplicationCommandType,
    name: &'builder str,
    name_localizations: Option<HashMap<Locale, String>>,
    description: &'builder str,
    description_localizations: Option<HashMap<Locale, String>>,
    required: Option<bool>,
    choices: Option<Vec<ApplicationCommandOptionChoice>>,
    options: Option<Vec<ApplicationCommandOption>>,
    channel_types: Option<Vec<ChannelType>>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    autocomplete: Option<bool>,
}

impl<'builder> ApplicationCommandOptionBuilder<'builder> {
    pub fn string_option(name: &'builder str) -> ApplicationCommandOptionBuilder<'builder> {
        ApplicationCommandOptionBuilder {
            command_type: ApplicationCommandType::String,
            name,
            description: "",
            description_localizations: None,
            required: None,
            choices: None,
            options: None,
            channel_types: None,
            min_value: None,
            max_value: None,
            name_localizations: None,
            autocomplete: None,
        }
    }
    pub fn with_description(self, description: &'builder str) -> ApplicationCommandOptionBuilder<'builder> {
        ApplicationCommandOptionBuilder {
            description,
            ..self
        }
    }
    pub fn required(mut self) -> Self {
        self.required = Some(true);
        self
    }

    pub fn not_required(mut self) -> Self {
        self.required = Some(false);
        self
    }

    pub fn build(self) -> ApplicationCommandOption {
        ApplicationCommandOption {
            command_type: self.command_type,
            name: self.name.to_string(),
            name_localizations: self.name_localizations,
            description: self.description.to_string(),
            description_localizations: self.description_localizations,
            required: self.required,
            choices: self.choices.map(Vec::into_boxed_slice),
            options: self.options.map(Vec::into_boxed_slice),
            channel_types: self.channel_types.map(Vec::into_boxed_slice),
            min_value: self.min_value,
            max_value: self.max_value,
            autocomplete: self.autocomplete,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Locale(String);

impl Locale {
    pub fn code(&self) -> &str {
        &self.0
    }

    pub const DANISH: &'static str = "da";
    pub fn danish() -> Self {
        Self(Self::DANISH.to_owned())
    }

    pub const GERMAN: &'static str = "de";
    pub fn german() -> Self {
        Self(Self::GERMAN.to_owned())
    }

    pub const ENGLISH_UK: &'static str = "en-GB";
    pub fn english_uk() -> Self {
        Self("en-GB".to_owned())
    }

    pub fn english_us() -> Self {
        Self("en-US".to_owned())
    }
    pub fn spanish() -> Self {
        Self("es-ES".to_owned())
    }
    pub fn french() -> Self {
        Self("fr".to_owned())
    }
    pub fn croatian() -> Self {
        Self("hr".to_owned())
    }
    pub fn italian() -> Self {
        Self("it".to_owned())
    }
    pub fn lithuanian() -> Self {
        Self("lt".to_owned())
    }
    pub fn hungarian() -> Self {
        Self("hu".to_owned())
    }
    pub fn dutch() -> Self {
        Self("nl".to_owned())
    }
    pub fn norwegian() -> Self {
        Self("no".to_owned())
    }
    pub fn polish() -> Self {
        Self("pl".to_owned())
    }
    pub fn portuguese_brazilian() -> Self {
        Self("pt-BR".to_owned())
    }
    pub fn romanian() -> Self {
        Self("ro".to_owned())
    }
    pub fn finnish() -> Self {
        Self("fi".to_owned())
    }
    pub fn swedish() -> Self {
        Self("sv-SE".to_owned())
    }
    pub fn vietnamese() -> Self {
        Self("vi".to_owned())
    }
    pub fn turkish() -> Self {
        Self("tr".to_owned())
    }
    pub fn czech() -> Self {
        Self("cs".to_owned())
    }
    pub fn greek() -> Self {
        Self("el".to_owned())
    }
    pub fn bulgarian() -> Self {
        Self("bg".to_owned())
    }
    pub fn russian() -> Self {
        Self("ru".to_owned())
    }
    pub fn ukrainian() -> Self {
        Self("uk".to_owned())
    }
    pub fn hindi() -> Self {
        Self("hi".to_owned())
    }
    pub fn thai() -> Self {
        Self("th".to_owned())
    }
    pub fn chinese_china() -> Self {
        Self("zh-CN".to_owned())
    }
    pub fn japanese() -> Self {
        Self("ja".to_owned())
    }
    pub fn chinese_taiwan() -> Self {
        Self("zh-TW".to_owned())
    }
    pub fn korean() -> Self {
        Self("ko".to_owned())
    }
}
