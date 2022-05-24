use crate::discord::rest::DiscordBotApiClient;
use crate::Storage;

#[derive(Clone)]
pub struct BotContext {
    store: Storage,
    api_client: DiscordBotApiClient,
}

impl BotContext {
    pub fn new(store: Storage, api_client: DiscordBotApiClient) -> Self {
        Self { store, api_client }
    }

    pub fn get_store(&self) -> &Storage {
        &self.store
    }
    pub fn get_api_client(&self) -> &DiscordBotApiClient {
        &self.api_client
    }
}
