use crate::{
    cli::RemoveArgs, error::{JotError, JotResult}, storage::{self, Journal}
};

// TODO: range support (e.g. 1..5) for multiple entries
// TODO: to/from date range support
pub fn execute(
    journal: &mut Journal,
    args: RemoveArgs,
) -> JotResult<()> {
    match journal.remove_entry(args.id) {
        Some(_) => match storage::save_journal(journal) {
            Ok(_) => {
                println!("Entry removed successfully");
                Ok(())
            }
            Err(e) => Err(JotError::RemoveError(e.to_string())),
        },
        None => Err(JotError::RemoveError(format!(
            "No entry found with id: {}",
            args.id
        ))),
    }
}
