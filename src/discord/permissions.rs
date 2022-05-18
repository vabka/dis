use serde::{Deserialize, Serialize};

type Inner = i64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permissions(
    #[serde(deserialize_with = "serde_aux::deserialize_number_from_string")] Inner,
);

pub trait PermissionsProvider {
    fn number_string(&self) -> String;
    fn number(&self) -> Inner;

    fn allowed_to_create_instant_invite(&self) -> bool;
    fn allowed_to_kick_members(&self) -> bool;
    fn allowed_to_ban_members(&self) -> bool;
    fn allowed_to_administrator(&self) -> bool;
    fn allowed_to_manage_channels(&self) -> bool;
    fn allowed_to_manage_guild(&self) -> bool;
    fn allowed_to_add_reactions(&self) -> bool;
    fn allowed_to_view_audit_log(&self) -> bool;
    fn allowed_to_priority_speaker(&self) -> bool;
    fn allowed_to_stream(&self) -> bool;
    fn allowed_to_view_channel(&self) -> bool;
    fn allowed_to_send_messages(&self) -> bool;
    fn allowed_to_send_tts_messages(&self) -> bool;
    fn allowed_to_manage_messages(&self) -> bool;
    fn allowed_to_embed_links(&self) -> bool;
    fn allowed_to_attach_files(&self) -> bool;
    fn allowed_to_read_message_history(&self) -> bool;
    fn allowed_to_mention_everyone(&self) -> bool;
    fn allowed_to_use_external_emojis(&self) -> bool;
    fn allowed_to_view_guild_insights(&self) -> bool;
    fn allowed_to_connect(&self) -> bool;
    fn allowed_to_speak(&self) -> bool;
    fn allowed_to_mute_members(&self) -> bool;
    fn allowed_to_deafen_members(&self) -> bool;
    fn allowed_to_move_members(&self) -> bool;
    fn allowed_to_use_vad(&self) -> bool;
    fn allowed_to_change_nickname(&self) -> bool;
    fn allowed_to_manage_nicknames(&self) -> bool;
    fn allowed_to_manage_roles(&self) -> bool;
    fn allowed_to_manage_webhooks(&self) -> bool;
    fn allowed_to_manage_emojis_and_stickers(&self) -> bool;
    fn allowed_to_use_application_commands(&self) -> bool;
    fn allowed_to_request_to_speak(&self) -> bool;
    fn allowed_to_manage_events(&self) -> bool;
    fn allowed_to_manage_threads(&self) -> bool;
    fn allowed_to_create_lic_threads(&self) -> bool;
    fn allowed_to_create_private_threads(&self) -> bool;
    fn allowed_to_use_external_stickers(&self) -> bool;
    fn allowed_to_send_messages_in_threads(&self) -> bool;
    fn allowed_to_use_embedded_activities(&self) -> bool;
    fn allowed_to_moderate_members(&self) -> bool;
}
fn is_flag_set(value: Inner, offset: Inner) -> bool {
    (value & (1 << offset)) > 0
}

#[allow(clippy::useless_conversion)]
impl PermissionsProvider for Permissions {
    fn number_string(&self) -> String {
        self.0.to_string()
    }
    fn number(&self) -> Inner {
        self.0
    }

    fn allowed_to_create_instant_invite(&self) -> bool {
        is_flag_set(self.0, 0.into())
    }

    fn allowed_to_kick_members(&self) -> bool {
        is_flag_set(self.0,     1.into())
    }

    fn allowed_to_ban_members(&self) -> bool {
        is_flag_set(self.0, 2.into())
    }

    fn allowed_to_administrator(&self) -> bool {
        is_flag_set(self.0, 3.into())
    }

    fn allowed_to_manage_channels(&self) -> bool {
        is_flag_set(self.0, 4.into())
    }

    fn allowed_to_manage_guild(&self) -> bool {
        is_flag_set(self.0, 5.into())
    }

    fn allowed_to_add_reactions(&self) -> bool {
        is_flag_set(self.0, 6.into())
    }

    fn allowed_to_view_audit_log(&self) -> bool {
        is_flag_set(self.0, 7.into())
    }

    fn allowed_to_priority_speaker(&self) -> bool {
        is_flag_set(self.0, 8.into())
    }

    fn allowed_to_stream(&self) -> bool {
        is_flag_set(self.0, 9.into())
    }

    fn allowed_to_view_channel(&self) -> bool {
        is_flag_set(self.0, 10.into())
    }

    fn allowed_to_send_messages(&self) -> bool {
        is_flag_set(self.0, 11.into())
    }

    fn allowed_to_send_tts_messages(&self) -> bool {
        is_flag_set(self.0, 12.into())
    }

    fn allowed_to_manage_messages(&self) -> bool {
        is_flag_set(self.0, 13.into())
    }

    fn allowed_to_embed_links(&self) -> bool {
        is_flag_set(self.0, 14.into())
    }

    fn allowed_to_attach_files(&self) -> bool {
        is_flag_set(self.0, 15.into())
    }

    fn allowed_to_read_message_history(&self) -> bool {
        is_flag_set(self.0, 16.into())
    }

    fn allowed_to_mention_everyone(&self) -> bool {
        is_flag_set(self.0, 17.into())
    }

    fn allowed_to_use_external_emojis(&self) -> bool {
        is_flag_set(self.0, 18.into())
    }

    fn allowed_to_view_guild_insights(&self) -> bool {
        is_flag_set(self.0, 19.into())
    }

    fn allowed_to_connect(&self) -> bool {
        is_flag_set(self.0, 20.into())
    }

    fn allowed_to_speak(&self) -> bool {
        is_flag_set(self.0, 21.into())
    }

    fn allowed_to_mute_members(&self) -> bool {
        is_flag_set(self.0, 22.into())
    }

    fn allowed_to_deafen_members(&self) -> bool {
        is_flag_set(self.0, 23.into())
    }

    fn allowed_to_move_members(&self) -> bool {
        is_flag_set(self.0, 24.into())
    }

    fn allowed_to_use_vad(&self) -> bool {
        is_flag_set(self.0, 25.into())
    }

    fn allowed_to_change_nickname(&self) -> bool {
        is_flag_set(self.0, 26.into())
    }

    fn allowed_to_manage_nicknames(&self) -> bool {
        is_flag_set(self.0, 27.into())
    }

    fn allowed_to_manage_roles(&self) -> bool {
        is_flag_set(self.0, 28.into())
    }

    fn allowed_to_manage_webhooks(&self) -> bool {
        is_flag_set(self.0, 29.into())
    }

    fn allowed_to_manage_emojis_and_stickers(&self) -> bool {
        is_flag_set(self.0, 30.into())
    }

    fn allowed_to_use_application_commands(&self) -> bool {
        is_flag_set(self.0, 31.into())
    }

    fn allowed_to_request_to_speak(&self) -> bool {
        is_flag_set(self.0, 32.into())
    }

    fn allowed_to_manage_events(&self) -> bool {
        is_flag_set(self.0, 33.into())
    }

    fn allowed_to_manage_threads(&self) -> bool {
        is_flag_set(self.0, 34.into())
    }

    fn allowed_to_create_lic_threads(&self) -> bool {
        is_flag_set(self.0, 35.into())
    }

    fn allowed_to_create_private_threads(&self) -> bool {
        is_flag_set(self.0, 36.into())
    }

    fn allowed_to_use_external_stickers(&self) -> bool {
        is_flag_set(self.0, 37.into())
    }

    fn allowed_to_send_messages_in_threads(&self) -> bool {
        is_flag_set(self.0, 38.into())
    }

    fn allowed_to_use_embedded_activities(&self) -> bool {
        is_flag_set(self.0, 39.into())
    }

    fn allowed_to_moderate_members(&self) -> bool {
        is_flag_set(self.0, 40.into())
    }
}
#[allow(clippy::useless_conversion)]
impl PermissionsProvider for PermissionsMut {
    fn number_string(&self) -> String {
        self.0.to_string()
    }
    fn number(&self) -> Inner {
        self.0
    }


    fn allowed_to_create_instant_invite(&self) -> bool {
        is_flag_set(self.0, 0.into())
    }

    fn allowed_to_kick_members(&self) -> bool {
        is_flag_set(self.0,     1.into())
    }

    fn allowed_to_ban_members(&self) -> bool {
        is_flag_set(self.0, 2.into())
    }

    fn allowed_to_administrator(&self) -> bool {
        is_flag_set(self.0, 3.into())
    }

    fn allowed_to_manage_channels(&self) -> bool {
        is_flag_set(self.0, 4.into())
    }

    fn allowed_to_manage_guild(&self) -> bool {
        is_flag_set(self.0, 5.into())
    }

    fn allowed_to_add_reactions(&self) -> bool {
        is_flag_set(self.0, 6.into())
    }

    fn allowed_to_view_audit_log(&self) -> bool {
        is_flag_set(self.0, 7.into())
    }

    fn allowed_to_priority_speaker(&self) -> bool {
        is_flag_set(self.0, 8.into())
    }

    fn allowed_to_stream(&self) -> bool {
        is_flag_set(self.0, 9.into())
    }

    fn allowed_to_view_channel(&self) -> bool {
        is_flag_set(self.0, 10.into())
    }

    fn allowed_to_send_messages(&self) -> bool {
        is_flag_set(self.0, 11.into())
    }

    fn allowed_to_send_tts_messages(&self) -> bool {
        is_flag_set(self.0, 12.into())
    }

    fn allowed_to_manage_messages(&self) -> bool {
        is_flag_set(self.0, 13.into())
    }

    fn allowed_to_embed_links(&self) -> bool {
        is_flag_set(self.0, 14.into())
    }

    fn allowed_to_attach_files(&self) -> bool {
        is_flag_set(self.0, 15.into())
    }

    fn allowed_to_read_message_history(&self) -> bool {
        is_flag_set(self.0, 16.into())
    }

    fn allowed_to_mention_everyone(&self) -> bool {
        is_flag_set(self.0, 17.into())
    }

    fn allowed_to_use_external_emojis(&self) -> bool {
        is_flag_set(self.0, 18.into())
    }

    fn allowed_to_view_guild_insights(&self) -> bool {
        is_flag_set(self.0, 19.into())
    }

    fn allowed_to_connect(&self) -> bool {
        is_flag_set(self.0, 20.into())
    }

    fn allowed_to_speak(&self) -> bool {
        is_flag_set(self.0, 21.into())
    }

    fn allowed_to_mute_members(&self) -> bool {
        is_flag_set(self.0, 22.into())
    }

    fn allowed_to_deafen_members(&self) -> bool {
        is_flag_set(self.0, 23.into())
    }

    fn allowed_to_move_members(&self) -> bool {
        is_flag_set(self.0, 24.into())
    }

    fn allowed_to_use_vad(&self) -> bool {
        is_flag_set(self.0, 25.into())
    }

    fn allowed_to_change_nickname(&self) -> bool {
        is_flag_set(self.0, 26.into())
    }

    fn allowed_to_manage_nicknames(&self) -> bool {
        is_flag_set(self.0, 27.into())
    }

    fn allowed_to_manage_roles(&self) -> bool {
        is_flag_set(self.0, 28.into())
    }

    fn allowed_to_manage_webhooks(&self) -> bool {
        is_flag_set(self.0, 29.into())
    }

    fn allowed_to_manage_emojis_and_stickers(&self) -> bool {
        is_flag_set(self.0, 30.into())
    }

    fn allowed_to_use_application_commands(&self) -> bool {
        is_flag_set(self.0, 31.into())
    }

    fn allowed_to_request_to_speak(&self) -> bool {
        is_flag_set(self.0, 32.into())
    }

    fn allowed_to_manage_events(&self) -> bool {
        is_flag_set(self.0, 33.into())
    }

    fn allowed_to_manage_threads(&self) -> bool {
        is_flag_set(self.0, 34.into())
    }

    fn allowed_to_create_lic_threads(&self) -> bool {
        is_flag_set(self.0, 35.into())
    }

    fn allowed_to_create_private_threads(&self) -> bool {
        is_flag_set(self.0, 36.into())
    }

    fn allowed_to_use_external_stickers(&self) -> bool {
        is_flag_set(self.0, 37.into())
    }

    fn allowed_to_send_messages_in_threads(&self) -> bool {
        is_flag_set(self.0, 38.into())
    }

    fn allowed_to_use_embedded_activities(&self) -> bool {
        is_flag_set(self.0, 39.into())
    }

    fn allowed_to_moderate_members(&self) -> bool {
        is_flag_set(self.0, 40.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionsMut(#[serde(deserialize_with = "serde_aux::deserialize_number_from_string")] Inner);

impl Default for PermissionsMut {
    fn default() -> Self { PermissionsMut::new() }
}

#[allow(clippy::useless_conversion)]
impl PermissionsMut {
    pub fn new() -> Self {
        PermissionsMut(0.into())
    }
    pub fn allow_to_create_instant_invite(&mut self) {
        self.0 |= Inner::from(1 << 0);
    }
    pub fn allow_to_kick_members(&mut self) {
        self.0 |= Inner::from(1 << 1);
    }
    pub fn allow_to_ban_members(&mut self) {
        self.0 |= Inner::from(1 << 2);
    }
    pub fn allow_to_administrator(&mut self) {
        self.0 |= Inner::from(1 << 3);
    }
    pub fn allow_to_manage_channels(&mut self) {
        self.0 |= Inner::from(1 << 4);
    }
    pub fn allow_to_manage_guild(&mut self) {
        self.0 |= Inner::from(1 << 5);
    }
    pub fn allow_to_add_reactions(&mut self) {
        self.0 |= Inner::from(1 << 6);
    }
    pub fn allow_to_view_audit_log(&mut self) {
        self.0 |= Inner::from(1 << 7);
    }
    pub fn allow_to_priority_speaker(&mut self) {
        self.0 |= Inner::from(1 << 8);
    }
    pub fn allow_to_stream(&mut self) {
        self.0 |= Inner::from(1 << 9);
    }
    pub fn allow_to_view_channel(&mut self) {
        self.0 |= Inner::from(1 << 10);
    }
    pub fn allow_to_send_messages(&mut self) {
        self.0 |= Inner::from(1 << 11);
    }
    pub fn allow_to_send_tts_messages(&mut self) {
        self.0 |= Inner::from(1 << 12);
    }
    pub fn allow_to_manage_messages(&mut self) {
        self.0 |= Inner::from(1 << 13);
    }
    pub fn allow_to_embed_links(&mut self) {
        self.0 |= Inner::from(1 << 14);
    }
    pub fn allow_to_attach_files(&mut self) {
        self.0 |= Inner::from(1 << 15);
    }
    pub fn allow_to_read_message_history(&mut self) {
        self.0 |= Inner::from(1 << 16);
    }
    pub fn allow_to_mention_everyone(&mut self) {
        self.0 |= Inner::from(1 << 17);
    }
    pub fn allow_to_use_external_emojis(&mut self) {
        self.0 |= Inner::from(1 << 18);
    }
    pub fn allow_to_view_guild_insights(&mut self) {
        self.0 |= Inner::from(1 << 19);
    }
    pub fn allow_to_connect(&mut self) {
        self.0 |= Inner::from(1 << 20);
    }
    pub fn allow_to_speak(&mut self) {
        self.0 |= Inner::from(1 << 21);
    }
    pub fn allow_to_mute_members(&mut self) {
        self.0 |= Inner::from(1 << 22);
    }
    pub fn allow_to_deafen_members(&mut self) {
        self.0 |= Inner::from(1 << 23);
    }
    pub fn allow_to_move_members(&mut self) {
        self.0 |= Inner::from(1 << 24);
    }
    pub fn allow_to_use_vad(&mut self) {
        self.0 |= Inner::from(1 << 25);
    }
    pub fn allow_to_change_nickname(&mut self) {
        self.0 |= Inner::from(1 << 26);
    }
    pub fn allow_to_manage_nicknames(&mut self) {
        self.0 |= Inner::from(1 << 27);
    }
    pub fn allow_to_manage_roles(&mut self) {
        self.0 |= Inner::from(1 << 28);
    }
    pub fn allow_to_manage_webhooks(&mut self) {
        self.0 |= Inner::from(1 << 29);
    }
    pub fn allow_to_manage_emojis_and_stickers(&mut self) {
        self.0 |= Inner::from(1 << 30);
    }
    pub fn allow_to_use_application_commands(&mut self) {
        self.0 |= Inner::from(1 << 31);
    }
    pub fn allow_to_request_to_speak(&mut self) {
        self.0 |= Inner::from(1_i64 << 32);
    }
    pub fn allow_to_manage_events(&mut self) {
        self.0 |= Inner::from(1_i64 << 33);
    }
    pub fn allow_to_manage_threads(&mut self) {
        self.0 |= Inner::from(1_i64 << 34);
    }
    pub fn allow_to_create_public_threads(&mut self) {
        self.0 |= Inner::from(1_i64 << 35);
    }
    pub fn allow_to_create_private_threads(&mut self) {
        self.0 |= Inner::from(1_i64 << 36);
    }
    pub fn allow_to_use_external_stickers(&mut self) {
        self.0 |= Inner::from(1_i64 << 37);
    }
    pub fn allow_to_send_messages_in_threads(&mut self) {
        self.0 |= Inner::from(1_i64 << 38);
    }
    pub fn allow_to_use_embedded_activities(&mut self) {
        self.0 |= Inner::from(1_i64 << 39);
    }
    pub fn allow_to_moderate_members(&mut self) {
        self.0 |= Inner::from(1_i64 << 40);
    }

    pub fn freeze(self) -> Permissions {
        Permissions(self.0)
    }
}
