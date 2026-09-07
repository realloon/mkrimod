use anyhow::{Result, bail};
use inquire::{Confirm, InquireError, Text};
use std::io::IsTerminal;
use std::path::Path;

use crate::cli::Cli;
use crate::config::ModConfig;

pub fn resolve_config(args: &Cli) -> Result<ModConfig> {
    let is_interactive =
        args.interactive || (args.project_name.is_none() && std::io::stdin().is_terminal());

    let (project_name, author, create_csharp) = if is_interactive {
        let name = unwrap_prompt(
            Text::new("Project name")
                .with_default(
                    args.project_name
                        .as_deref()
                        .unwrap_or(ModConfig::DEFAULT_PROJECT_NAME),
                )
                .prompt(),
            ModConfig::DEFAULT_PROJECT_NAME.to_string(),
        )?;

        let auth = unwrap_prompt(
            Text::new("Author").with_default(&args.author).prompt(),
            ModConfig::DEFAULT_AUTHOR.to_string(),
        )?;

        let csharp = unwrap_prompt(
            Confirm::new("Create C# project")
                .with_default(true)
                .prompt(),
            true,
        )?;

        (name, auth, csharp)
    } else {
        let name = args
            .project_name
            .clone()
            .unwrap_or_else(|| ModConfig::DEFAULT_PROJECT_NAME.to_string());
        (name, args.author.clone(), args.csharp)
    };

    let config = ModConfig::new(&project_name, &author, create_csharp, args.output.clone());

    check_directory_safety(&config.target_dir, args.force, is_interactive)?;

    Ok(config)
}

fn unwrap_prompt<T>(res: Result<T, InquireError>, default: T) -> Result<T> {
    match res {
        Ok(val) => Ok(val),
        Err(InquireError::NotTTY) => Ok(default),
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
            bail!("Operation aborted");
        }
        Err(e) => Err(e.into()),
    }
}

fn check_directory_safety(target_dir: &Path, force: bool, interactive: bool) -> Result<()> {
    let is_not_empty = target_dir
        .read_dir()
        .map(|mut entries| entries.next().is_some())
        .unwrap_or(false);

    if !is_not_empty || force {
        return Ok(());
    }

    if interactive {
        let question = format!(
            "Target directory '{}' is not empty. Continue and potentially overwrite files?",
            target_dir.display()
        );
        match Confirm::new(&question).with_default(false).prompt() {
            Ok(true) => return Ok(()),
            Ok(false) => bail!("Aborted: target directory is not empty"),
            Err(InquireError::NotTTY) => {}
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                bail!("Operation aborted");
            }
            Err(e) => return Err(e.into()),
        }
    }

    bail!(
        "Target directory '{}' is not empty. Use --force to proceed.",
        target_dir.display()
    );
}
