use util::{shell_exec, file_put_contents};

pub struct Phpmyadmin;

impl Phpmyadmin{

    
    pub fn install(){

        println!();

        let lastest_phpmyadmin_version = "5.2.2";

        shell_exec(&format!("wget files.phpmyadmin.net/phpMyAdmin/{lastest_phpmyadmin_version}/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip && mv phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip && unzip /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip -d /usr/share && mv /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages /usr/share/phpmyadmin  && rm -rf /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip"));


        file_put_contents("phpmyadmin.conf", &format!(include_str!("../../templates/phpmyadmin/phpmyadmin.conf")));

        shell_exec("mv phpmyadmin.conf /etc/nginx/sites-available/phpmyadmin.conf");

        shell_exec("sudo ln -s /etc/nginx/sites-available/phpmyadmin.conf /etc/nginx/sites-enabled/");

        shell_exec("sudo ufw allow 7070");

        shell_exec("sudo systemctl restart nginx");
        
    }

}