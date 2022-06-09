use crate::discord::rest::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::rest::application_command::ApplicationCommandType;

use crate::domain::command_handlers::{
    CommandHandler, CommandHandlerResult,
};
use crate::domain::interaction_pipeline::Task;
use crate::discord::interaction::{ApplicationCommandInteractionDataOption, InteractionCallback, InteractionCallbackMessage, InteractionData};
use crate::domain::bot::{Get};
use crate::domain::store::Storage;

pub struct SetCommandHandler;

pub struct SetCommandArgs {
    key: String,
    value: String,
}

impl<C: Get<Storage>> CommandHandler<C> for SetCommandHandler {
    type Args = SetCommandArgs;
    type Future = Task<CommandHandlerResult>;

    fn name() -> &'static str {
        "set"
    }

    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args> {
        interaction_data.options.as_ref()
            .and_then(|o| match o.as_ref() {
                [ApplicationCommandInteractionDataOption {
                    name: key_name,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(key),
                }, ApplicationCommandInteractionDataOption {
                    name: value_name,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(value),
                }] if key_name == "key" && value_name == "value" => Some(SetCommandArgs {
                    key: key.to_string(),
                    value: value.to_string(),
                }),
                _ => None,
            })
    }

    fn handle(&self, args: Self::Args, context: &C) -> Self::Future {
        let store: Storage = context.get().clone();
        Box::pin(async move {
            let Self::Args { key, value } = args;
            store.upsert(&key, &value).await?;
            let message = InteractionCallbackMessage {
                content: Some(String::from("Successfully set value for note!")),
            };
            let callback = InteractionCallback::channel_message_with_source(message);
            Ok(callback)
        })
    }
}
