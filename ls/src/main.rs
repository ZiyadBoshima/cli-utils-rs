use colored::Colorize;
use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &Path = &Path::new(match args.len() < 2 {
        true => ".",
        false => &args[1],
    });
    let path_str: &str = path.to_str().unwrap();

    let entries: Vec<_> = path
        .read_dir()
        .expect("read_dir failed")
        .map(|entry| {
            let entry_path = entry.unwrap().path();
            let entry_path_str = entry_path
                .strip_prefix(path_str)
                .unwrap()
                .display()
                .to_string();
            entry_path_str
        })
        .collect();

    //let max_length = entries.iter().map(|entry| entry.len()).max().unwrap_or(0);

    for entry_path_str in entries {
        println!("{}", entry_path_str.blue());
    }
}
