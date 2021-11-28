mod app;
mod cli;
mod enums;
mod readers;

use structopt::StructOpt;

fn main() {
    let args = cli::Opt::from_args();
    app::run_jet(args);
}
