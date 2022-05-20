use crate::BotContext;
use crate::discord::application_command::{ApplicationCommandType};
use crate::discord::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::interactions::ApplicationCommandInteractionDataOption;
use crate::discord::interactions::Interaction;
use crate::discord::interactions::InteractionCallback;
use crate::discord::interactions::InteractionCallbackMessage;
use crate::discord::interactions::InteractionData;
use crate::discord::interactions::InteractionType;
use crate::endpoints::interaction_pipeline::{Task};
use crate::endpoints::interaction_pipeline::command_handlers::{CommandHandler, CommandHandlerResult};

pub struct SetCommandHandler;

struct SetCommandArgs {
    key: String,
    value: String,
}

impl CommandHandler for SetCommandHandler {
    type Args = SetCommandArgs;
    type Context = BotContext;
    type Future = Task<CommandHandlerResult>;

    fn name() -> &'static str {
        "set"
    }

    fn parse_args(interaction_data: &InteractionData) -> Option<Self::Args> {
        interaction_data.options.and_then(|o|
            match o.as_ref() {
                [ApplicationCommandInteractionDataOption {
                    name: key_name,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(key),
                }, ApplicationCommandInteractionDataOption {
                    name: value_name,
                    application_command_option_type: ApplicationCommandType::String,
                    value: Str(value),
                }] if key_name == "key" && value_name == "value" => {
                    Some(SetCommandArgs { key: key.to_string(), value: value.to_string() })
                }
                _ => None
            })
    }

    fn handle(&self, args: Self::Args, context: &Self::Context) -> Self::Future {
        let store = context.store.clone();
        Box::pin(async move {
            let Self::Args { key, value } = args;
            store.upsert(key.as_str(), value.as_str()).await?;
            let message = InteractionCallbackMessage {
                content: Some(String::from("Successfully set value for note!")),
            };
            let callback =
                InteractionCallback::channel_message_with_source(message);
            Ok(callback)
        })
    }
}
