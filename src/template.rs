use anyhow::{Context, Result};
use include_dir::{Dir, DirEntry, include_dir};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::ModConfig;

pub static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/template");

pub fn render_template(content: &str, values: &[(&str, &str)]) -> String {
    let mut result = content.to_string();
    for &(k, v) in values {
        result = result.replace(k, v);
    }
    result
}

fn render_dir(
    dir: &Dir<'_>,
    target_dir: &Path,
    values: &[(&str, &str)],
    created_files: &mut Vec<PathBuf>,
) -> Result<()> {
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

    for entry in dir.entries() {
        let name = entry
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let target = target_dir.join(render_template(name, values));

        match entry {
            DirEntry::File(file) => {
                let raw = file.contents_utf8().context("Template file is not valid UTF-8")?;
                fs::write(&target, render_template(raw, values))
                    .with_context(|| format!("Failed to write file: {}", target.display()))?;
                created_files.push(target);
            }
            DirEntry::Dir(sub) => {
                render_dir(sub, &target, values, created_files)?;
            }
        }
    }
    Ok(())
}

pub fn scaffold_mod(config: &ModConfig) -> Result<Vec<PathBuf>> {
    let package_id = config.package_id();
    let values = [
        ("{projectName}", config.project_name.as_str()),
        ("{author}", config.author.as_str()),
        ("{packageId}", package_id.as_str()),
    ];
    let target_dir = &config.target_dir;
    let mut created_files = Vec::new();

    let about_dir = TEMPLATE_DIR
        .get_dir("About")
        .context("Missing 'About' template directory in embedded assets")?;
    render_dir(about_dir, &target_dir.join("About"), &values, &mut created_files)?;

    if config.create_csharp {
        let source_dir = TEMPLATE_DIR
            .get_dir("Source")
            .context("Missing 'Source' template directory in embedded assets")?;
        render_dir(source_dir, &target_dir.join("Source"), &values, &mut created_files)?;

        if let Some(gitignore) = TEMPLATE_DIR.get_file(".gitignore") {
            let target = target_dir.join(".gitignore");
            fs::write(&target, gitignore.contents())
                .with_context(|| format!("Failed to write file: {}", target.display()))?;
            created_files.push(target);
        }
    }

    Ok(created_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_template() {
        let template =
            "Name: {projectName}, Author: {author}, ID: {packageId}, Unchanged: {unknown}";
        let rendered = render_template(
            template,
            &[
                ("{projectName}", "AwesomeMod"),
                ("{author}", "RimDev"),
                ("{packageId}", "RimDev.AwesomeMod"),
            ],
        );
        assert_eq!(
            rendered,
            "Name: AwesomeMod, Author: RimDev, ID: RimDev.AwesomeMod, Unchanged: {unknown}"
        );
    }

    #[test]
    fn test_embedded_templates_exist() {
        assert!(TEMPLATE_DIR.get_dir("About").is_some());
        assert!(TEMPLATE_DIR.get_dir("Source").is_some());
        assert!(TEMPLATE_DIR.get_file(".gitignore").is_some());
        assert!(TEMPLATE_DIR.get_file("About/About.xml").is_some());
        assert!(TEMPLATE_DIR.get_file("Source/{projectName}.cs").is_some());
        assert!(
            TEMPLATE_DIR
                .get_file("Source/{projectName}.csproj")
                .is_some()
        );
        assert!(TEMPLATE_DIR.get_file("Source/{projectName}.slnx").is_some());
    }
}
