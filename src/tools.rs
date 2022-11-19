use colored::Colorize;
use std::process::Command;
use strum_macros::{Display, EnumIter};

#[derive(Debug, EnumIter, Display)]
pub enum Tool {
    Homebrew,
    Neovim,
    Lazygit,
    Packer,
}

pub fn update(tool: &Tool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️ Updating {}", tool.to_string().blue());

    let output = match tool {
        Tool::Homebrew => Command::new("brew").arg("update").output()?,
        Tool::Neovim => Command::new("brew").arg("upgrade").arg("neovim").output()?,
        // Reference: https://github.com/wbthomason/packer.nvim/issues/502
        Tool::Packer => Command::new("nvim")
            .args([
                "--headless",
                "-c",
                "\"PackerSync\"",
                "-c",
                "\"autocmd User PackerComplete quitall\"",
            ])
            .output()?,
        Tool::Lazygit => Command::new("brew")
            .arg("upgrade")
            .arg("lazygit")
            .output()?,
    };

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
