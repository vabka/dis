use std::future::ready;
use crate::BotContext;
use crate::discord::interactions::{Interaction, InteractionType};
use crate::endpoints::interaction_pipeline::{InteractionHandler, InteractionHandlerResult, Task};

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