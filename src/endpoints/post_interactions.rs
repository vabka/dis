use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::application_command::ApplicationCommandType;
use crate::discord::interactions::{
    ApplicationCommandInteractionDataOption, Interaction, InteractionCallback,
    InteractionCallbackMessage, InteractionData,
};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{post, web};
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use log::{debug, error};

use tokio::sync::RwLock;
use crate::domain::store::*;
use crate::Storage;
use crate::endpoints::interaction_pipeline::InteractionPipeline;

#[post("/interactions")]
pub async fn interactions(
    interaction: Json<Interaction>,
    pipeline: web::Data<InteractionPipeline>,
) -> Result<Json<InteractionCallback>, InteractionError> {
    pipeline.handle(interaction.into_inner()).await.map(Json)
}


async fn dispatch(data: &InteractionData, store: web::Data<RwLock<Storage>>) -> Result<InteractionCallback, InteractionError> {
    match data.name.as_str() {
        "echo" => data
            .options
            .as_ref()
            .and_then(|x| match x.as_ref() {
                [ApplicationCommandInteractionDataOption {
                    name: n,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(s),
                }] if n == "text" => {
                    let msg = InteractionCallbackMessage {
                        content: Some(s.to_owned()),
                    };
                    Some(InteractionCallback::channel_message_with_source(msg))
                }
                _ => None,
            })
            .ok_or(InteractionError::InvalidCommand),
        "set" => {
            let (key, value) = data.options
                .as_ref()
                .and_then(|x| match x.as_ref() {
                    [
                    ApplicationCommandInteractionDataOption {
                        name: k,
                        application_command_option_type: ApplicationCommandType::String,
                        value: Str(key),
                    },
                    ApplicationCommandInteractionDataOption {
                        name: v,
                        application_command_option_type: ApplicationCommandType::String,
                        value: Str(value),
                    }
                    ] if k == "key" && v == "value" => {
                        Some((key, value))
                    }
                    _ => None,
                })
                .ok_or(InteractionError::InvalidCommand)?;
            let mut write_store = store.write().await;
            write_store.upsert(key, value).await?;
            Ok(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                content: Some("Data saved".to_string())
            }))
        }
        "get" => {
            debug!("Trying read saved note");
            let key = data.options
                .as_ref()
                .and_then(|x| match x.as_ref() {
                    [
                    ApplicationCommandInteractionDataOption {
                        name: k,
                        application_command_option_type: ApplicationCommandType::String,
                        value: Str(key),
                    }
                    ] if k == "key" => {
                        Some(key)
                    }
                    _ => None,
                })
                .ok_or(InteractionError::InvalidCommand)?;
            debug!("Interaction parsed");
            let read_store = store.read().await;
            debug!("Trying to read data");
            let value = read_store.read(key).await?;
            Ok(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                content: Some(value)
            }))
        }
        _ => Err(InteractionError::UnknownCommand),
    }
}

#[derive(Debug, err_derive::Error)]
pub enum InteractionError {
    #[error(display = "No suitable handler found")]
    NoHandlerFound,
    #[error(display = "Unexpected error occurred")]
    Unexpected,
    #[error(display = "Command not implemented")]
    CommandNotImplemented,
    #[error(display = "Unknown command")]
    UnknownCommand,
    #[error(display = "Invalid command parameters")]
    InvalidCommand,
    #[error(display = "Key not found")]
    KeyNotFound,
}

impl From<UpsertError> for InteractionError {
    fn from(_: UpsertError) -> Self {
        InteractionError::Unexpected
    }
}

impl From<ReadError> for InteractionError {
    fn from(e: ReadError) -> Self {
        match e {
            ReadError::MissingKey => InteractionError::KeyNotFound,
            ReadError::NoData => {
                debug!("No data for key");
                InteractionError::Unexpected
            }
            ReadError::Kv(e) => {
                debug!("Error in kv: {:#?}", e);
                InteractionError::Unexpected
            }
        }
    }
}

impl ResponseError for InteractionError {
    fn status_code(&self) -> StatusCode {
        match self {
            InteractionError::Unexpected | InteractionError::NoHandlerFound => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());
        match self {
            InteractionError::Unexpected | InteractionError::NoHandlerFound => {
                error!("Error occured: {}", self);
                response.json(())
            }
            InteractionError::CommandNotImplemented => {
                response.json(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(String::from("***This command is not implemented***")),
                }))
            }
            InteractionError::UnknownCommand => {
                response.json(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(String::from("***This command is unknown***")),
                }))
            }
            InteractionError::InvalidCommand => {
                response.json(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(String::from("***This command is invalid***")),
                }))
            }
            e => {
                response.json(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(format!("***{}***", e.to_string())),
                }))
            }
        }
    }
}
