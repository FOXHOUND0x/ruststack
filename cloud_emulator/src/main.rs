mod cli;
mod emulator;
mod utils;

use clap::Parser;
use cli::args::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::args::Commands::List { cloud_provider } => {
            cli::cmd::list::execute(cloud_provider);
        },
        cli::args::Commands::Start { cloud_provider, service } => {
            cli::cmd::start::execute(cloud_provider, service);
        },
    }
}
