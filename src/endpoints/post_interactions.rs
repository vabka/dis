use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::application_command::ApplicationCommandType;
use crate::discord::interactions::{
    ApplicationCommandInteractionDataOption, Interaction, InteractionCallback,
    InteractionCallbackMessage, InteractionData, InteractionType,
};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use log::info;
use tokio::sync::RwLock;
use futures_util::future::TryFutureExt;
use crate::domain::store::*;
use crate::Storage;

#[post("/interactions")]
pub async fn interactions(
    interaction: Json<Interaction>, store: actix_web::web::Data<RwLock<Storage>>,
) -> Result<Json<InteractionCallback>, InteractionError> {
    info!("Interaction received! {:#?}", interaction);
    return match interaction.interaction_type {
        InteractionType::Ping => Ok(Json(InteractionCallback::pong())),
        InteractionType::ApplicationCommand => {
            let fut = std::future::ready((&interaction.data)
                .as_ref()
                .ok_or(InteractionError::Unexpected))
                .and_then(|x| { dispatch(x, store) });
            let res = fut.await?;
            Ok(Json(res))
        }
        _ => Err(InteractionError::Unexpected),
    };
}

async fn dispatch(data: &InteractionData, store: actix_web::web::Data<RwLock<Storage>>) -> Result<InteractionCallback, InteractionError> {
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
            let read_store = store.read().await;
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
    #[error(display = "Unexpected error occurred")]
    Unexpected,
    #[error(display = "Command not implemented")]
    CommandNotImplemented,
    #[error(display = "Unknown command")]
    UnknownCommand,
    #[error(display = "Invalid command parameters")]
    InvalidCommand,
    #[error(display = "Key not found for /get command")]
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
            ReadError::NoData => InteractionError::Unexpected,
            ReadError::Kv(_) => InteractionError::Unexpected
        }
    }
}

impl ResponseError for InteractionError {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .json(match self {
                InteractionError::Unexpected => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("***Unexpected error occurred. Try again later***")),
                    })
                }
                InteractionError::CommandNotImplemented => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("***This command is not implemented***")),
                    })
                }
                InteractionError::UnknownCommand => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("***This command is unknown***")),
                    })
                }
                InteractionError::InvalidCommand => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("***This command is invalid***")),
                    })
                }
                e => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(format!("***{}***", e.to_string())),
                    })
                }
            })
    }
}
