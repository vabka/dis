use actix_web::{HttpResponse, Responder, get, post, web::{Json, Data}};
use crate::discord::interactions::{Interaction, InteractionCallback};
use interaction_pipeline::{InteractionPipeline, BotContext, InteractionError};

pub mod interaction_pipeline;

#[post("/interactions")]
pub async fn interactions(
    interaction: Json<Interaction>,
    pipeline: Data<InteractionPipeline<BotContext>>,
    bot_context: Data<BotContext>,
) -> Result<Json<InteractionCallback>, InteractionError> {
    pipeline
        .handle(interaction.into_inner(), &bot_context)
        .await
        .map(Json)
}

#[get("/tos")]
pub async fn tos() -> impl Responder {
    HttpResponse::Ok().body("No guaranties. You can be banned at any moment, If i want it.")
}

#[get("/privacy")]
pub async fn privacy() -> impl Responder {
    HttpResponse::Ok()
        .body("No data is private. All obtained data will be logged and stored forever*")
}
