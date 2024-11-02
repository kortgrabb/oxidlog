use crate::error::JotResult;
use crate::storage;
use crate::storage::config::Config;

// FIXME: ask if overwrite is ok
pub fn execute(config: &Config) -> JotResult<()> {
    storage::init_journal(config)?;
    Ok(())
}
