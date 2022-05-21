use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Locale(String);

impl Locale {
    pub fn code(&self) -> &str {
        &self.0
    }

    pub const DANISH: &'static str = "da";
    pub fn danish() -> Self {
        Self(Self::DANISH.to_owned())
    }

    pub const GERMAN: &'static str = "de";
    pub fn german() -> Self {
        Self(Self::GERMAN.to_owned())
    }

    pub const ENGLISH_UK: &'static str = "en-GB";
    pub fn english_uk() -> Self {
        Self("en-GB".to_owned())
    }

    pub fn english_us() -> Self {
        Self("en-US".to_owned())
    }
    pub fn spanish() -> Self {
        Self("es-ES".to_owned())
    }
    pub fn french() -> Self {
        Self("fr".to_owned())
    }
    pub fn croatian() -> Self {
        Self("hr".to_owned())
    }
    pub fn italian() -> Self {
        Self("it".to_owned())
    }
    pub fn lithuanian() -> Self {
        Self("lt".to_owned())
    }
    pub fn hungarian() -> Self {
        Self("hu".to_owned())
    }
    pub fn dutch() -> Self {
        Self("nl".to_owned())
    }
    pub fn norwegian() -> Self {
        Self("no".to_owned())
    }
    pub fn polish() -> Self {
        Self("pl".to_owned())
    }
    pub fn portuguese_brazilian() -> Self {
        Self("pt-BR".to_owned())
    }
    pub fn romanian() -> Self {
        Self("ro".to_owned())
    }
    pub fn finnish() -> Self {
        Self("fi".to_owned())
    }
    pub fn swedish() -> Self {
        Self("sv-SE".to_owned())
    }
    pub fn vietnamese() -> Self {
        Self("vi".to_owned())
    }
    pub fn turkish() -> Self {
        Self("tr".to_owned())
    }
    pub fn czech() -> Self {
        Self("cs".to_owned())
    }
    pub fn greek() -> Self {
        Self("el".to_owned())
    }
    pub fn bulgarian() -> Self {
        Self("bg".to_owned())
    }
    pub fn russian() -> Self {
        Self("ru".to_owned())
    }
    pub fn ukrainian() -> Self {
        Self("uk".to_owned())
    }
    pub fn hindi() -> Self {
        Self("hi".to_owned())
    }
    pub fn thai() -> Self {
        Self("th".to_owned())
    }
    pub fn chinese_china() -> Self {
        Self("zh-CN".to_owned())
    }
    pub fn japanese() -> Self {
        Self("ja".to_owned())
    }
    pub fn chinese_taiwan() -> Self {
        Self("zh-TW".to_owned())
    }
    pub fn korean() -> Self {
        Self("ko".to_owned())
    }
}
