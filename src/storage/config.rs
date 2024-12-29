use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub journal_cfg: JournalConfig,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct JournalConfig {
    pub body_tags: bool,
    pub show_time: bool,
    #[serde(default = "default_export_dir")]
    pub export_dir: String,
}

fn default_export_dir() -> String {
    "exports".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(!config.journal_cfg.body_tags);
        assert!(!config.journal_cfg.show_time);
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.journal_cfg.body_tags = true;
        config.journal_cfg.show_time = true;

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.journal_cfg.body_tags, true);
        assert_eq!(deserialized.journal_cfg.show_time, true);
    }
}
