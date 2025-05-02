use util::{awk_update, command_exists, shell_exec};

pub struct PHP;

impl PHP{

    pub fn configure(){
        
        awk_update("upload_max_filesize", "512M", "/etc/php/8.2/fpm/php.ini", Some(";"), Some(true));

        awk_update("post_max_size", "512M", "/etc/php/8.2/fpm/php.ini", Some(";"), Some(true));

        awk_update("memory_limit", "512M", "/etc/php/8.2/fpm/php.ini", Some(";"), Some(true));
    
        shell_exec("sudo systemctl restart nginx");
        
        println!("Restarted nginx service");

    }


    
    pub fn install(){

        shell_exec("sudo apt install software-properties-common -y && sudo add-apt-repository ppa:ondrej/php -y && sudo add-apt-repository ppa:ondrej/nginx -y && sudo apt-get update && sudo apt install php8.2 -y && sudo apt install php8.2-fpm php8.2-common php8.2-mysql php8.2-xml php8.2-xmlrpc php8.2-curl php8.2-gd php8.2-imagick php8.2-cli php8.2-dev php8.2-imap php8.2-mbstring php8.2-opcache php8.2-soap php8.2-zip php8.2-intl -y && sudo systemctl restart nginx");

        if command_exists("apache2"){
            shell_exec("sudo apt purge apache2 -y");
            shell_exec("rm -f /usr/sbin/apache2");
        }

        shell_exec("php -v");

        Self::configure();
        
    }

}