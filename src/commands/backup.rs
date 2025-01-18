use crate::{
    error::JotResult,
    storage::{self, Journal},
};
use colored::Colorize;

#[derive(Debug, Clone)]
enum BackupAction {
    Create,
    Restore,
}

impl std::str::FromStr for BackupAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" | "c" => Ok(BackupAction::Create),
            "restore" | "r" => Ok(BackupAction::Restore),
            _ => Err("Invalid backup action".to_string()),
        }
    }
}

#[derive(clap::Args, Clone)]
pub struct BackupArgs {
    #[clap(default_value = "create")]
    action: BackupAction,
}

pub fn execute(journal: &mut Journal, args: BackupArgs) -> JotResult<()> {
    match args.action {
        BackupAction::Create => create(journal),
        BackupAction::Restore => restore(journal),
    }
}

fn create(journal: &Journal) -> JotResult<()> {
    let backup_creater = storage::Backup::from_journal(journal);
    backup_creater.create()?;

    println!(
        "Backup created at: {}",
        backup_creater.backup_path.to_string_lossy().green()
    );

    Ok(())
}

fn restore(journal: &mut Journal) -> JotResult<()> {
    let backup_creater = storage::Backup::from_journal(journal);
    backup_creater.restore()?;

    println!(
        "Backup restored from: {}",
        backup_creater.backup_path.to_string_lossy().green()
    );

    Ok(())
}
