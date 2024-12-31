use crate::{
    error::{JotError, JotResult},
    storage::{config::Config, Journal},
};
use chrono::Local;
use std::fs;

#[derive(clap::Args, Clone)]
pub struct ExportArgs {
    #[clap(value_enum)]
    /// The format to export the journal in (json, csv, plain)
    pub format: ExportFormat,

    #[clap(short, long)]
    /// Open the exported file with the default program
    pub open: bool,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
    Plain,
}

pub fn execute(journal: &mut Journal, args: ExportArgs, config: &Config) -> JotResult<()> {
    let entries = journal.get_entries();
    let export_dir = journal.path()
        .parent()
        .unwrap_or(journal.path())
        .join(&config.journal_cfg.export_dir);
    fs::create_dir_all(&export_dir)?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = match args.format {
        ExportFormat::Json => format!("journal_{}.json", timestamp),
        ExportFormat::Csv => format!("journal_{}.csv", timestamp),
        ExportFormat::Plain => format!("journal_{}.txt", timestamp),
    };

    let content = match args.format {
        ExportFormat::Json => serde_json::to_string_pretty(&entries)?,
        ExportFormat::Csv => {
            let mut csv = String::from("date,title,body,tags\n");
            for entry in entries {
                let tags = entry.tags.iter()
                    .map(|t| t.name.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                csv.push_str(&format!(
                    "{},{},{}\n",
                    entry.date,
                    entry.body.replace("\n", " "),
                    tags

                    
                ));
            }
            csv
        }
        ExportFormat::Plain => {
            let mut text = String::new();
            for entry in entries {
                text.push_str(&format!("Date: {}\n", entry.date));
                if !entry.tags.is_empty() {
                    let tags_str = entry.tags.iter()
                        .map(|t| t.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    text.push_str(&format!("Tags: {}\n", tags_str));
                }
                text.push_str(&format!("\n{}\n", entry.body));
                text.push_str("\n---\n\n");
            }
            text
        }
    };

    let export_path = export_dir.join(filename);
    fs::write(&export_path, content)?;

    if args.open {
        // run xdg-open on Linux, open on macOS, or start on Windows
        let platform = std::env::consts::OS;
        let command = match platform {
            "linux" => "xdg-open",
            "macos" => "open",
            "windows" => "start",
            _ => {
                return Err(JotError::ExportError(
                    format!("Cannot open exported file: unsupported platform '{}'", platform)
                ));
            }
        };

        let path = export_path.to_str().ok_or_else(|| {
            JotError::ExportError("Export path contains invalid Unicode".to_string())
        })?;

        let status = std::process::Command::new(command).arg(path).status()?;

        if !status.success() {
            return Err(JotError::ExportError(
                format!("Failed to open exported file '{}' with system command '{}'", path, command)
            ));
        }
    }

    println!(
        "Journal exported successfully to {}",
        config.journal_cfg.export_dir
    );
    Ok(())
}
