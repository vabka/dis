use std::future::{Future, ready};
use futures_util::future::BoxFuture;
use crate::discord::interactions::{Interaction, InteractionCallback, InteractionType};
use crate::endpoints::post_interactions::InteractionError;

pub type InteractionHandlerResult = Option<Result<InteractionCallback, InteractionError>>;

pub trait InteractionHandler {
    type Future: Future<Output=InteractionHandlerResult>;
    fn handle(&self, interaction: &Interaction) -> Self::Future;
}

pub type Task<T> = BoxFuture<'static, T>;

pub struct InteractionPipeline {
    handlers: Vec<Box<dyn InteractionHandler<Future=Task<InteractionHandlerResult>>>>,
}

impl InteractionPipeline {
    pub fn new(handlers: Vec<Box<dyn InteractionHandler<Future=Task<InteractionHandlerResult>>>>) -> Self {
        Self {
            handlers
        }
    }
    pub async fn handle(&self, interaction: Interaction) -> Result<InteractionCallback, InteractionError> {
        for handler in &self.handlers {
            if let Some(result) = handler.handle(&interaction).await {
                return result;
            }
        }
        Err(InteractionError::NoHandlerFound)
    }
}

pub struct PingInteractionHandler;

impl InteractionHandler for PingInteractionHandler {
    type Future = Task<InteractionHandlerResult>;

    fn handle(&self, interaction: &Interaction) -> Self::Future {
        if interaction.interaction_type == InteractionType::Ping {
            Box::pin(ready(Some(Ok(InteractionCallback::pong()))))
        } else {
            Box::pin(ready(None))
        }
    }
}