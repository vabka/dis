use crate::discord::application_command::{
    ApplicationCommand, ApplicationCommandOption, ApplicationCommandOptionBuilder,
};
use crate::discord::DiscordBotApiClient;

pub async fn declare_commands(client: &DiscordBotApiClient) -> anyhow::Result<()> {
    let set = ApplicationCommand::build_for_application("set", client.app_id())
        .with_description("Save new note")
        .with_option(
            ApplicationCommandOption::build_string_option("key")
                .required()
                .with_description("Key of note")
                .finish(),
        )
        .with_option(
            ApplicationCommandOption::build_string_option("value")
                .required()
                .with_description("Text of note")
                .finish(),
        )
        // TODO autocomplete
        .finish();

    let get = ApplicationCommand::build_for_application("get", client.app_id())
        .with_description("Read saved note")
        .with_option(
            ApplicationCommandOption::build_string_option("key")
                .required()
                .with_description("Key of note")
                .finish(),
        )
        // TODO autocomplete
        .finish();

    let echo = ApplicationCommand::build_for_application("echo", client.app_id())
        .with_description("Reply with same text")
        .with_option(
            ApplicationCommandOption::build_string_option("text")
                .required()
                .with_description("Text to reply")
                .finish(),
        )
        .finish();

    let ls = ApplicationCommand::build_for_application("ls", client.app_id())
        .with_description("List all available notes")
        .finish();

    client.create_application_command(&set).await?;
    client.create_application_command(&get).await?;
    client.create_application_command(&echo).await?;
    client.create_application_command(&ls).await?;
    Ok(())
}
