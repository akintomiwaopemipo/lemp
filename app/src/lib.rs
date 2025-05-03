use std::process::exit;

use util::shell_exec_as_string;

pub mod nginx;
pub mod mariadb;
pub mod php;
pub mod phpmyadmin;


pub fn require_sudo(){
    let whoami = shell_exec_as_string("whoami");

    if whoami != "root"{
        println!("Permission denied. Please run this command with sudo.");
        exit(-1);
    }

}