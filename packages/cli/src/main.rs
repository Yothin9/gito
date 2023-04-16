mod constants;
mod get_upstream;
mod user_command;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gito")]
#[command(about="Git command enhancement CLI", long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "get the upstream remote-url", alias = "gup")]
    GetUpstream {
        #[arg(short = 'n', long = "remote-name", default_value = "upstream")]
        remote_name: String,
    },
    User(UserArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct UserArgs {
    #[command(subcommand)]
    command: UserCmd,
}
#[derive(Debug, Subcommand)]
enum UserCmd {
    #[command(about = "list all git users", alias = "ls")]
    List,
    #[command(about = "add git user")]
    Add {
        /// git user
        name: String,
        /// git email
        email: String,
        /// alias for this account
        alias: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::GetUpstream { remote_name } => {
            get_upstream::run(&remote_name).await;
        }
        Commands::User(user) => match user.command {
            UserCmd::List => {
                println!("in list ");
            }
            UserCmd::Add { name, email, alias } => {
                user_command::add::run(&name, &email, &alias);
            }
        },
    }
}
