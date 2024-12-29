use crate::error::JotResult;
use crate::storage;
use crate::storage::config::{Config, JournalConfig};
use dialoguer::Confirm;

#[derive(clap::Args, Clone)]
pub struct InitArgs {}

pub fn execute(_args: InitArgs) -> JotResult<()> {
    if storage::journal_exists() {
        let proceed = Confirm::new()
            .with_prompt("A journal already exists. Do you want to overwrite it?")
            .default(false)
            .interact()
            .unwrap();

        if !proceed {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    let new_config = Config {
        journal_cfg: JournalConfig {
            show_time: true,
            body_tags: true,
            export_dir: "exports".to_string(),
        },
    };
    storage::init_journal(&new_config)?;
    Ok(())
}
