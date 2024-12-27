use crate::{
    cli::RemoveArgs,
    error::{JotError, JotResult},
    storage::{self, Journal},
};

// TODO: range support (e.g. 1..5) for multiple entries
// TODO: to/from date range support
pub fn execute(journal: &mut Journal, args: RemoveArgs) -> JotResult<()> {
    dbg!(args.range);
    let id = args
        .id
        .ok_or_else(|| JotError::RemoveError("No ID provided".to_string()))?;
    match journal.remove_entry(id) {
        Some(_) => match storage::save_journal(journal) {
            Ok(_) => {
                println!("Entry removed successfully");
                Ok(())
            }
            Err(e) => Err(JotError::RemoveError(e.to_string())),
        },
        None => Err(JotError::RemoveError(format!(
            "Entry with ID {} not found",
            id
        ))),
    }
}
