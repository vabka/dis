use crate::discord::application_command::{ApplicationCommandBuilder, ApplicationCommandOptionBuilder};
use crate::discord::DiscordBotApiClient;

pub async fn define_commands(client: &DiscordBotApiClient) -> anyhow::Result<()> {
    let set = ApplicationCommandBuilder::for_application("set", client.app_id())
        .with_description("Save new note")
        .with_option(ApplicationCommandOptionBuilder::string_option("key")
            .required()
            .with_description("Key of note")
            .build())
        .with_option(ApplicationCommandOptionBuilder::string_option("value")
            .required()
            .with_description("Text of note")
            .build())
        .build();

    let get = ApplicationCommandBuilder::for_application("get", client.app_id())
        .with_description("Read saved note")
        .with_option(ApplicationCommandOptionBuilder::string_option("key")
            .required()
            .with_description("Key of note")
            .build())
        .build();

    let echo = ApplicationCommandBuilder::for_application("echo", client.app_id())
        .with_description("Reply with same text")
        .with_option(ApplicationCommandOptionBuilder::string_option("text")
            .required()
            .with_description("Text to reply")
            .build())
        .build();

    client.create_application_command(&set).await?;
    client.create_application_command(&get).await?;
    client.create_application_command(&echo).await?;
    Ok(())
}