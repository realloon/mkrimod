use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::config::ModConfig;

const TEMPLATES: &[(&str, &str)] = &[
    ("About/About.xml", include_str!("../template/About/About.xml")),
];

const CSHARP_TEMPLATES: &[(&str, &str)] = &[
    ("Source/{projectName}.cs", include_str!("../template/Source/{projectName}.cs")),
    ("Source/{projectName}.csproj", include_str!("../template/Source/{projectName}.csproj")),
    ("Source/{projectName}.slnx", include_str!("../template/Source/{projectName}.slnx")),
    (".gitignore", include_str!("../template/.gitignore")),
];

pub fn render_template(content: &str, values: &[(&str, &str)]) -> String {
    let mut result = content.to_string();
    for &(k, v) in values {
        result = result.replace(k, v);
    }
    result
}

pub fn scaffold_mod(config: &ModConfig) -> Result<Vec<PathBuf>> {
    let package_id = config.package_id();
    let values = [
        ("{projectName}", config.project_name.as_str()),
        ("{author}", config.author.as_str()),
        ("{packageId}", package_id.as_str()),
    ];

    let files = if config.create_csharp {
        [TEMPLATES, CSHARP_TEMPLATES].concat()
    } else {
        TEMPLATES.to_vec()
    };

    let mut created_files = Vec::with_capacity(files.len());
    for (rel_path, content) in files {
        let path = config.target_dir.join(render_template(rel_path, &values));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        fs::write(&path, render_template(content, &values))
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
        created_files.push(path);
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
}
