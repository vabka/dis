use crate::domain::interaction_pipeline::{InteractionHandler, InteractionHandlerResult, Task};
use crate::BotContext;
use std::future::ready;
use crate::discord::interaction::{Interaction, InteractionCallback, InteractionType};
use crate::domain::bot::BotContext;

pub struct PingInteractionHandler;

impl InteractionHandler for PingInteractionHandler {
    type Future = Task<InteractionHandlerResult>;
    type Context = BotContext;

    fn handle(&self, interaction: &Interaction, _: &Self::Context) -> Self::Future {
        if interaction.interaction_type == InteractionType::Ping {
            Box::pin(ready(Some(Ok(InteractionCallback::pong()))))
        } else {
            Box::pin(ready(None))
        }
    }
}
