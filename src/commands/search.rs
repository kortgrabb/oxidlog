use crate::{
    cli::SearchArgs,
    error::JotResult,
    storage::{config::Config, Journal},
    utils,
};

pub fn execute(journal: &Journal, args: SearchArgs, config: &Config) -> JotResult<()> {
    let term = args.query.to_lowercase();
    let entries = journal.get_entries();

    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let found: Vec<String> = entries
            .iter()
            .filter(|e| {
                let content_matches = e.body.to_lowercase().contains(&term);
                utils::do_tags_match(&args.tags, &e.tags) && content_matches
            })
            .map(|e| utils::format_entry(e, config.journal_cfg.show_time))
            .collect();

        println!("{} entries found", found.len());
        found.iter().for_each(|e| println!("{}", e));
    }

    Ok(())
}
