use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};

use serde_repr::{Deserialize_repr, Serialize_repr};

use snowflake::Snowflake;

use self::application_command::ApplicationCommand;

pub mod application_command;
pub mod interactions;
pub mod permissions;
pub mod snowflake;

#[derive(Clone)]
pub struct DiscordBotApiClient {
    token: String,
    base_url: String,
    app_id: Snowflake,
    client: reqwest::Client,
}

impl DiscordBotApiClient {
    pub fn app_id(&self) -> Snowflake {
        self.app_id
    }

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
                        let mut tmp = HeaderValue::from_str(format!("Bot {}", token).as_str())
                            .expect("authorization header value");
                        tmp.set_sensitive(true);
                        tmp
                    };
                    map.append("Authorization", authorization_header_value);

                    let user_agent_header_value = {
                        HeaderValue::from_str(
                            format!("DiscordBot ({}, {})", bot_url, bot_version).as_str(),
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
        let url = format!("{}/v8/applications/{}/commands", base_url, app_id);
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
