use crate::discord::rest::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::rest::application_command::ApplicationCommandType;
use crate::endpoints::interaction_pipeline::command_handlers::{
    CommandHandler, CommandHandlerResult,
};
use crate::endpoints::interaction_pipeline::Task;
use crate::BotContext;
use crate::discord::interaction::{ApplicationCommandInteractionDataOption, InteractionCallback, InteractionCallbackMessage, InteractionData};

pub struct GetCommandHandler;

impl CommandHandler for GetCommandHandler {
    type Args = String;
    type Context = BotContext;
    type Future = Task<CommandHandlerResult>;

    fn name() -> &'static str {
        "get"
    }

    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args> {
        interaction_data.options.as_ref()
            .and_then(|o| match o.as_ref() {
                [ApplicationCommandInteractionDataOption {
                    name: key_name,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(key),
                }] if key_name == "key" => Some(key.to_string()),
                _ => None,
            })
    }

    fn handle(&self, args: Self::Args, context: &Self::Context) -> Self::Future {
        let store = context.store.clone();
        Box::pin(async move {
            let value = store.read(args.as_str()).await?;
            let message = InteractionCallbackMessage {
                content: Some(format!("Your data: `{value}`")),
            };
            let callback = InteractionCallback::channel_message_with_source(message);
            Ok(callback)
        })
    }
}
