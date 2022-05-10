use anyhow::{anyhow, bail};
use log::{debug, error, info};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder, Method,
};
use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

use self::application_command::ApplicationCommand;

pub mod application_command;
pub mod permissions;

pub struct DiscordBotApiClient {
    token: String,
    base_url: String,
    app_id: Snowflake,
    client: reqwest::Client,
}

impl DiscordBotApiClient {
    pub fn new(
        token: &str,
        base_url: &str,
        bot_url: &str,
        bot_version: &str,
        app_id: Snowflake,
    ) -> Self {
        Self {
            token: token.to_owned(),
            base_url: base_url.to_owned(),
            app_id: app_id,
            client: {
                let headers = {
                    let mut map = HeaderMap::new();

                    let authorization_header_value = {
                        let mut tmp = HeaderValue::from_str(format!("Bot {token}").as_str())
                            .expect("authorization header value");
                        tmp.set_sensitive(true);
                        tmp
                    };
                    map.append("Authorization", authorization_header_value);

                    let user_agent_header_value = {
                        HeaderValue::from_str(
                            format!("DiscordBot ({bot_url}, {bot_version})").as_str(),
                        )
                        .expect("user agent header value")
                    };
                    map.append("User-Agent", user_agent_header_value);
                    map
                };
                ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .expect("Http client")
            },
        }
    }

    pub async fn create_application_command(
        &self,
        command: &ApplicationCommand,
    ) -> anyhow::Result<ApplicationCommand> {
        let base_url = &self.base_url;
        let app_id = self.app_id;
        let url = format!("{base_url}/v8/applications/{app_id}/commands");
        Ok(self
            .client
            .post(url)
            .json(command)
            .send()
            .await?
            .json::<ApplicationCommand>()
            .await?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelType(u8);
impl ChannelType {
    pub const GUILD_TEXT: Self = Self(0);
    pub const DM: Self = Self(1);
    pub const GUILD_VOICE: Self = Self(2);
    pub const GROUP_DM: Self = Self(3);
    pub const GUILD_CATEGORY: Self = Self(4);
    pub const GUILD_NEWS: Self = Self(5);
    pub const GUILD_NEWS_THREAD: Self = Self(10);
    pub const GUILD_PUBLIC_THREAD: Self = Self(11);
    pub const GUILD_PRIVATE_THREAD: Self = Self(12);
    pub const GUILD_STAGE_VOICE: Self = Self(13);
    pub const GUILD_DIRECTORY: Self = Self(14);
    pub const GUILD_FORUM: Self = Self(15);
}
