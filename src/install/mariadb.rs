use app::mariadb::Mariadb;
use util::read_and_confirm_password;



#[derive(clap::Args)]
pub struct Args{
    #[arg(short = 'p', long)]
    root_password: Option<String>
}


pub fn action(args: Args){
        
    let root_password = args.root_password.unwrap_or_else(|| read_and_confirm_password());

    println!();

    Mariadb::install(&root_password);

    println!();

}



