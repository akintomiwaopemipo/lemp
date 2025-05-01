


#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){        
    //let mysqld_path = "/etc/mysql/mysql.conf.d/mysqld.cnf";
    let mysqld_path = "/etc/mysql/conf.d";

    println!("Adding mariadb configurations");

    util::file_put_contents("my.cnf", include_str!("../../templates/mysql/my.cnf"));
    util::shell_exec(&format!("cp my.cnf {} && rm -f my.cnf",mysqld_path));

    util::shell_exec("sudo systemctl enable mariadb && systemctl restart mariadb");

    println!("Restarted mysql service");
}



