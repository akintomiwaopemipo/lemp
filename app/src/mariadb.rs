use std::path::Path;

use util::{file_put_contents, shell_exec, shell_exec_as_string, user_home_dir};

pub struct Mariadb;

impl Mariadb{


    pub fn configure(user: &str, password: &str, ssh_user: Option<&str>){

        let ssh_user = ssh_user.unwrap_or(user);
        
        shell_exec_as_string(&format!(r#"mysql --execute 'CREATE USER "{user}"@"localhost" IDENTIFIED BY "{password}";'"#));

        shell_exec_as_string(&format!(r#"mysql --execute 'GRANT ALL PRIVILEGES ON `{user}\_%` . * TO "{user}"@"localhost";'"#));

        shell_exec_as_string(&format!(r#"mysql --execute 'GRANT ALL PRIVILEGES ON `{user}` . * TO "{user}"@"localhost"; FLUSH PRIVILEGES;'"#));

        let home_dir = user_home_dir(&ssh_user);
        let mysql_cnf_file = Path::new(&home_dir).join(".my.cnf").into_os_string().into_string().unwrap();

        file_put_contents(&mysql_cnf_file,&format!(include_str!("../../templates/mysql/client.cnf"),user,password));

        shell_exec(&format!("chown {ssh_user} {mysql_cnf_file}"));

        println!("Created MariaDB account for user"); 
    
    }



    pub fn install(root_password: &str){

        shell_exec("sudo apt-get install mariadb-server -y");

        shell_exec("systemctl start mariadb");

        shell_exec("sudo systemctl enable mariadb");

        shell_exec("sudo apt-get install dos2unix -y");

        shell_exec("sudo apt-get install expect -y");

        file_put_contents("mysql_secure.sh", include_str!("../../templates/bash/mysql_secure.sh"));

        shell_exec(&format!(r#"dos2unix mysql_secure.sh && chmod +x mysql_secure.sh && ./mysql_secure.sh "{root_password}" && rm -rf mysql_secure.sh"#));

        shell_exec(&format!(r#"mysql --execute "ALTER USER root@'localhost' IDENTIFIED BY '{root_password}'""#));


        file_put_contents("/root/.my.cnf",&format!(include_str!("../../templates/mysql/client.cnf"),"root",root_password));

        shell_exec("sudo systemctl enable mariadb.service");

    }

}