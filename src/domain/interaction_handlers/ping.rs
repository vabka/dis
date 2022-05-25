use crate::domain::interaction_pipeline::{NoContextInteractionHandler, InteractionHandlerResult, Task};
use std::future::ready;
use crate::discord::interaction::{Interaction, InteractionCallback, InteractionType};

pub struct PingInteractionHandler;

impl NoContextInteractionHandler for PingInteractionHandler {
    type Future = Task<InteractionHandlerResult>;

    fn handle(&self, interaction: &Interaction) -> Self::Future {
        if interaction.interaction_type == InteractionType::Ping {
            Box::pin(ready(Some(Ok(InteractionCallback::pong()))))
        } else {
            Box::pin(ready(None))
        }
    }
}
