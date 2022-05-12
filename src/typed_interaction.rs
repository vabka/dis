use crate::Snowflake;

#[derive(Deserialize, Debug)]
pub struct Interaction {
    pub application_id: Snowflake,
    pub id: Snowflake,
    pub token: String,
    pub version: u8,

    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub data: Option<InteractionData>,

    pub user: Option<User>,

    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub message: Option<Message>,
    pub locale: Option<Locale>,
    pub guild_locale: Option<Locale>,
}
enum Sender {
    Member(GuildMember),
    User(User)
}

enum InteractionData {
    Ping,
    ApplicationCommand {data: ApplicationCommandData, locale: Locale},
    MessageComponent {data: MessageComponentDat, message: Message, locale: Locale},
    ApplicationCommandAutocomplete,
    ModalSubmit {data: ModalSubmitData, locale: Locale}
}

struct ApplicationCommandData {
    pub id: Snowflake
}