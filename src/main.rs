use clap::Parser;
mod args;

fn main() {
    let args = args::Args::parse();

    println!(
        "Your args are: {:?}, {:?}",
        args.directory.unwrap(),
        args.prog
    )
}
