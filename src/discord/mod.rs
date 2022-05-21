use serde_repr::{Deserialize_repr, Serialize_repr};


pub mod interaction;
pub mod voice;
pub mod gateway;
pub mod rest;

mod permissions;
mod snowflake;
mod locale;

pub use snowflake::Snowflake;
pub use locale::Locale;
pub use permissions::{Permissions, PermissionsMut, PermissionsProvider};


#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
}


