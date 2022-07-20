use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    #[clap(value_parser)]
    pub directory: Option<String>,

    #[clap(short, value_parser, default_value_t = false)]
    pub flag: bool,
}
