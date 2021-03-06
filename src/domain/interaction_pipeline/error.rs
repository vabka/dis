use crate::domain::store::{ListError, ReadError, UpsertError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::{debug, error};
use crate::discord::interaction::{InteractionCallback, InteractionCallbackMessage};

#[derive(Debug, thiserror::Error)]
pub enum InteractionError {
    #[error("No suitable handler found")]
    NoHandlerFound,
    #[error("Unexpected error occurred")]
    Unexpected,
    #[error("Command not implemented")]
    CommandNotImplemented,
    #[error("Unknown command")]
    UnknownCommand,
    #[error("Invalid command parameters")]
    InvalidCommand,
    #[error("Key not found")]
    KeyNotFound,
}

impl From<UpsertError> for InteractionError {
    fn from(_: UpsertError) -> Self {
        InteractionError::Unexpected
    }
}

impl From<ListError> for InteractionError {
    fn from(_: ListError) -> Self {
        InteractionError::Unexpected
    }
}

impl From<ReadError> for InteractionError {
    fn from(e: ReadError) -> Self {
        match e {
            ReadError::MissingKey | ReadError::NoData => InteractionError::KeyNotFound,
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
            InteractionError::Unexpected | InteractionError::NoHandlerFound => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => StatusCode::OK,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut response = HttpResponse::build(self.status_code());
        match self {
            InteractionError::Unexpected | InteractionError::NoHandlerFound => {
                error!("Error occured: {}", self);
                response.json(())
            }
            InteractionError::CommandNotImplemented => response.json(
                InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(String::from("***This command is not implemented***")),
                }),
            ),
            InteractionError::UnknownCommand => response.json(
                InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(String::from("***This command is unknown***")),
                }),
            ),
            InteractionError::InvalidCommand => response.json(
                InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                    content: Some(String::from("***This command is invalid***")),
                }),
            ),
            e => response.json(InteractionCallback::channel_message_with_source(
                InteractionCallbackMessage {
                    content: Some(format!("***{}***", e)),
                },
            )),
        }
    }
}
