use crate::error::{JotError, JotResult};
use crate::storage::{self, Journal};

pub fn execute(journal: &mut Journal, content: String) -> JotResult<()> {
    let entry = journal.add_entry(content);
    match storage::save_journal(journal) {
        Ok(_) => {
            println!("Added entry: {}", entry.content);
            Ok(())
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Err(JotError::AddError(
                    "Journal not found. Run `jot init` to create one.".to_string(),
                ))
            } else {
                Err(JotError::AddError(e.to_string()))
            }
        }
    }
}
