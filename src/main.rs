use strum::IntoEnumIterator;
mod tools;

use colored::Colorize;
use dialoguer::MultiSelect;
use std::process::{exit, Command};

use crate::tools::{update, Tool};

/// The `insync` application is a simple command line tool that allows me to
/// ensure popularly used tools are up to date.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Welcome prompt for the user
    println!(
        "{}",
        "🔄 Welcome to insync, ensuring your tools are up to date."
            .green()
            .bold()
    );

    println!(
        "{}",
        "Please select the tools you would like to update.".bold()
    );
    let subtitle = format!(
        "❓ Press {} to select, and {} to finish. Selecting no tools will exit the application.",
        "SPACE".bold(),
        "ENTER".bold()
    );
    println!();
    println!("{}", subtitle);

    let tools = Tool::iter().collect::<Vec<_>>();
    let chosen: Vec<usize> = MultiSelect::new().items(&tools).interact()?;

    if chosen.is_empty() {
        println!("👋 No tools selected. Exiting.");
        exit(0);
    }

    for i in chosen {
        update(&tools[i])?;
    }

    Ok(())
}
