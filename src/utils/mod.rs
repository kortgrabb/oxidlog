use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn matches_tags(tags: &[String], entry_tags: &[String]) -> bool {
    if tags.is_empty() {
        return true;
    }
    entry_tags.iter().any(|t| tags.contains(&t.to_string()))
}

pub fn parse_date(date: &str) -> chrono::NaiveDate {
    match chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Invalid date format");
            std::process::exit(1);
        }
    }
}
