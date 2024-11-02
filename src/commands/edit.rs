use crate::{
    error::{JotError, JotResult},
    storage::{self, Journal},
    utils,
};
use colored::Colorize;

pub fn execute(journal: &mut Journal, id: usize) -> JotResult<()> {
    match journal.get_entry(id) {
        Some(entry) => {
            println!("Editing entry: {}", entry.body);
            let new_body = utils::get_input("Enter new content: ");

            journal.edit_entry(id, &new_body);
            storage::save_journal(journal)?;

            println!("{}", "Entry updated!".green());

            Ok(())
        }
        None => Err(JotError::EditError(format!(
            "Entry with ID {} not found",
            id
        ))),
    }
}
