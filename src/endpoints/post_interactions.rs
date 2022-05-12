use std::fmt::{Display, Formatter};
use actix_web::{HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, StatusCode};
use actix_web::web::Json;
use actix_web::post;
use log::info;
use crate::discord::application_command::ApplicationCommandOptionValue;
use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::interactions::{Interaction, InteractionCallback, InteractionCallbackMessage, InteractionType};

#[post("/interactions")]
pub async fn interactions(
    interaction: Json<Interaction>,
) -> actix_web::Result<impl Responder> {
    info!("Interaction received! {:#?}", interaction);
    return match interaction.interaction_type {
        InteractionType::Ping => Ok(Json(InteractionCallback::pong())),
        InteractionType::ApplicationCommand => {
            (&interaction.data)
                .as_ref()
                .filter(|d| d.name == "echo")
                .and_then(|p| {
                    if let Some(s) = &p.options {
                        if let Str(s) = &s[0].value {
                            return Some(InteractionCallback::channel_message_with_source(InteractionCallbackMessage {
                                content: Some(s.to_owned())
                            }));
                        }
                    }
                    return None;
                })
                .map(|c| Json(c))
                .ok_or_else(|| InteractionError::Unknown.into())
        }
        _ => todo!("Not covered")
    };
}

#[derive(Debug)]
enum InteractionError {
    Unknown
}

impl Display for InteractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl ResponseError for InteractionError {
    fn status_code(&self) -> StatusCode {
        match self {
            InteractionError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());
        res.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        res.set_body(BoxBody::new("{}"))
    }
}

