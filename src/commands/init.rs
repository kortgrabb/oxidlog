use crate::{error::JotError, error::JotResult, storage};

pub fn execute() -> JotResult<()> {
    match storage::init_journal() {
        Ok(_) => Ok(()),
        Err(e) => Err(JotError::InitError(e.to_string())),
    }
}
