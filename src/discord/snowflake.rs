use std::{fmt::Display, num::ParseIntError, str::FromStr};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Snowflake(#[serde(deserialize_with = "serde_aux::deserialize_number_from_string")] i64);

pub const DISCORD_EPOCH: i64 = 1420070400000;

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl Default for Snowflake {
    fn default() -> Self {
        Snowflake::zero()
    }
}

impl Snowflake {
    fn unwrap(&self) -> i64 {
        *&self.0
    }

    pub fn zero() -> Self {
        Snowflake(0)
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        let discord_ts = &self.unwrap() >> 22;
        let unix_ts = discord_ts + DISCORD_EPOCH;
        Utc.timestamp_millis(unix_ts)
    }

    pub fn from_timestamp(timestamp: &DateTime<Utc>) -> Self {
        let unix_ts = timestamp.timestamp_millis();
        let discord_ts = unix_ts - DISCORD_EPOCH;
        Snowflake(discord_ts << 22)
    }

    pub fn increment(&self) -> u16 {
        // Never panic
        (&self.unwrap() & 0xFFF).try_into().unwrap()
    }

    pub fn ipid(&self) -> u8 {
        // Never panic
        (&self.unwrap() & 0x1F000).try_into().unwrap()
    }

    pub fn iwid(&self) -> u8 {
        // Never panic
        (&self.unwrap() & 0x3E0000).try_into().unwrap()
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
