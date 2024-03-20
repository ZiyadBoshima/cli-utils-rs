use clap::Parser;

#[derive(Parser)]
struct Args {
    first_file: std::path::PathBuf,
    second_file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    match &args.second_file {
        Some(second_file) => {
            if args.first_file.is_file() && second_file.is_file() {
                println!(
                    "{}{}",
                    std::fs::read_to_string(&args.first_file).expect("Failed to read first file."),
                    std::fs::read_to_string(second_file).expect("Failed to read second file."),
                )
            }
        }
        None => {
            if args.first_file.is_file() {
                println!(
                    "{}",
                    std::fs::read_to_string(&args.first_file).expect("Failed to read file."),
                )
            } else {
                eprintln!("Path provided is not a file.");
            }
        }
    }
}
