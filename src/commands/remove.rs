use crate::{
    error::{JotError, JotResult},
    storage::{self, Journal},
};

pub fn execute(journal: &mut Journal, id: usize) -> JotResult<()> {
    match journal.remove_entry(id) {
        Some(entry) => match storage::save_journal(journal) {
            Ok(_) => {
                println!("Removed entry: {}", entry.body);
                Ok(())
            }
            Err(e) => Err(JotError::RemoveError(e.to_string())),
        },
        None => Err(JotError::RemoveError(format!(
            "No entry found with id: {}",
            id
        ))),
    }
}
