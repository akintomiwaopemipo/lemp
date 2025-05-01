mod sshd;
mod mysql;
mod nginx;
mod php;
mod server_block;


#[derive(clap::Args)]
#[command(arg_required_else_help(true))]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: Option<ConfigCommands>,
}



#[derive(clap::Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Configure sshd settings and restart the sshd service")]
    Sshd(sshd::Args),

    #[command(about = "Configure mysql settings and restart the mysql service")]
    Mysql(mysql::Args),

    #[command(about = "Configure apache settings and restart the apache service")]
    Apache(nginx::Args),

    #[command(about = "Configure php settings and restart the apache service")]
    Php(php::Args),

    #[command(about = "Configure virtual host of a specific user")]
    VirtualHost(server_block::Args),

    #[command(about = "Configure confirm-transactions service")]
    ConfirmTransactions(confirm_transactions::Args)

}



pub fn action(cmd: ConfigCommands){
   
    match cmd{

        ConfigCommands::Sshd(args) => sshd::action(args),

        ConfigCommands::Mysql(args) => mysql::action(args),

        ConfigCommands::Apache(args) => nginx::action(args),

        ConfigCommands::Php(args) => php::action(args),

        ConfigCommands::VirtualHost(args) => server_block::action(args),

        ConfigCommands::ConfirmTransactions(args) => confirm_transactions::action(args)

    }

}