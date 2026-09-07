use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "mkrimod",
    author,
    version,
    about = "RimWorld mod project scaffolder"
)]
pub struct Cli {
    /// Name of the mod project (e.g. "MyMod")
    #[arg(value_name = "PROJECT_NAME")]
    pub project_name: Option<String>,

    /// Author of the mod
    #[arg(short, long, value_name = "AUTHOR", default_value = "Author")]
    pub author: String,

    /// Create C# project (Source, .csproj, .slnx)
    #[arg(long)]
    pub csharp: bool,

    /// Target directory to create the mod in (defaults to ./{projectName})
    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Overwrite files if the target directory is not empty
    #[arg(short, long)]
    pub force: bool,

    /// Run interactive prompt wizard
    #[arg(short, long)]
    pub interactive: bool,
}
