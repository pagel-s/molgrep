use clap::Parser;
use std::process;

fn main() {
    let args = molgrep::Args::parse();

    if let Err(e) = molgrep::find_mol(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
