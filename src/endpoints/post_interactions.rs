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
use actix_web::{web, HttpResponse, ResponseError};
use log::info;
use std::fmt::{Display, Formatter};

#[post("/interactions")]
pub async fn interactions(
    interaction: Json<Interaction>,
) -> Result<Json<InteractionCallback>, InteractionError> {
    info!("Interaction received! {:#?}", interaction);
    return match interaction.interaction_type {
        InteractionType::Ping => Ok(Json(InteractionCallback::pong())),
        InteractionType::ApplicationCommand => (&interaction.data)
            .as_ref()
            .ok_or(InteractionError::Unexpected)
            .and_then(dispatch)
            .map(Json),
        _ => Err(InteractionError::Unexpected),
    };
}

fn dispatch(data: &InteractionData) -> Result<InteractionCallback, InteractionError> {
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
        "set" => Err(InteractionError::CommandNotImplemented),
        "get" => Err(InteractionError::CommandNotImplemented),
        _ => Err(InteractionError::UnknownCommand),
    }
}

#[derive(Debug, err_derive::Error)]
pub enum InteractionError {
    #[error(display = "Unexpected error occurred")]
    Unexpected,
    #[error(display = "Command not implemented")]
    CommandNotImplemented,
    #[error(display = "Unexpected error occurred")]
    UnknownCommand,
    #[error(display = "Unexpected error occurred")]
    InvalidCommand,
}

impl ResponseError for InteractionError {
    fn status_code(&self) -> StatusCode {
        match self {
            InteractionError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
            InteractionError::CommandNotImplemented => StatusCode::OK,
            InteractionError::UnknownCommand => StatusCode::OK,
            InteractionError::InvalidCommand => StatusCode::OK,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .json(match self {
                InteractionError::Unexpected => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("Unexpected error occurred. Try again later")),
                    })
                }
                InteractionError::CommandNotImplemented => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("This command is not implemented")),
                    })
                }
                InteractionError::UnknownCommand => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("This command is unknown")),
                    })
                }
                InteractionError::InvalidCommand => {
                    InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                        content: Some(String::from("This command is invalid")),
                    })
                }
            })
    }
}
