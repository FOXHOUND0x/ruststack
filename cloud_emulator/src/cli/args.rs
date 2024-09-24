use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List {
        #[arg(short, long)]
        cloud_provider: String,
    },
    
    Start {
        #[arg(short, long)]
        cloud_provider: String,
        #[arg(short, long)]
        service: String,
    }
}
