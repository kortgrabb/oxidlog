use colored::*;
use storage::load_config;

mod cli;
mod commands;
mod error;
mod storage;
mod utils;

fn main() {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), format!("Failed to load config - {}", e));
            eprintln!("{} Run 'xlog init' to create a new configuration", "Tip:".cyan().bold());
            std::process::exit(1);
        }
    };

    if let Err(e) = cli::run(&config) {
        let error_type = match e {
            error::JotError::_InitError(_) => "Initialization",
            error::JotError::AddError(_) => "Add Entry",
            error::JotError::RemoveError(_) => "Remove Entry",
            error::JotError::EditError(_) => "Edit Entry",
            error::JotError::IoError(_) => "File System",
            error::JotError::SerdeError(_) => "Data Format",
            error::JotError::TomlParseError(_) => "Config Parse",
            error::JotError::TomlSerializeError(_) => "Config Save",
            error::JotError::ExportError(_) => "Export",
            error::JotError::Other(_) => "Unknown",
        };

        eprintln!("\n{} {} Error", "Error:".red().bold(), error_type);
        eprintln!("{} {}", "Details:".yellow().bold(), e);
        
        // Provide helpful tips based on error type
        match e {
            error::JotError::IoError(_) => {
                eprintln!("\n{} Check file permissions and disk space", "Tip:".cyan().bold());
            }
            error::JotError::_InitError(_) => {
                eprintln!("\n{} Try running 'xlog init' again", "Tip:".cyan().bold());
            }
            error::JotError::SerdeError(_) | error::JotError::TomlParseError(_) => {
                eprintln!("\n{} The journal file may be corrupted. Try backing up and reinitializing", "Tip:".cyan().bold());
            }
            _ => {}
        }

        std::process::exit(1);
    }
}
