use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use crate::discord::interaction::{Interaction, InteractionCallback};
use crate::domain::bot::BotContext;
use crate::domain::interaction_pipeline::{InteractionError, InteractionPipeline};
use actix_rt::task::spawn_blocking;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};

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

#[get("/index")]
pub async fn index() -> impl Responder {
    let path = Path::new("test.txt");
    spawn_blocking(|| cat(path)).await?
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
