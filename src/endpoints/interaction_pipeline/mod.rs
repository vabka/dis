mod error;
mod command_handlers;
mod ping;
use crate::discord::interactions::Interaction;
use crate::discord::interactions::InteractionCallback;


use crate::{DiscordBotApiClient, Storage};
use futures_util::future::{LocalBoxFuture};
use std::future::{Future};

pub use error::InteractionError;
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

