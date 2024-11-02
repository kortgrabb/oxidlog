use colored::Colorize;

use crate::error::{JotError, JotResult};
use crate::storage::{self, Entry, Journal};

pub fn execute(journal: &mut Journal, content: String) -> JotResult<()> {
    let tags = content
        .split_whitespace()
        .filter(|w| w.starts_with('#'))
        .map(|w| w[1..].to_string())
        .collect();

    if content.is_empty() {
        return Err(JotError::AddError("Entry cannot be empty".to_string()));
    }

    let entry = Entry::new(journal.next_id(), content, tags);
    journal.add_entry(entry);
    storage::save_journal(journal)?;

    println!(
        "Entry #{} added!",
        (journal.next_id() - 1).to_string().green()
    );
    Ok(())
}
