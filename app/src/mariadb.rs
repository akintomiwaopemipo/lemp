use util::{file_put_contents, shell_exec};

pub struct Mariadb;

impl Mariadb{

    pub fn install(root_password: &str){

        shell_exec("sudo apt install mariadb-server -y");

        shell_exec("systemctl start mariadb");

        shell_exec("sudo systemctl enable mariadb");

        shell_exec("sudo apt install dos2unix -y");

        shell_exec("sudo apt install expect -y");

        file_put_contents("mysql_secure.sh", include_str!("../../templates/bash/mysql_secure.sh"));

        shell_exec(&format!(r#"dos2unix mysql_secure.sh && chmod +x mysql_secure.sh && ./mysql_secure.sh "{}" && rm -rf mysql_secure.sh"#,root_password));

        shell_exec(&format!(r#"mysql --execute "ALTER USER root@'localhost' IDENTIFIED BY '{}'""#,root_password));


        file_put_contents("/root/.my.cnf",&format!(include_str!("../../templates/mysql/client.cnf"),"root",root_password));

        shell_exec("sudo systemctl enable mariadb.service");

    }

}