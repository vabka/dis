use crate::discord::rest::application_command::ApplicationCommandOptionValue::Str;
use crate::discord::rest::application_command::ApplicationCommandType;
use crate::domain::command_handlers::{
    CommandHandler, CommandHandlerResult,
};
use crate::domain::interaction_pipeline::Task;
use crate::BotContext;
use std::future::ready;
use crate::discord::interaction::{ApplicationCommandInteractionDataOption, InteractionCallback, InteractionCallbackMessage, InteractionData};
use crate::domain::bot::BotContext;

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

#[cfg(test)]
mod tests {
    use crate::discord::interaction::{ApplicationCommandInteractionDataOption, InteractionData};
    use crate::discord::rest::application_command::{ApplicationCommandOptionValue, ApplicationCommandType};
    use crate::EchoCommandHandler;
    use crate::domain::command_handlers::CommandHandler;

    #[test]
    fn echo_name() {
        assert_eq!(EchoCommandHandler::name(), "echo")
    }

    fn create_default_interaction_data_with_options(options: Vec<ApplicationCommandInteractionDataOption>) -> InteractionData {
        InteractionData {
            options: Some(options.into_boxed_slice()),

            command_type: ApplicationCommandType::SubCommand,
            id: Default::default(),
            name: Default::default(),
            resolved: Default::default(),
            guild_id: Default::default(),
            custom_id: Default::default(),
            values: Default::default(),
            target_id: Default::default(),
            components: Default::default(),
        }
    }

    #[test]
    fn echo_parse_args() {
        let options = vec![ApplicationCommandInteractionDataOption {
            name: String::from("text"),
            application_command_option_type: ApplicationCommandType::String,
            value: ApplicationCommandOptionValue::Str(String::from("test")),
        }];

        let interaction_data = create_default_interaction_data_with_options(options);

        let args = EchoCommandHandler::parse_args(&interaction_data);
        assert_eq!(args, Some(String::from("test")));
    }

    #[test]
    fn echo_fail_parse_args_when_option_name_is_invalid() {
        let options = vec![ApplicationCommandInteractionDataOption {
            name: String::from("some wrong parameter name"),
            application_command_option_type: ApplicationCommandType::String,
            value: ApplicationCommandOptionValue::Str(String::from("test")),
        }];

        let interaction_data = create_default_interaction_data_with_options(options);

        let args = EchoCommandHandler::parse_args(&interaction_data);
        assert_eq!(args, None);
    }

    #[test]
    fn echo_fail_parse_args_when_no_options() {
        let options = vec![];
        let interaction_data = create_default_interaction_data_with_options(options);

        let args = EchoCommandHandler::parse_args(&interaction_data);
        assert_eq!(args, None);
    }

    #[test]
    fn echo_fail_parse_args_when_more_than_one_option() {
        let options = vec![ApplicationCommandInteractionDataOption {
            name: String::from("text"),
            application_command_option_type: ApplicationCommandType::String,
            value: ApplicationCommandOptionValue::Str(String::from("test")),
        }, ApplicationCommandInteractionDataOption {
            name: String::from("text"),
            application_command_option_type: ApplicationCommandType::String,
            value: ApplicationCommandOptionValue::Str(String::from("test")),
        }];

        let interaction_data = create_default_interaction_data_with_options(options);

        let args = EchoCommandHandler::parse_args(&interaction_data);
        assert_eq!(args, None);
    }
}
