pub fn to_pascal_name(value: &str) -> String {
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
}
