use clap::Parser;
mod args;
pub mod date;

fn main() {
    let args = args::Args::parse();

    println!(
        "Your args are: {:?}, {:?}",
        args.directory.unwrap(),
        args.prog
    )
}
