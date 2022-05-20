use std::future::{Future, ready};
use futures_util::future::BoxFuture;
use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::application_command::ApplicationCommandType;
use crate::discord::interactions::{ApplicationCommandInteractionDataOption, Interaction, InteractionCallback, InteractionCallbackMessage, InteractionType};
use crate::discord::interactions::InteractionType::ApplicationCommand;
use crate::{DiscordBotApiClient, Storage};
use crate::endpoints::post_interactions::InteractionError;

pub type InteractionHandlerResult = Option<Result<InteractionCallback, InteractionError>>;

pub trait InteractionHandler {
    type Future: Future<Output=InteractionHandlerResult>;
    type Context;
    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future;
}

pub type Task<T> = BoxFuture<'static, T>;

#[derive(Clone)]
pub struct BotContext {
    store: Storage,
    api_client: DiscordBotApiClient,
}

impl BotContext {
    pub fn new(store: Storage, api_client: DiscordBotApiClient) -> Self {
        Self { store, api_client }
    }
}

pub struct InteractionPipeline<TContext> {
    handlers: Vec<Box<dyn InteractionHandler<Future=Task<InteractionHandlerResult>, Context=TContext>>>,
}

impl<TContext> InteractionPipeline<TContext> {
    pub fn new(handlers: Vec<Box<dyn InteractionHandler<Future=Task<InteractionHandlerResult>, Context=TContext>>>) -> Self {
        Self {
            handlers
        }
    }
    pub async fn handle(&self, interaction: Interaction, context: &TContext) -> Result<InteractionCallback, InteractionError> {
        for handler in &self.handlers {
            if let Some(result) = handler.handle(&interaction, context).await {
                return result;
            }
        }
        Err(InteractionError::NoHandlerFound)
    }
}

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

pub struct EchoCommandHandler;

impl InteractionHandler for EchoCommandHandler {
    type Future = Task<InteractionHandlerResult>;
    type Context = BotContext;

    fn handle(&self, interaction: &Interaction, _: &Self::Context) -> Self::Future {
        Box::pin(ready(Some(interaction)
            .filter(|i| i.interaction_type == ApplicationCommand)
            .and_then(|i|
                i.data.as_ref()
                    .filter(|d| d.name == "echo")
                    .and_then(|d| d.options.as_ref())
                    .map(|o| match o.as_ref() {
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