use crate::{error::JotResult, storage::Journal};

pub fn execute(journal: &Journal) -> JotResult<()> {
    let entires = journal.get_entries();
    if entires.is_empty() {
        println!("No entries found.");
    } else {
        for entry in entires.iter() {
            println!("{entry}");
        }
    }

    Ok(())
}
