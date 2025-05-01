mod lemp;
mod nginx;
mod php;
mod mariadb;
mod phpmyadmin;


#[derive(clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 'p', long)]
    root_password: Option<String>
}



#[derive(clap::Subcommand)]
pub enum Commands {

    #[command(about = "Install nginx")]
    Nginx(nginx::Args),

    #[command(about = "Install phpMyAdmin at port 7070")]
    Phpmyadmin(phpmyadmin::Args),

    #[command(about = "Install PHP")]
    Php(php::Args),

    #[command(about = "Install MariaDb")]
    Mariadb(mariadb::Args)

}



pub fn action(args: Args){

    if let Some(cmd) = args.command{

        match cmd{

            Commands::Nginx(args) => nginx::action(args),
            
            Commands::Phpmyadmin(args) => phpmyadmin::action(args),
    
            Commands::Php(args) => php::action(args),
    
            Commands::Mariadb(args) => mariadb::action(args)
    
        }

    } else{
        
        lemp::action(args);

    }

    
}