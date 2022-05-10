use serde::{Deserialize, Serialize};

type Inner = i64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions(#[serde(deserialize_with = "serde_aux::deserialize_number_from_string")] Inner);
pub struct PermissionsBuilder(Inner);

impl PermissionsBuilder {
    pub fn empty() -> Self {
        PermissionsBuilder(0.into())
    }

    pub fn allow_create_instant_invite(&mut self) {
        self.0 |= Inner::from(1 << 0);
    }
    pub fn allow_kick_members(&mut self) {
        self.0 |= Inner::from(1 << 1);
    }
    pub fn allow_ban_members(&mut self) {
        self.0 |= Inner::from(1 << 2);
    }
    pub fn allow_administrator(&mut self) {
        self.0 |= Inner::from(1 << 3);
    }
    pub fn allow_manage_channels(&mut self) {
        self.0 |= Inner::from(1 << 4);
    }
    pub fn allow_manage_guild(&mut self) {
        self.0 |= Inner::from(1 << 5);
    }
    pub fn allow_add_reactions(&mut self) {
        self.0 |= Inner::from(1 << 6);
    }
    pub fn allow_view_audit_log(&mut self) {
        self.0 |= Inner::from(1 << 7);
    }
    pub fn allow_priority_speaker(&mut self) {
        self.0 |= Inner::from(1 << 8);
    }
    pub fn allow_stream(&mut self) {
        self.0 |= Inner::from(1 << 9);
    }
    pub fn allow_view_channel(&mut self) {
        self.0 |= Inner::from(1 << 10);
    }
    pub fn allow_send_messages(&mut self) {
        self.0 |= Inner::from(1 << 11);
    }
    pub fn allow_send_tts_messages(&mut self) {
        self.0 |= Inner::from(1 << 12);
    }
    pub fn allow_manage_messages(&mut self) {
        self.0 |= Inner::from(1 << 13);
    }
    pub fn allow_embed_links(&mut self) {
        self.0 |= Inner::from(1 << 14);
    }
    pub fn allow_attach_files(&mut self) {
        self.0 |= Inner::from(1 << 15);
    }
    pub fn allow_read_message_history(&mut self) {
        self.0 |= Inner::from(1 << 16);
    }
    pub fn allow_mention_everyone(&mut self) {
        self.0 |= Inner::from(1 << 17);
    }
    pub fn allow_use_external_emojis(&mut self) {
        self.0 |= Inner::from(1 << 18);
    }
    pub fn allow_view_guild_insights(&mut self) {
        self.0 |= Inner::from(1 << 19);
    }
    pub fn allow_connect(&mut self) {
        self.0 |= Inner::from(1 << 20);
    }
    pub fn allow_speak(&mut self) {
        self.0 |= Inner::from(1 << 21);
    }
    pub fn allow_mute_members(&mut self) {
        self.0 |= Inner::from(1 << 22);
    }
    pub fn allow_deafen_members(&mut self) {
        self.0 |= Inner::from(1 << 23);
    }
    pub fn allow_move_members(&mut self) {
        self.0 |= Inner::from(1 << 24);
    }
    pub fn allow_use_vad(&mut self) {
        self.0 |= Inner::from(1 << 25);
    }
    pub fn allow_change_nickname(&mut self) {
        self.0 |= Inner::from(1 << 26);
    }
    pub fn allow_manage_nicknames(&mut self) {
        self.0 |= Inner::from(1 << 27);
    }
    pub fn allow_manage_roles(&mut self) {
        self.0 |= Inner::from(1 << 28);
    }
    pub fn allow_manage_webhooks(&mut self) {
        self.0 |= Inner::from(1 << 29);
    }
    pub fn allow_manage_emojis_and_stickers(&mut self) {
        self.0 |= Inner::from(1 << 30);
    }
    pub fn allow_use_application_commands(&mut self) {
        self.0 |= Inner::from(1 << 31);
    }
    pub fn allow_request_to_speak(&mut self) {
        self.0 |= Inner::from(1 << 32);
    }
    pub fn allow_manage_events(&mut self) {
        self.0 |= Inner::from(1 << 33);
    }
    pub fn allow_manage_threads(&mut self) {
        self.0 |= Inner::from(1 << 34);
    }
    pub fn allow_create_public_threads(&mut self) {
        self.0 |= Inner::from(1 << 35);
    }
    pub fn allow_create_private_threads(&mut self) {
        self.0 |= Inner::from(1 << 36);
    }
    pub fn allow_use_external_stickers(&mut self) {
        self.0 |= Inner::from(1 << 37);
    }
    pub fn allow_send_messages_in_threads(&mut self) {
        self.0 |= Inner::from(1 << 38);
    }
    pub fn allow_use_embedded_activities(&mut self) {
        self.0 |= Inner::from(1 << 39);
    }
    pub fn allow_moderate_members(&mut self) {
        self.0 |= Inner::from(1 << 40);
    }

    pub fn build(&self) -> Permissions {
        Permissions(self.0.clone())
    }
}
