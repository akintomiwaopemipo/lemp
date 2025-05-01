use app::{nginx::Nginx, mariadb::Mariadb, php::PHP, phpmyadmin::Phpmyadmin};
use util::{shell_exec_as_string, read_and_confirm_password};
use crate::install;
use std::process::exit;




pub fn action(args: install::Args){

    let whoami = shell_exec_as_string("whoami");

    if whoami != "root"{
        println!("Permission denied. Please run this command with sudo.");
        exit(-1);
    }

        
    let root_password = args.root_password.unwrap_or_else(|| read_and_confirm_password("MariaDb root password"));

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



