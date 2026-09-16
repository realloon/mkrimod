use mkrimod::config::ModConfig;
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

    let about = fs::read_to_string(target.join("About").join("About.xml")).unwrap();
    assert!(about.contains("<packageId>Loon.MySuperMod</packageId>"));
    assert!(about.contains("<name>MySuperMod</name>"));
    assert!(about.contains("<author>Loon</author>"));

    let cs = fs::read_to_string(target.join("Source").join("MySuperMod.cs")).unwrap();
    assert!(cs.contains("namespace MySuperMod;"));
    assert!(cs.contains("public class MySuperMod"));

    assert!(target.join("Source").join("MySuperMod.csproj").exists());

    let slnx = fs::read_to_string(target.join("Source").join("MySuperMod.slnx")).unwrap();
    assert!(slnx.contains(r#"<Project Path="MySuperMod.csproj" />"#));

    let gitignore = fs::read_to_string(target.join(".gitignore")).unwrap();
    assert!(gitignore.contains("Assemblies/"));

    assert_eq!(files.len(), 5);

    let _ = fs::remove_dir_all(&test_dir);
}

#[test]
fn test_scaffold_without_csharp() {
    let test_dir = get_test_dir("no_csharp");
    let target = test_dir.join("SimpleXmlMod");

    let config = ModConfig::new("simple_xml_mod", "Alice", false, Some(target.clone()));
    let files = scaffold_mod(&config).unwrap();

    let about = fs::read_to_string(target.join("About").join("About.xml")).unwrap();
    assert!(about.contains("<packageId>Alice.SimpleXmlMod</packageId>"));
    assert!(about.contains("<name>SimpleXmlMod</name>"));
    assert!(about.contains("<author>Alice</author>"));

    assert!(!target.join("Source").exists());
    assert!(!target.join(".gitignore").exists());

    assert_eq!(files.len(), 1);

    let _ = fs::remove_dir_all(&test_dir);
}
