pub mod config;
mod journal;

use config::Config;
use std::path::PathBuf;
use std::{fs, process};

pub use journal::{Entry, Journal};

const CONFIG_FILE: &str = "config.toml";
const JOURNAL_DIR: &str = ".oxidlog";
const JOURNAL_FILE: &str = "journal.json";

// ! Journal Related

// Update load_journal to take config as parameter
pub fn load_journal() -> Journal {
    let journal_path = if get_journal_path().unwrap().exists() {
        get_journal_path().unwrap()
    } else {
        PathBuf::from(JOURNAL_DIR).join(JOURNAL_FILE)
    };

    match load_from_path(journal_path) {
        Ok(journal) => journal,
        Err(e) => {
            eprintln!("Error loading journal: {}", e);
            process::exit(1);
        }
    }
}

pub fn load_from_path(path: PathBuf) -> std::io::Result<Journal> {
    match fs::read_to_string(&path) {
        Ok(content) => {
            let entries: Vec<Entry> = serde_json::from_str(&content)?;
            Ok(Journal::from_entries(path, entries))
        }
        Err(_) => Ok(Journal::new(path)), // Return an empty journal if the file doesn't exist
    }
}

pub fn save_journal(journal: &Journal) -> std::io::Result<()> {
    let serialized_entries = serde_json::to_string(journal.entries())?;
    match fs::write(journal.path(), serialized_entries) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Journal could not be found, create one with `xlog init`");
            } else {
                eprintln!("Error saving journal: {}", e);
            }
            process::exit(1);
        }
    }
}

/// Get the directory where the journal is stored
pub fn get_journal_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path;
    if cfg!(debug_assertions) {
        path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    } else {
        path = dirs::home_dir().ok_or("Could not find home directory")?;
    }

    path.push(JOURNAL_DIR);
    Ok(path)
}

/// Get the path to the journal file
pub fn get_journal_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = get_journal_dir()?;
    path.push(JOURNAL_FILE);
    Ok(path)
}

// Update init_journal to take config
pub fn init_journal(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let journal_path = if get_journal_path()?.exists() {
        get_journal_path()?
    } else {
        let path = get_journal_path()?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        path
    };

    // create all parent directories if they don't exist
    fs::create_dir_all(journal_path.parent().unwrap())?;
    fs::write(journal_path, "[]")?;

    // Initialize the config file
    save_config(config)?;

    Ok(())
}

pub fn journal_exists() -> bool {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let journal_dir = home_dir.join(".oxidlog");
    journal_dir.exists()
}

// ! Config Related

/// Get the path to the config file
pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = get_journal_dir()?;
    path.push(CONFIG_FILE);
    Ok(path)
}

/// Load configuration from the config file
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Save configuration to the config file
pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let content = toml::to_string(config)?;
    fs::write(config_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_temp_journal() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let journal_path = temp_dir.path().join("test_journal.json");
        (temp_dir, journal_path)
    }

    #[test]
    fn test_load_empty_journal() {
        let (_temp_dir, path) = setup_temp_journal();
        let journal = load_from_path(path.clone()).unwrap();
        assert!(journal.entries().is_empty());
        assert_eq!(*journal.path(), path);
    }

    #[test]
    fn test_save_and_load_journal() {
        let (_temp_dir, path) = setup_temp_journal();

        // Create and save a journal with one entry
        let mut journal = Journal::new(path.clone());
        journal.add_entry(Entry::new(0, "Test entry".to_string(), vec![]));
        save_journal(&journal).unwrap();

        // Load the journal and verify contents
        let loaded_journal = load_from_path(path).unwrap();
        assert_eq!(loaded_journal.entries().len(), 1);
        assert_eq!(loaded_journal.entries()[0].body, "Test entry");
    }

    #[test]
    fn test_get_journal_dir() {
        let dir = get_journal_dir().unwrap();
        if cfg!(debug_assertions) {
            assert!(dir.ends_with(JOURNAL_DIR));
        } else {
            assert!(dir.to_str().unwrap().contains(JOURNAL_DIR));
        }
    }

    #[test]
    fn test_init_journal() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());

        let config = Config::default();
        init_journal(&config).unwrap();

        let journal_dir = get_journal_dir().unwrap();
        let journal_path = get_journal_path().unwrap();

        assert!(journal_dir.exists());
        assert!(journal_path.exists());

        // Should be able to init multiple times without error
        assert!(init_journal(&config).is_ok());
    }
}
