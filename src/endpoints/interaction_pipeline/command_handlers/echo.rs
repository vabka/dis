use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::application_command::ApplicationCommandType;
use crate::discord::interactions::ApplicationCommandInteractionDataOption;
use crate::discord::interactions::InteractionCallback;
use crate::discord::interactions::InteractionCallbackMessage;
use crate::discord::interactions::InteractionData;
use crate::endpoints::interaction_pipeline::command_handlers::{
    CommandHandler, CommandHandlerResult,
};
use crate::endpoints::interaction_pipeline::Task;
use crate::BotContext;
use std::future::ready;

pub struct EchoCommandHandler;

impl CommandHandler for EchoCommandHandler {
    type Args = String;
    type Context = BotContext;
    type Future = Task<CommandHandlerResult>;

    fn name() -> &'static str {
        "echo"
    }
    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args> {
        interaction_data
            .options
            .as_ref()
            .and_then(|o| match o.as_ref() {
                [ApplicationCommandInteractionDataOption {
                    name: n,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(text),
                }] if n == "text" => Some(text.to_string()),
                _ => None,
            })
    }
    fn handle(&self, args: Self::Args, _: &Self::Context) -> Self::Future {
        let msg = InteractionCallbackMessage {
            content: Some(args),
        };
        let callback = InteractionCallback::channel_message_with_source(msg);
        Box::pin(ready(Ok(callback)))
    }
}
