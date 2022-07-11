use regex::Regex;
use std::fs;
use std::process::{Command, Stdio};

fn main() -> Result<(), std::io::Error> {
    // We use markdown instead of something like toml or json, since I really want to give context
    // behind the rules
    let regex = Regex::new(r"```txt\n([^`]*)```").unwrap();

    let file: String = fs::read_to_string("src/ublock.md")?.parse().unwrap();

    let captures: String = regex
        .captures_iter(&file)
        .flat_map(|cap| {
            // dbg!(&cap);
            cap.get(1).unwrap().as_str().trim().lines()
        })
        .map(|l|l.to_string())
        .filter(|l|&l[..1] != "#")
        .collect::<Vec<String>>()
        .join("\n");

    if cfg!(target_os = "macos") {
        let pbcopy = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()?;
        
        let _echo = Command::new("echo")
            .arg(&captures)
            .stdout(pbcopy.stdin.unwrap())
            .spawn()?;

        // println!("{output:?}");
        // Command::new("pbcopy").spawn().expect("error copying to clipboard");
        println!("successfully copied to clipboard");
    } else {
        println!("not copied to clipboard, OS not supported");
    }

    fs::create_dir_all("out")?;
    fs::write("out/OUTPUT.txt", captures)?;

    Ok(())
}
