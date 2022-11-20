use strum::IntoEnumIterator;
mod tools;

use colored::Colorize;
use std::process::exit;

use crate::tools::{select_tools, update, Tool};

fn welcome() {
    println!(
        "{}",
        "🔄 Welcome to insync, ensuring your tools are up to date."
            .green()
            .bold()
    );
}

/// The `insync` application is a simple command line tool that allows me to
/// ensure popularly used tools are up to date.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    welcome();

    let tools = Tool::iter().collect::<Vec<_>>();
    let chosen = select_tools(&tools)?;

    if chosen.is_empty() {
        println!("👋 No tools selected. Exiting.");
        exit(0);
    }

    for i in chosen {
        update(&tools[i])?;
    }

    Ok(())
}
