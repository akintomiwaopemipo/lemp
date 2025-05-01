use app::{nginx::Nginx, mariadb::Mariadb, php::PHP};
use util::read_and_confirm_password;
use crate::install;




pub fn action(args: install::Args){
        
    let root_password = args.root_password.unwrap_or_else(|| read_and_confirm_password());

    println!();
    
    Nginx::install();

    Mariadb::install(&root_password);

    PHP::install();

    println!();

}



