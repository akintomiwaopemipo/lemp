use util::{awk_update, shell_exec};

pub struct PHP;

impl PHP{

    pub fn configure(){
        
        awk_update("upload_max_filesize", "512M", "/etc/php/8.2/apache2/php.ini", Some(";"), Some(true));

        awk_update("post_max_size", "512M", "/etc/php/8.2/apache2/php.ini", Some(";"), Some(true));

        awk_update("memory_limit", "512M", "/etc/php/8.2/apache2/php.ini", Some(";"), Some(true));
    
        shell_exec("sudo systemctl restart apache2");
        
        println!("Restarted apache service");

    }


    
    pub fn install(){

        shell_exec("sudo apt install software-properties-common -y && sudo add-apt-repository ppa:ondrej/php -y && sudo apt-get update && sudo apt install php8.2 -y && sudo apt install php8.2-common php8.2-mysql php8.2-xml php8.2-xmlrpc php8.2-curl php8.2-gd php8.2-imagick php8.2-cli php8.2-dev php8.2-imap php8.2-mbstring php8.2-opcache php8.2-soap php8.2-zip php8.2-intl -y && sudo a2enmod php8.2 && sudo service apache2 restart");

        shell_exec("php -v");

        Self::configure();
        
    }

}