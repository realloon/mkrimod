use mkrimod::config::ModConfig;
use mkrimod::naming::to_pascal_name;
use mkrimod::template::scaffold_mod;
use std::fs;
use std::path::PathBuf;

fn get_test_dir(test_name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rimod_test_{}_{}", test_name, std::process::id()));
    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_scaffold_with_csharp() {
    let test_dir = get_test_dir("with_csharp");
    let target = test_dir.join("MySuperMod");

    let config = ModConfig::new("my-super-mod", "Loon", true, Some(target.clone()));
    let files = scaffold_mod(&config).unwrap();

    assert_eq!(config.project_name, "MySuperMod");
    assert_eq!(config.author, "Loon");
    assert_eq!(config.package_id(), "Loon.MySuperMod");

    let about_path = target.join("About").join("About.xml");
    assert!(about_path.exists());
    let about_content = fs::read_to_string(&about_path).unwrap();
    assert!(about_content.contains("<packageId>Loon.MySuperMod</packageId>"));
    assert!(about_content.contains("<name>MySuperMod</name>"));
    assert!(about_content.contains("<author>Loon</author>"));

    let cs_path = target.join("Source").join("MySuperMod.cs");
    assert!(cs_path.exists());
    let cs_content = fs::read_to_string(&cs_path).unwrap();
    assert!(cs_content.contains("namespace MySuperMod;"));
    assert!(cs_content.contains("public class MySuperMod"));

    let csproj_path = target.join("Source").join("MySuperMod.csproj");
    assert!(csproj_path.exists());

    let slnx_path = target.join("Source").join("MySuperMod.slnx");
    assert!(slnx_path.exists());
    let slnx_content = fs::read_to_string(&slnx_path).unwrap();
    assert!(slnx_content.contains(r#"<Project Path="MySuperMod.csproj" />"#));

    let gitignore_path = target.join(".gitignore");
    assert!(gitignore_path.exists());
    let gitignore_content = fs::read_to_string(&gitignore_path).unwrap();
    assert!(gitignore_content.contains("Assemblies/"));

    assert_eq!(files.len(), 5);

    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_scaffold_without_csharp() {
    let test_dir = get_test_dir("no_csharp");
    let target = test_dir.join("SimpleXmlMod");

    let config = ModConfig::new("simple_xml_mod", "Alice", false, Some(target.clone()));
    let files = scaffold_mod(&config).unwrap();

    assert_eq!(config.project_name, "SimpleXmlMod");
    assert_eq!(config.author, "Alice");
    assert_eq!(config.package_id(), "Alice.SimpleXmlMod");

    let about_path = target.join("About").join("About.xml");
    assert!(about_path.exists());
    let about_content = fs::read_to_string(&about_path).unwrap();
    assert!(about_content.contains("<packageId>Alice.SimpleXmlMod</packageId>"));
    assert!(about_content.contains("<name>SimpleXmlMod</name>"));
    assert!(about_content.contains("<author>Alice</author>"));

    assert!(!target.join("Source").exists());
    assert!(!target.join(".gitignore").exists());

    assert_eq!(files.len(), 1);

    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_pascal_name_parity_with_ts() {
    assert_eq!(to_pascal_name("myMod"), "MyMod");
    assert_eq!(to_pascal_name("create-rimworld-mod"), "CreateRimworldMod");
    assert_eq!(to_pascal_name("rim_world_core"), "RimWorldCore");
    assert_eq!(to_pascal_name("test 123 foo"), "Test123Foo");
}
