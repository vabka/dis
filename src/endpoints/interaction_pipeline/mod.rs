mod command_handlers;
mod error;
mod ping;

use crate::discord::interactions::Interaction;
use crate::discord::interactions::InteractionCallback;

use crate::{DiscordBotApiClient, Storage};
use futures_util::future::LocalBoxFuture;
use std::future::Future;

pub use command_handlers::EchoCommandHandler;
pub use command_handlers::GetCommandHandler;
pub use command_handlers::LsCommandHandler;
pub use command_handlers::SetCommandHandler;
pub use error::InteractionError;
pub use ping::PingInteractionHandler;

pub type InteractionHandlerResult = Option<Result<InteractionCallback, InteractionError>>;

pub trait InteractionHandler {
    type Future: Future<Output = InteractionHandlerResult>;
    type Context;
    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future;
}

pub type Task<T> = LocalBoxFuture<'static, T>;

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

pub struct InteractionPipeline<TContext> {
    handlers: Vec<
        Box<dyn InteractionHandler<Future = Task<InteractionHandlerResult>, Context = TContext>>,
    >,
}

impl<TContext> InteractionPipeline<TContext> {
    pub fn new(
        handlers: Vec<
            Box<
                dyn InteractionHandler<Future = Task<InteractionHandlerResult>, Context = TContext>,
            >,
        >,
    ) -> Self {
        Self { handlers }
    }
    pub async fn handle(
        &self,
        interaction: Interaction,
        context: &TContext,
    ) -> Result<InteractionCallback, InteractionError> {
        for handler in &self.handlers {
            if let Some(result) = handler.handle(&interaction, context).await {
                return result;
            }
        }
        Err(InteractionError::NoHandlerFound)
    }
}
