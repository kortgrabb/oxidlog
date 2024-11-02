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
    #[serde(default = "default_true")]
    pub add_tags_to_body: bool,
    #[serde(default = "default_true")]
    pub show_time: bool,
}
// HACK: serde doesn't support default values for bools
fn default_true() -> bool {
    true
}

fn default_journal_dir() -> String {
    let home = dirs::home_dir().unwrap();
    home.join(".jot").to_str().unwrap().to_string()
}
