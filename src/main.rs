use app::require_sudo;
use clap::{Parser, Subcommand};


mod config;
mod install;
mod maintenance_mode;
mod upgrade;


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

    #[command(about = "Manage maintenance mode for domain")]
    MaintenanceMode(maintenance_mode::Args),

    #[command(about = "Upgrade lemp")]
    Upgrade(upgrade::Args)

}


#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    require_sudo();

    match cli.command{

        Commands::Install(args) => install::action(args),

        Commands::Config(args) => config::action(args.command.unwrap()),

        Commands::MaintenanceMode(args) => maintenance_mode::action(args),

        Commands::Upgrade(args) => upgrade::action(args).await

    }
    

}


