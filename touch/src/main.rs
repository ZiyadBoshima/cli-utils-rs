use clap::Parser;
use std::fs::File;

#[derive(Parser)]
struct Args {
    filename: String,
}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    File::create(args.filename)?;
    Ok(())
}
