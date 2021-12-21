use std::fs;
use toml::Value;

fn main() -> Result<(), std::io::Error> {
    // for entry in fs::read_dir(".")?.flatten() {
    //     dbg!(entry.file_name());
    // }

    let categories: Value = fs::read_to_string("src/blocking.toml")?.parse().unwrap();

    let websites = categories.as_table().unwrap().get("websites").unwrap();

    let mut total_rules = Vec::new();
    for rules in websites.as_table().unwrap().values() {
        for rule in rules.as_array().unwrap() {
            total_rules.push(rule.as_str().unwrap());
        }
    }

    fs::create_dir_all("out")?;
    fs::write("out/OUTPUT.txt", total_rules.join("\n"))?;

    Ok(())
}
