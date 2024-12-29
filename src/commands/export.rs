use crate::{
    error::JotResult,
    storage::{config::Config, Entry, Journal},
};
use chrono::Local;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(clap::Args, Clone)]
pub struct ExportArgs {
    #[clap(value_enum)]
    pub format: ExportFormat,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
    Toml,
    Plain,
}

// Add wrapper struct for TOML
#[derive(Serialize)]
struct TomlWrapper {
    entries: Vec<Entry>,
}

pub fn execute(journal: &mut Journal, args: ExportArgs, config: &Config) -> JotResult<()> {
    let entries = journal.get_entries();
    let export_dir = PathBuf::from(&config.journal_cfg.export_dir);
    fs::create_dir_all(&export_dir)?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = match args.format {
        ExportFormat::Json => format!("journal_{}.json", timestamp),
        ExportFormat::Csv => format!("journal_{}.csv", timestamp),
        ExportFormat::Toml => format!("journal_{}.toml", timestamp),
        ExportFormat::Plain => format!("journal_{}.txt", timestamp),
    };

    let content = match args.format {
        ExportFormat::Json => serde_json::to_string_pretty(&entries)?,
        ExportFormat::Csv => {
            let mut csv = String::from("date,title,body,tags\n");
            for entry in entries {
                let tags = entry.tags.join(",");
                csv.push_str(&format!(
                    "{},{},{}\n",
                    entry.date,
                    entry.body.replace(",", "\\,"),
                    tags
                ));
            }
            csv
        }
        ExportFormat::Toml => {
            let wrapper = TomlWrapper {
                entries: entries.to_vec(),
            };
            toml::to_string(&wrapper)?
        }
        ExportFormat::Plain => {
            let mut text = String::new();
            for entry in entries {
                text.push_str(&format!("Date: {}\n", entry.date));
                if !entry.tags.is_empty() {
                    text.push_str(&format!("Tags: {}\n", entry.tags.join(", ")));
                }
                text.push_str(&format!("\n{}\n", entry.body));
                text.push_str("\n---\n\n");
            }
            text
        }
    };

    let export_path = export_dir.join(filename);
    fs::write(export_path, content)?;

    println!(
        "Journal exported successfully to {}",
        config.journal_cfg.export_dir
    );
    Ok(())
}
