use crate::{error::JotResult, storage::Journal};

pub fn execute(journal: &Journal, term: &str) -> JotResult<()> {
    let term = term.to_lowercase();
    let entries = journal.get_entries();
    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let mut found = false;
        for entry in entries {
            if entry.body.to_lowercase().contains(&term) {
                println!("{}", entry);
                found = true;
            }
        }

        if !found {
            println!("No entries found containing: {}", term);
        }
    }

    Ok(())
}
