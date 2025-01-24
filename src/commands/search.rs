use crate::{
    error::JotResult,
    storage::{config::Config, Journal, Tag},
    utils::{self, TagMatch},
};

// TODO: add regex search
#[derive(clap::Args, Clone)]
pub struct SearchArgs {
    pub query: String,
    #[clap(long, value_delimiter = ' ')]
    pub tags: Vec<String>,
    #[clap(long)]
    pub from: Option<String>,
    #[clap(long)]
    pub to: Option<String>,
    #[clap(short, long)]
    pub fuzzy: bool,
    #[clap(short, long)]
    pub all: bool,
    #[clap(short, long)]
    pub case_sensitive: bool,
}

fn check_date_range(
    entry_date: chrono::NaiveDate,
    from: &Option<String>,
    to: &Option<String>,
) -> bool {
    if let Some(from_date) = from {
        let date = utils::parse_date(from_date);
        if date > entry_date {
            return false;
        }
    }

    if let Some(to_date) = to {
        let date = utils::parse_date(to_date);
        if date < entry_date {
            return false;
        }
    }

    true
}

fn check_content_match(content: &str, term: &str, fuzzy: bool) -> bool {
    if fuzzy {
        utils::fuzzy_match(content, term)
    } else {
        content.contains(term)
    }
}

fn print_results(found: Vec<String>, term: &str) {
    println!("{} entries found", found.len());
    found.iter().for_each(|e| {
        if term.is_empty() {
            println!("{}", e);
        } else {
            let highlighted =
                e.to_lowercase()
                    .match_indices(&term)
                    .fold(e.to_string(), |acc, (i, _)| {
                        let orig_match = &acc[i..i + term.len()];
                        acc.replacen(orig_match, &format!("\x1b[42m{}\x1b[0m", orig_match), 1)
                    });
            println!("{}", highlighted);
        }
    });
}

pub fn execute(journal: &Journal, args: SearchArgs, config: &Config) -> JotResult<()> {
    let term = if args.case_sensitive {
        args.query
    } else {
        args.query.to_lowercase()
    };

    let entries = journal.get_entries();

    if entries.is_empty() {
        println!("No entries found.");
    } else {
        let found: Vec<String> = entries
            .iter()
            .filter(|e| {
                let content = if args.case_sensitive {
                    e.body.clone()
                } else {
                    e.body.to_lowercase()
                };

                let content_matches = check_content_match(&content, &term, args.fuzzy);
                let dates_match = check_date_range(e.date, &args.from, &args.to);

                let match_type = if args.all {
                    TagMatch::All
                } else {
                    TagMatch::Any
                };

                let tags_match = utils::do_tags_match(
                    &args
                        .tags
                        .iter()
                        .map(|t| Tag::new(t.to_string()))
                        .collect::<Vec<_>>(),
                    &e.tags,
                    match_type,
                );

                content_matches && dates_match && tags_match
            })
            .map(|e| utils::format_entry(e, config.journal_cfg.clone()))
            .collect();

        print_results(found, &term);
    }

    Ok(())
}
