pub mod cli;
pub mod config;
pub mod naming;
pub mod prompt;
pub mod template;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use crate::cli::Cli;
use crate::prompt::resolve_config;
use crate::template::scaffold_mod;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let config = resolve_config(&cli)?;
    let created_files = scaffold_mod(&config)?;

    println!(
        "\n{} Successfully created RimWorld mod '{}' at:\n   {}",
        "✨".green(),
        config.project_name.cyan().bold(),
        config.target_dir.display().to_string().bold()
    );

    println!("\n{}", "Created files:".green().bold());
    for file in &created_files {
        let display = file.strip_prefix(&config.target_dir).unwrap_or(file);
        println!("   {} {}", "✓".green(), display.display());
    }

    println!("\n{}", "Next steps:".yellow().bold());
    println!("   1. cd {}", config.project_name);
    if config.create_csharp {
        println!(
            "   2. Open Source/{}.slnx or build with `dotnet build Source`",
            config.project_name
        );
    } else {
        println!("   2. Add Defs/ and Patches/ for your XML mod");
    }

    Ok(())
}
