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
        BackupAction::Create => create_backup(journal),
        BackupAction::Restore => restore_backup(journal),
    }
}

fn create_backup(journal: &Journal) -> JotResult<()> {
    let backup = storage::Backup::from_journal(journal);
    backup.create()?;

    println!(
        "Backup created at: {}",
        backup.backup_path.to_string_lossy().green()
    );

    Ok(())
}

fn restore_backup(journal: &mut Journal) -> JotResult<()> {
    let backup = storage::Backup::from_journal(journal);
    backup.restore()?;

    println!(
        "Backup restored from: {}",
        backup.backup_path.to_string_lossy().green()
    );

    Ok(())
}
