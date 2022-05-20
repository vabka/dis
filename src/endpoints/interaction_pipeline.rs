use std::future::{Future, ready};
use futures_util::future::{BoxFuture, LocalBoxFuture};
use log::debug;
use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::application_command::ApplicationCommandType;
use crate::discord::interactions::{ApplicationCommandInteractionDataOption, Interaction, InteractionCallback, InteractionCallbackMessage, InteractionType};
use crate::discord::interactions::InteractionType::ApplicationCommand;
use crate::{DiscordBotApiClient, Storage};
use crate::domain::store::ListError;
use crate::endpoints::post_interactions::InteractionError;

pub type InteractionHandlerResult = Option<Result<InteractionCallback, InteractionError>>;

pub trait InteractionHandler {
    type Future: Future<Output=InteractionHandlerResult>;
    type Context;
    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future;
}

pub type Task<T> = LocalBoxFuture<'static, T>;

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

pub struct SetCommandHandler;

impl InteractionHandler for SetCommandHandler {
    type Future = Task<InteractionHandlerResult>;
    type Context = BotContext;
    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future {
        let args = Some(interaction)
            .filter(|i| i.interaction_type == ApplicationCommand)
            .and_then(|i|
                i.data.as_ref()
                    .filter(|d| d.name == "set")
                    .and_then(|d| d.options.as_ref())
                    .map(|o| match o.as_ref() {
                        [ApplicationCommandInteractionDataOption {
                            name: keyName,
                            application_command_option_type: ApplicationCommandType::String,
                            value: Str(key)
                        },
                        ApplicationCommandInteractionDataOption {
                            name: valueName,
                            application_command_option_type: ApplicationCommandType::String,
                            value: Str(value)
                        }] if keyName == "key" && valueName == "value" => {
                            Ok((key.to_string(), value.to_string()))
                        }
                        _ => Err(InteractionError::InvalidCommand)
                    }));

        let store = context.store.clone();
        Box::pin(async move {
            match args {
                Some(Ok((key, value))) => {
                    let result = store.upsert(key.as_str(), value.as_str()).await;
                    match result {
                        Ok(_) => {
                            let message = InteractionCallbackMessage {
                                content: Some(String::from("Successfully set value for note!"))
                            };
                            let callback = InteractionCallback::channel_message_with_source(message);
                            Some(Ok(callback))
                        }
                        Err(e) => Some(Err(e.into()))
                    }
                }
                Some(Err(e)) => Some(Err(e)),
                None => None,
            }
        })
    }
}

pub struct LsCommandHandler;
impl InteractionHandler for LsCommandHandler {
    type Future = Task<InteractionHandlerResult>;
    type Context = BotContext;

    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future {
        let args = Some(interaction)
            .filter(|i| i.interaction_type == ApplicationCommand)
            .and_then(|i|
                i.data.as_ref()
                    .filter(|d| {
                        debug!("Command name: {}", d.name);
                        d.name == "ls"
                    })
                    .map(|d| match d.options.as_ref() {
                        None => {
                            Ok(())
                        }
                        _ => Err(InteractionError::InvalidCommand)
                    }));

        let store = context.store.clone();
        Box::pin(async move {
            match args {
                Some(Ok(_)) => {
                    let result = store.list().await;
                    match result {
                        Ok(entries) => {
                            let message_text = entries.iter()
                                .map(|s| { format!("| {s}") })
                                .collect::<Vec<String>>()
                                .join("\n");
                            let message = InteractionCallbackMessage {
                                content: Some(message_text)
                            };
                            let callback = InteractionCallback::channel_message_with_source(message);
                            Some(Ok(callback))
                        }
                        Err(e) => Some(Err(e.into()))
                    }
                }
                Some(Err(e)) => Some(Err(e)),
                None => None,
            }
        })
    }
}

pub struct GetCommandHandler;

impl InteractionHandler for GetCommandHandler {
    type Future = Task<InteractionHandlerResult>;
    type Context = BotContext;
    fn handle(&self, interaction: &Interaction, context: &Self::Context) -> Self::Future {
        let args = Some(interaction)
            .filter(|i| i.interaction_type == ApplicationCommand)
            .and_then(|i|
                i.data.as_ref()
                    .filter(|d| d.name == "get")
                    .and_then(|d| d.options.as_ref())
                    .map(|o| match o.as_ref() {
                        [ApplicationCommandInteractionDataOption {
                            name: keyName,
                            application_command_option_type: ApplicationCommandType::String,
                            value: Str(key)
                        }] if keyName == "key" => Ok(key.to_string()),
                        _ => Err(InteractionError::InvalidCommand)
                    }));

        let store = context.store.clone();
        Box::pin(async move {
            match args {
                Some(Ok(key)) => {
                    let result = store.read(key.as_str()).await;
                    match result {
                        Ok(value) => {
                            let message = InteractionCallbackMessage {
                                content: Some(format!("Your data: `{value}`"))
                            };
                            let callback = InteractionCallback::channel_message_with_source(message);
                            Some(Ok(callback))
                        }
                        Err(e) => Some(Err(e.into()))
                    }
                }
                Some(Err(e)) => Some(Err(e)),
                None => None,
            }
        })
    }
}


impl From<ListError> for InteractionError {
    fn from(_: ListError) -> Self {
        InteractionError::Unexpected
    }
}