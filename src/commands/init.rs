use crate::error::JotResult;
use crate::storage;
use crate::storage::config::Config;

pub fn execute(config: &Config) -> JotResult<()> {
    storage::init_journal(config)?;
    Ok(())
}
