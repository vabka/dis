mod error;
mod ping;


use crate::Storage;
use futures_util::future::LocalBoxFuture;
use std::future::Future;

pub use crate::domain::command_handlers::EchoCommandHandler;
pub use crate::domain::command_handlers::GetCommandHandler;
pub use crate::domain::command_handlers::LsCommandHandler;
pub use crate::domain::command_handlers::SetCommandHandler;
pub use error::InteractionError;
pub use ping::PingInteractionHandler;
use crate::discord::interaction::{Interaction, InteractionCallback};
use crate::discord::rest::DiscordBotApiClient;

pub type InteractionHandlerResult = Option<Result<InteractionCallback, InteractionError>>;

pub trait InteractionHandler {
    type Future: Future<Output=InteractionHandlerResult>;
    type Context;
    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future;
}

pub type Task<T> = LocalBoxFuture<'static, T>;

pub struct InteractionPipeline<TContext> {
    handlers: Vec<
        Box<dyn InteractionHandler<Future=Task<InteractionHandlerResult>, Context=TContext>>,
    >,
}

impl<TContext> InteractionPipeline<TContext> {
    pub fn new(
        handlers: Vec<
            Box<
                dyn InteractionHandler<Future=Task<InteractionHandlerResult>, Context=TContext>,
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
