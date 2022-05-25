use crate::domain::interaction_pipeline::{InteractionError, InteractionHandler, InteractionHandlerResult, NoContextInteractionHandler, Task};
use futures_util::FutureExt;
use std::future::ready;
use crate::discord::interaction::{Interaction, InteractionCallback, InteractionData, InteractionType};

pub type CommandHandlerResult = Result<InteractionCallback, InteractionError>;

pub trait CommandHandler<Context> {
    type Args;
    type Future;

    fn name() -> &'static str;
    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args>;
    fn handle(&self, args: Self::Args, context: &Context) -> Self::Future;
}

pub trait NoContextCommandHandler {
    type Args;
    type Future;

    fn name() -> &'static str;
    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args>;
    fn handle(&self, args: Self::Args) -> Self::Future;
}

impl<T: NoContextCommandHandler, C> CommandHandler<C> for T {
    type Args = <Self as NoContextCommandHandler>::Args;
    type Future = <Self as NoContextCommandHandler>::Future;

    fn name() -> &'static str {
        Self::name()
    }
    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args> {
        Self::parse_args(interaction_data)
    }
    fn handle(&self, args: Self::Args, _: &C) -> Self::Future {
        self.handle(args)
    }
}