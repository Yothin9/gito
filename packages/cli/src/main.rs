pub mod get_upstream;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gito")]
#[command(about="Git command enhancement CLI", long_about = None, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "get the upstream remote-url", alias = "gup")]
    GetUpstream {
        #[arg(short = 'n', long = "remote-name", default_value = "upstream")]
        remote_name: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::GetUpstream { remote_name } => {
            get_upstream::run(&remote_name).await;
        }
    }
}
