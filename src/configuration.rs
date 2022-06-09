use std::env;
use crate::Snowflake;

pub struct BotConfig {
    pub token: String,
    pub socket_addr: (String, u16),
    pub intent_bits: u64,
    pub app_id: Snowflake,
    pub bot_url: String,
    pub base_url: String,
    pub public_key: ed25519_dalek::PublicKey,
    pub storage_path: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigLoadError {
    #[error("Invalid value for field '{}'. Expected: '{}'", field_name, expected)]
    InvalidValue {
        field_name: &'static str,
        expected: &'static str,
    },

    #[error("Missing required value for field '{}'", field_name)]
    MissingRequired {
        field_name: &'static str
    },
}

impl BotConfig {
    pub fn load_env() -> Result<Self, ConfigLoadError> {
        use ConfigLoadError::{InvalidValue, MissingRequired};
        const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
        const LISTEN_ADDRESS: &str = "LISTEN";
        const PORT: &str = "PORT";
        const PERMISSIONS: &str = "PERMISSIONS_INTEGER";
        const DISCORD_BASE_URL: &str = "BASE_URL";
        const CLIENT_ID: &str = "CLID";
        const BOT_URL: &str = "URL";
        const PUBLIC_KEY: &str = "PUBLIC_KEY";
        const STORAGE_PATH: &str = "STORAGE_PATH";

        let token = env::var(DISCORD_TOKEN)
            .map_err(|_| MissingRequired { field_name: DISCORD_TOKEN })?;

        let socket_addr = {
            let listen_addr = env::var(LISTEN_ADDRESS)
                .unwrap_or_else(|_| "127.0.0.1".to_string());

            let port: u16 = env::var(PORT)
                .map(|s| s.parse::<u16>())
                .unwrap_or(Ok(8080u16))
                .map_err(|_| InvalidValue {
                    field_name: PORT,
                    expected: "Integer in range 0..u16::MAX",
                })?;

            (listen_addr, port)
        };

        let intent_bits = {
            let intent_str = env::var(PERMISSIONS)
                .map_err(|_| MissingRequired { field_name: PERMISSIONS })?;

            let intent_bits: u64 = intent_str.parse::<u64>()
                .map_err(|_| InvalidValue {
                    field_name: PERMISSIONS,
                    expected: "Integer in range 0..u64::MAX",
                })?;
            intent_bits
        };
        let base_url = env::var(DISCORD_BASE_URL).unwrap_or_else(|_| "https://discord.com/api".to_owned());

        let app_id = env::var(CLIENT_ID)
            .map_err(|_| MissingRequired { field_name: CLIENT_ID })?
            .parse()
            .map_err(|_| InvalidValue { field_name: CLIENT_ID, expected: "Valid snowflake id of app" })?;

        let bot_url = env::var(BOT_URL).map_err(|_| MissingRequired { field_name: BOT_URL })?;

        let public_key = {
            let text = env::var(PUBLIC_KEY)
                .map_err(|_| MissingRequired { field_name: PUBLIC_KEY })?;
            let byte_vec = hex::decode(text)
                .map_err(|_| InvalidValue {
                    field_name: PUBLIC_KEY,
                    expected: "Valid ed25519 public key obtained from Discord",
                })?;
            let key = ed25519_dalek::PublicKey::from_bytes(&byte_vec)
                .map_err(|_| InvalidValue {
                    field_name: PUBLIC_KEY,
                    expected: "Valid ed25519 public key obtained from Discord",
                })?;
            key
        };

        let storage_path = env::var(STORAGE_PATH)
            .map_err(|_| MissingRequired { field_name: STORAGE_PATH })?;

        Ok(BotConfig {
            token,
            socket_addr,
            intent_bits,
            base_url,
            app_id,
            bot_url,
            public_key,
            storage_path,
        })
    }
}

