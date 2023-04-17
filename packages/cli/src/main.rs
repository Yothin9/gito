mod constants;
mod utils;
//  command
mod amend_command;
mod get_upstream;
mod init_command;
mod user_command;
// extern
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
    #[command(about = "amend the commit's author and email by alias")]
    Amend {
        alias: String,
    },
    #[command(about = "git init with specific user info by alias")]
    Init {
        alias: String,
    },
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
    #[command(about = "change local git user by alias")]
    Use {
        /// alias
        alias: String,
        #[arg(
            short = 'g',
            default_value = "false",
            help = "set git user locally and globally"
        )]
        global: bool,
    },
    #[command(about = "add git user")]
    Add {
        /// alias for this account
        alias: String,
        /// git user
        name: String,
        /// git email
        email: String,
    },
    #[command(about = "delete git user by alias")]
    Del {
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
                user_command::list::run();
            }
            UserCmd::Add { alias, name, email } => {
                user_command::add::run(&alias, &name, &email);
            }
            UserCmd::Use { alias, global } => user_command::use_user::run(&alias, global),
            UserCmd::Del { alias } => user_command::del::run(&alias),
        },
        Commands::Amend { alias } => amend_command::run(&alias),
        Commands::Init { alias } => init_command::run(&alias),
    }
}
