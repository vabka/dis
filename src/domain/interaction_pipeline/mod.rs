mod error;


use futures_util::future::LocalBoxFuture;
use std::future::Future;

pub use error::InteractionError;
use crate::discord::interaction::{Interaction, InteractionCallback};

pub type InteractionHandlerResult = Option<Result<InteractionCallback, InteractionError>>;

pub trait InteractionHandler<Context> {
    type Future: Future<Output=InteractionHandlerResult>;
    fn handle(&self, interaction: &Interaction, context: &Context) -> Self::Future;
}
pub type Task<T> = LocalBoxFuture<'static, T>;

pub trait NoContextInteractionHandler {
    type Future: Future<Output=InteractionHandlerResult>;
    fn handle(&self, interaction: &Interaction) -> Self::Future;
}

impl<T, C> InteractionHandler<C> for T where T: NoContextInteractionHandler {
    type Future = <Self as NoContextInteractionHandler>::Future;

    fn handle(&self, interaction: &Interaction, _: &C) -> Self::Future {
        self.handle(interaction)
    }
}


impl<TContext> InteractionPipeline<TContext> {
    pub fn new(
        handlers: Vec<
            Box<
                dyn InteractionHandler<TContext, Future=Task<InteractionHandlerResult>>,
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

pub struct InteractionPipeline<TContext> {
    handlers: Vec<
        Box<dyn InteractionHandler<TContext, Future=Task<InteractionHandlerResult>>>,
    >,
}
