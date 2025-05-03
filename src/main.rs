use app::require_sudo;
use clap::{Parser, Subcommand};


mod upgrade;
mod publish;
mod config;
mod install;


#[derive(Parser)]
#[command(name = "lemp", version, about = "Manage server resources", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {

    #[command(about = "Helper to install major programs")]
    Install(install::Args),

    #[command(about = "Config major settings in sever")]
    Config(config::ConfigArgs),

    #[command(about = "Upgrade lemp")]
    Upgrade(upgrade::UpgradeArgs),

    #[command(about = "Publish lemp")]
    Publish(publish::PublishArgs)

}


#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    require_sudo();

    match cli.command{

        Commands::Install(args) => install::action(args),

        Commands::Config(args) => config::action(args.command.unwrap()),

        Commands::Upgrade(args) => upgrade::action(args),
        
        Commands::Publish(args) => publish::action(args)

    }
    

}


