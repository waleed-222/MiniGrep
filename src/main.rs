//! # Minigrep CLI Application
//!
//! This is the binary crate for the `minigrep` project.
//! It parses command-line arguments, builds a configuration,
//! and delegates the search logic to the `minigrep` library.

use std::env;
use std::process;

use ::minigrep::Config;

/// Entry point of the application.
/// Parses arguments and runs the main logic defined in the `minigrep` library.
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
