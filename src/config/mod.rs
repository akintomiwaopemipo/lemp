mod mysql;
mod nginx;
mod php;
mod server_block;


#[derive(clap::Args)]
#[command(arg_required_else_help(true))]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}



#[derive(clap::Subcommand)]
pub enum Commands {

    #[command(about = "Configure mysql settings and restart the mysql service")]
    Mysql(mysql::Args),

    #[command(about = "Configure nginx settings and restart the nginx service")]
    Nginx(nginx::Args),

    #[command(about = "Configure php settings and restart the nginx service")]
    Php(php::Args),

    #[command(about = "Configure server block of a specific user")]
    ServerBlock(server_block::Args)

}



pub fn action(cmd: Commands){
   
    match cmd{

        Commands::Nginx(args) => nginx::action(args),

        Commands::Mysql(args) => mysql::action(args),

        Commands::Php(args) => php::action(args),

        Commands::ServerBlock(args) => server_block::action(args)

    }

}