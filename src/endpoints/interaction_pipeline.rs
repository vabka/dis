use std::future::{Future, ready};
use futures_util::future::BoxFuture;
use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::application_command::ApplicationCommandType;
use crate::discord::interactions::{ApplicationCommandInteractionDataOption, Interaction, InteractionCallback, InteractionCallbackMessage, InteractionType};
use crate::discord::interactions::InteractionType::ApplicationCommand;
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

pub struct EchoCommandHandler;

impl InteractionHandler for EchoCommandHandler {
    type Future = Task<InteractionHandlerResult>;

    fn handle(&self, interaction: &Interaction) -> Self::Future {
        Box::pin(ready(Some(interaction)
            .filter(|i| i.interaction_type == ApplicationCommand)
            .and_then(|i|
                i.data.as_ref()
                    .filter(|data| data.name == "echo")
                    .and_then(|data| data.options.as_ref())
                    .map(|options| match options.as_ref() {
                        [ApplicationCommandInteractionDataOption {
                            name: n,
                            application_command_option_type: ApplicationCommandType::String,
                            value: Str(text)
                        }] if n == "text" => {
                            let msg = InteractionCallbackMessage {
                                content: Some(text.to_string())
                            };
                            let callback = InteractionCallback::channel_message_with_source(msg);
                            Ok(callback)
                        }
                        _ => Err(InteractionError::InvalidCommand)
                    }))
        ))
    }
}