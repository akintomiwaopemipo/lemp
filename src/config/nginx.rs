

#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){
        
    util::add_new_line(include_str!("../../templates/apache/config-append.conf"), "/etc/apache2/apache2.conf");

    util::shell_exec("sudo a2ensite 000-default.conf");

    util::shell_exec("sudo a2enmod rewrite");

    util::shell_exec("sudo systemctl restart apache2");

    println!("Restarted apache service");
}



