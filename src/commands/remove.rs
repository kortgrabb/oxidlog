use crate::error::{JotError, JotResult};
use crate::storage::{self, Journal};
use crate::utils;

#[derive(clap::Args, Clone, Debug)]
pub struct RemoveArgs {
    pub id: Option<usize>,
    #[clap(short, long)]
    pub range: Option<String>,
    #[clap(short, long)]
    pub from: Option<String>,
    #[clap(short, long)]
    pub to: Option<String>,
}

fn remove_entry(journal: &mut Journal, id: usize) -> JotResult<()> {
    if let Some(_) = journal.remove_entry(id) {
        storage::save_journal(journal)
            .map_err(|e| JotError::RemoveError(e.to_string()))
            .map(|_| {
                println!("Entry {} removed", id);
            })
    } else {
        Err(JotError::RemoveError(format!(
            "Entry with ID {} not found",
            id
        )))
    }
}

pub fn execute(journal: &mut Journal, args: RemoveArgs) -> JotResult<()> {
    let mut to_remove = Vec::new();
    let entries = journal.get_entries();

    if let Some(id) = args.id {
        to_remove.push(id);
    }

    if let Some(range) = args.range {
        let range_parts: Vec<&str> = range.split("..").collect();
        if range_parts.len() != 2 {
            return Err(JotError::RemoveError(
                "Invalid range format. Use start..end".to_string(),
            ));
        }

        let start = range_parts[0]
            .parse::<usize>()
            .map_err(|_| JotError::RemoveError("Invalid range start".to_string()))?;
        let end = range_parts[1]
            .parse::<usize>()
            .map_err(|_| JotError::RemoveError("Invalid range end".to_string()))?;

        to_remove.extend(start..=end);
    }

    if let (Some(from), Some(to)) = (args.from.as_ref(), args.to.as_ref()) {
        let from_date = utils::parse_date(from);
        let to_date = utils::parse_date(to);
        to_remove.extend(
            entries
                .iter()
                .filter(|e| e.date >= from_date && e.date <= to_date)
                .map(|e| e.id),
        );
    } else if let Some(from) = args.from {
        let from_date = crate::utils::parse_date(&from);
        to_remove.extend(entries.iter().filter(|e| e.date >= from_date).map(|e| e.id));
    } else if let Some(to) = args.to {
        let to_date = crate::utils::parse_date(&to);
        to_remove.extend(entries.iter().filter(|e| e.date <= to_date).map(|e| e.id));
    }

    if to_remove.is_empty() {
        return Err(JotError::RemoveError("No entries to remove".to_string()));
    }

    // Dedup in case of overlapping ranges
    to_remove.sort_unstable();
    to_remove.dedup();

    to_remove
        .iter()
        .try_for_each(|id| remove_entry(journal, *id))?;

    Ok(())
}
