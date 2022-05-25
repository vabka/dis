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
}

impl Get<DiscordBotApiClient> for BotContext {
    fn get(&self) -> &DiscordBotApiClient {
        &self.api_client
    }
}

impl Get<Storage> for BotContext {
    fn get(&self) -> &Storage {
        &self.store
    }
}

pub trait Get<T> {
    fn get(&self) -> &T;
}