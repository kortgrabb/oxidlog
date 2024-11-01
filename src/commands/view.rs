use crate::{error::JotResult, storage::Journal};

pub fn execute(journal: &Journal) -> JotResult<()> {
    let entires = journal.get_entries();
    if entires.is_empty() {
        println!("No entries found.");
    } else {
        for (i, entry) in entires.iter().enumerate() {
            println!("{entry}");
        }
    }

    Ok(())
}
