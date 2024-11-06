use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub journal_cfg: JournalConfig,
}

#[derive(Serialize, Deserialize, Default)]
pub struct JournalConfig {
    pub body_tags: bool,
    pub show_time: bool,
}