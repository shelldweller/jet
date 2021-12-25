mod app;
mod cli;
mod enums;
mod jsonpath;
mod readers;
mod writers;

use structopt::StructOpt;

fn main() {
    let args = cli::Opt::from_args();
    app::run_jet(args);
}
