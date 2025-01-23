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

fn remove_entries_by_range(journal: &mut Journal, range: &str) -> JotResult<Vec<usize>> {
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

    Ok((start..=end).collect())
}

fn remove_entries_by_date_range(journal: &Journal, from: &str, to: &str) -> JotResult<Vec<usize>> {
    let from_date = utils::parse_date(from);
    let to_date = utils::parse_date(to);
    Ok(journal
        .get_entries()
        .iter()
        .filter(|e| e.date >= from_date && e.date <= to_date)
        .map(|e| e.id)
        .collect())
}

fn remove_entries_from_date(journal: &Journal, from: &str) -> JotResult<Vec<usize>> {
    let from_date = utils::parse_date(from);
    Ok(journal
        .get_entries()
        .iter()
        .filter(|e| e.date >= from_date)
        .map(|e| e.id)
        .collect())
}

fn remove_entries_to_date(journal: &Journal, to: &str) -> JotResult<Vec<usize>> {
    let to_date = utils::parse_date(to);
    Ok(journal
        .get_entries()
        .iter()
        .filter(|e| e.date <= to_date)
        .map(|e| e.id)
        .collect())
}

pub fn execute(journal: &mut Journal, args: RemoveArgs) -> JotResult<()> {
    let mut to_remove = Vec::new();

    if let Some(id) = args.id {
        to_remove.push(id);
    }

    if let Some(range) = args.range {
        to_remove.extend(remove_entries_by_range(journal, &range)?);
    }

    if let (Some(from), Some(to)) = (args.from.as_ref(), args.to.as_ref()) {
        to_remove.extend(remove_entries_by_date_range(journal, from, to)?);
    } else if let Some(from) = args.from {
        to_remove.extend(remove_entries_from_date(journal, &from)?);
    } else if let Some(to) = args.to {
        to_remove.extend(remove_entries_to_date(journal, &to)?);
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
