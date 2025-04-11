//! # Minigrep Library
//!
//! This library provides the core functionality for the `minigrep` CLI application.
//! It includes logic for parsing command-line arguments, reading file content,
//! and performing a search for lines that match a given query string.
//!
//! The search can be case-sensitive or case-insensitive, based on an environment variable.

use std::fs;
use std::error::Error;
use std::env;

/// Holds the configuration for the search operation.
///
/// This includes the search query, the path to the file to search in,
/// and a flag indicating whether the search should ignore case.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Constructs a new `Config` from the given iterator of arguments.
    ///
    /// The iterator should contain:
    /// - Program name (ignored)
    /// - Search query string
    /// - Path to the file
    ///
    /// Returns an error if any required argument is missing.
    /// Case-insensitivity is determined by checking if the `IGNORE_CASE` environment variable is set.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Runs the main logic of the application:
/// - Reads the file content
/// - Performs the search based on the query and case-sensitivity
/// - Prints each matching line
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

/// Performs a case-sensitive search.
///
/// Returns all lines in `contents` that contain the `query` string.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Performs a case-insensitive search.
///
/// Returns all lines in `contents` that contain the `query` string,
/// ignoring letter case.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
