use std::future::{Future, ready};
use futures_util::FutureExt;
use crate::discord::interaction::{Interaction, InteractionType};
use crate::domain::command_handlers::{CommandHandler, CommandHandlerResult};
use crate::domain::interaction_pipeline::{InteractionError, InteractionHandler, InteractionHandlerResult, Task};

pub struct InteractionCommandInteractionHandler<T>(T);

impl<CH, C, F> InteractionHandler<C> for InteractionCommandInteractionHandler<CH>
    where CH: CommandHandler<C, Future=F>,
          F: Future<Output=CommandHandlerResult> + 'static
{
    type Future = Task<InteractionHandlerResult>;

    fn handle(&self, interaction: &Interaction, context: &C) -> Self::Future {
        let args: Option<Result<<CH as CommandHandler<C>>::Args, InteractionError>> =
            Some(interaction)
                .filter(|i| i.interaction_type == InteractionType::ApplicationCommand)
                .and_then(|i| i.data.as_ref())
                .filter(|d| d.name == <CH as CommandHandler<C>>::name())
                .map(|i| <CH as CommandHandler<C>>::parse_args(i).ok_or(InteractionError::InvalidCommand));
        match args {
            Some(Ok(args)) => Box::pin(self.0.handle(args, context).map(Some)),
            Some(Err(e)) => Box::pin(ready(Some(Err(e)))),
            None => Box::pin(ready(None)),
        }
    }
}

impl<T> From<T> for InteractionCommandInteractionHandler<T> {
    fn from(e: T) -> Self {
        InteractionCommandInteractionHandler(e)
    }
}