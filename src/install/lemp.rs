use app::{nginx::Nginx, mariadb::Mariadb, php::PHP, phpmyadmin::Phpmyadmin};
use util::{shell_exec_as_string, read_and_confirm_password, command_exists, directory_exists};
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
    
    if !command_exists("nginx"){
        Nginx::install();
    }else{
        println!("Nginx already installed");
    }
    
    println!();
    println!();
    println!();


    println!("Installing Mariadb");
    println!();

    if !command_exists("mysql"){
        Mariadb::install(&root_password);
    }else{
        println!("Mariadb/MySql already installed");
    }

    println!();
    println!();
    println!();


    println!("Installing PHP");
    println!();
    
    if !command_exists("php"){
        PHP::install();
    }else{
        println!("PHP already installed");
    }

    println!();
    println!();
    println!();


    println!("Installing PhpMyAdmin");
    println!();
    
    if !directory_exists("/usr/share/phpmyadmin"){
        Phpmyadmin::install();
    }else{
        println!("PhpMyAdmin already installed")
    }
    
    println!();
    println!();

}



