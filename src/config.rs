use std::path::PathBuf;

fn to_pascal_name(value: &str) -> String {
    let mut result = String::new();
    for word in value
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
    {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.extend(first.to_uppercase());
            result.push_str(chars.as_str());
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModConfig {
    pub project_name: String,
    pub author: String,
    pub create_csharp: bool,
    pub target_dir: PathBuf,
}

impl ModConfig {
    pub const DEFAULT_PROJECT_NAME: &'static str = "MyMod";
    pub const DEFAULT_AUTHOR: &'static str = "Author";

    pub fn new(
        raw_project_name: &str,
        raw_author: &str,
        create_csharp: bool,
        custom_output: Option<PathBuf>,
    ) -> Self {
        let pascal = to_pascal_name(raw_project_name);
        let project_name = if pascal.is_empty() {
            Self::DEFAULT_PROJECT_NAME.to_string()
        } else {
            pascal
        };

        let trimmed = raw_author.trim();
        let author = if trimmed.is_empty() {
            Self::DEFAULT_AUTHOR
        } else {
            trimmed
        }
        .to_string();

        let target_dir = custom_output.unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_default()
                .join(&project_name)
        });

        Self {
            project_name,
            author,
            create_csharp,
            target_dir,
        }
    }

    pub fn package_id(&self) -> String {
        format!("{}.{}", self.author, self.project_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_name() {
        assert_eq!(to_pascal_name("MyMod"), "MyMod");
        assert_eq!(to_pascal_name("my-mod"), "MyMod");
        assert_eq!(to_pascal_name("my_cool_mod"), "MyCoolMod");
        assert_eq!(to_pascal_name("  rimworld   mod  "), "RimworldMod");
        assert_eq!(to_pascal_name("RimWorld - 1.6 Mod!"), "RimWorld16Mod");
        assert_eq!(to_pascal_name("helloWorld"), "HelloWorld");
        assert_eq!(to_pascal_name(""), "");
        assert_eq!(to_pascal_name("---"), "");
        assert_eq!(to_pascal_name("123abc"), "123abc");
    }

    #[test]
    fn test_mod_config_defaults() {
        let config = ModConfig::new("", "", true, None);
        assert_eq!(config.project_name, "MyMod");
        assert_eq!(config.author, "Author");
        assert_eq!(config.package_id(), "Author.MyMod");
        assert!(config.create_csharp);
    }

    #[test]
    fn test_mod_config_custom() {
        let config = ModConfig::new(
            "awesome-mod",
            " Alice ",
            false,
            Some(PathBuf::from("/tmp/mod")),
        );
        assert_eq!(config.project_name, "AwesomeMod");
        assert_eq!(config.author, "Alice");
        assert_eq!(config.package_id(), "Alice.AwesomeMod");
        assert!(!config.create_csharp);
        assert_eq!(config.target_dir, PathBuf::from("/tmp/mod"));
    }
}
