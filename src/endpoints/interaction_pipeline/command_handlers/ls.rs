use crate::BotContext;
use crate::discord::interactions::InteractionCallback;
use crate::discord::interactions::InteractionCallbackMessage;
use crate::discord::interactions::InteractionData;
use crate::endpoints::interaction_pipeline::{Task};
use crate::endpoints::interaction_pipeline::command_handlers::{CommandHandler, CommandHandlerResult};

pub struct LsCommandHandler;

impl CommandHandler for LsCommandHandler {
    type Args = ();
    type Context = BotContext;
    type Future = Task<CommandHandlerResult>;

    fn name() -> &'static str {
        "ls"
    }

    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args> {
        match interaction_data.options.as_deref() {
            Some([]) | None => Some(()),
            _ => None
        }
    }

    fn handle(&self, _: Self::Args, context: &Self::Context) -> Self::Future {
        let store = context.store.clone();
        Box::pin(async move {
            let entries = store.list().await?;
            let message_text = entries
                .iter()
                .map(|s| format!("| {s}"))
                .collect::<Vec<String>>()
                .join("\n");
            let message = InteractionCallbackMessage {
                content: Some(message_text),
            };
            let callback =
                InteractionCallback::channel_message_with_source(message);
            Ok(callback)
        })
    }
}