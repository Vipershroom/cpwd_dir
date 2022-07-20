use clap::Parser;
mod args;
mod date;
mod file;

fn main() {
    let args = args::Args::parse();

    let friday = date::friday_of_week();
    file::rename_file(friday, args.directory.unwrap(), args.flag).unwrap();
    println!("Success")
}
