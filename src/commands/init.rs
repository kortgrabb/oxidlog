use crate::error::JotResult;
use crate::storage;
use crate::storage::config::{Config, JournalConfig};

pub fn execute() -> JotResult<()> {
    let new_config = Config {
        journal_cfg: JournalConfig {
            show_time: true,
            body_tags: true,
            ..Default::default()
        }
    };
    storage::init_journal(&new_config)?;
    Ok(())
}
