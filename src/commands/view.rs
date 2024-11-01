use crate::{error::JotResult, storage::Journal};

pub fn execute(
    journal: &Journal,
    id: Option<usize>,
    from: Option<String>,
    to: Option<String>,
) -> JotResult<()> {
    if let Some(id) = id {
        if let Some(entry) = journal.entries().iter().find(|e| e.id == id) {
            println!("ID: {}", entry.id);
            println!("Body: {}", entry.body);
            println!("Tags: {:?}", entry.tags);
            println!("Date: {}", entry.date);
            println!("Timestamp: {}", entry.timestamp);
        } else {
            eprintln!("Entry with id {} not found", id);
        }
    } else {
        let entries = journal.get_entries();
        let entries = entries
            .iter()
            .filter(|e| {
                // TODO: move into utils
                if let Some(from) = &from {
                    // parse simple date format: yyyy-mm-dd
                    if let Ok(from) = chrono::NaiveDate::parse_from_str(from, "%Y-%m-%d") {
                        if e.date < from {
                            return false;
                        }
                    } else {
                        eprintln!("Invalid date format for --from");
                        println!("Expected format: yyyy-mm-dd");
                        std::process::exit(1);
                    }
                }
                if let Some(to) = &to {
                    if let Ok(to) = chrono::NaiveDate::parse_from_str(to, "%Y-%m-%d") {
                        if e.date > to {
                            return false;
                        }
                    } else {
                        eprintln!("Invalid date format for --to");
                        println!("Expected format: yyyy-mm-dd");
                        std::process::exit(1);
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        for entry in entries {
            println!("{}", entry);
        }
    }

    Ok(())
}
