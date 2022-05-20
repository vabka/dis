use crate::discord::interactions::{
    Interaction, InteractionCallback, InteractionData, InteractionType,
};
use crate::endpoints::interaction_pipeline::{
    InteractionError, InteractionHandler, InteractionHandlerResult, Task,
};
use futures_util::FutureExt;
use std::future::ready;

mod echo;
mod get;
mod ls;
mod set;

pub use echo::EchoCommandHandler;
pub use get::GetCommandHandler;
pub use ls::LsCommandHandler;
pub use set::SetCommandHandler;

type CommandHandlerResult = Result<InteractionCallback, InteractionError>;
trait CommandHandler {
    type Args;
    type Context;
    type Future;

    fn name() -> &'static str;
    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args>;
    fn handle(&self, args: Self::Args, context: &Self::Context) -> Self::Future;
}

impl<T: CommandHandler<Future = Task<CommandHandlerResult>, Context = C>, C> InteractionHandler
    for T
{
    type Future = Task<InteractionHandlerResult>;
    type Context = C;

    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future {
        let args = Some(interaction)
            .filter(|i| i.interaction_type == InteractionType::ApplicationCommand)
            .and_then(|i| {
                Some(
                    i.data
                        .as_ref()
                        .filter(|d| d.name == <Self as CommandHandler>::name())
                        .and_then(<Self as CommandHandler>::parse_args)
                        .ok_or(InteractionError::InvalidCommand),
                )
            });
        match args {
            Some(Ok(args)) => Box::pin(self.handle(args, context).map(Some)),
            Some(Err(e)) => Box::pin(ready(Some(Err(e)))),
            None => Box::pin(ready(None)),
        }
    }
}
