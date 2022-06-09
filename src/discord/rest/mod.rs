use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::discord::rest::application_command::ApplicationCommand;
use crate::Snowflake;

pub mod application_command;


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
            app_id,
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

    // TODO wrap error
    pub async fn create_application_command(
        &self,
        command: &ApplicationCommand,
    ) -> Result<ApplicationCommand, reqwest::Error> {
        let base_url = &self.base_url;
        let app_id = self.app_id;
        let url = format!("{}/v8/applications/{}/commands", base_url, app_id);
        self
            .client
            .post(url)
            .json(command)
            .send()
            .await?
            .json::<ApplicationCommand>()
            .await
    }
}
