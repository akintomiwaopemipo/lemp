

#[derive(clap::Args)]
pub struct Args;



pub fn action(_args: Args){
    
    println!();

    let lastest_phpmyadmin_version = "5.1.0";

    util::shell_exec(&format!("wget files.phpmyadmin.net/phpMyAdmin/{lastest_phpmyadmin_version}/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip -q && mv phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip && unzip -qq /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip -d /usr/share && mv /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages /usr/share/phpmyadmin  && rm -rf /usr/share/phpMyAdmin-{lastest_phpmyadmin_version}-all-languages.zip",lastest_phpmyadmin_version=lastest_phpmyadmin_version));

    //util::shell_exec("wget api.wpanel.dev/wpanel/phpmyadmin/phpmyadmin.conf -q && mv phpmyadmin.conf /etc/apache2/sites-available/phpmyadmin.conf && rm -rf phpmyadmin.conf && sudo a2ensite phpmyadmin && service apache2 restart");

    util::file_put_contents("phpmyadmin.conf", include_str!("../../templates/phpmyadmin/phpmyadmin.conf"));

    util::shell_exec("mv phpmyadmin.conf /etc/apache2/sites-available/phpmyadmin.conf");

    util::shell_exec("sudo a2ensite phpmyadmin");

    util::shell_exec("sudo systemctl restart apache2");
}



