use crate::discord::rest::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::rest::application_command::ApplicationCommandType;
use crate::domain::command_handlers::{
    CommandHandler, CommandHandlerResult,
};
use crate::domain::interaction_pipeline::Task;
use crate::domain::store::Storage;
use crate::discord::interaction::{ApplicationCommandInteractionDataOption, InteractionCallback, InteractionCallbackMessage, InteractionData};
use crate::domain::bot::{Get};

pub struct GetCommandHandler;

impl<C: Get<Storage>> CommandHandler<C> for GetCommandHandler {
    type Args = String;
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

    fn handle(&self, args: Self::Args, context: &C) -> Self::Future {
        let store: Storage = context.get().clone();
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
