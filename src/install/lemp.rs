use app::{nginx::Nginx, mariadb::Mariadb, php::PHP, phpmyadmin::Phpmyadmin};
use util::read_and_confirm_password;
use crate::install;




pub fn action(args: install::Args){
        
    let root_password = args.root_password.unwrap_or_else(|| read_and_confirm_password());

    println!();
    
    println!("Installing nginx");
    println!();
    Nginx::install();
    println!();
    println!();
    println!();


    println!("Installing Mariadb");
    println!();
    Mariadb::install(&root_password);
    println!();
    println!();
    println!();


    println!("Installing PHP");
    println!();
    PHP::install();
    println!();
    println!();
    println!();


    println!("Installing PhpMyAdmin");
    println!();
    Phpmyadmin::install();
    println!();
    println!();

}



