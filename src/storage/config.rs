use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub journal_cfg: JournalConfig,
}

impl Config {
    pub fn journal_path(&self) -> &str {
        &self.journal_cfg.path
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct JournalConfig {
    #[serde(default = "default_journal_dir")]
    path: String,
    #[serde(default)]
    pub add_tags_to_body: bool,
    #[serde(default)]
    pub show_time: bool,
}

fn default_journal_dir() -> String {
    let home = dirs::home_dir().unwrap();
    home.join(".jot").to_str().unwrap().to_string()
}
