#[macro_use]
extern crate clap;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("value of verbose: {}", opts.verbose);
}
