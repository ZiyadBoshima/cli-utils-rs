use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments. Enter a message you want to echo, or enter -h for the help menu.");
        process::exit(0);
    } else if args.len() > 2 {
        eprintln!("Too many arguments!");
        process::exit(0);
    }

    if &args[1] == "-h" || &args[1] == "--help" {
        println!(
            "Usage: Use double quotes likeos to echo a message: \"YOUR_MESSAGE\"
             -h or --help to show this help message"
        );
        process::exit(0);
    }

    let message: &String = &args[1];
    println!("{}", message);
}
