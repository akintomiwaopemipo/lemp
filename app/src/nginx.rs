use util::{file_put_contents, shell_exec};

pub struct Nginx;

impl Nginx{

    pub fn install(){

        shell_exec("sudo apt-get install nginx -y");

        shell_exec("sudo apt-get install ufw");
        
        shell_exec("sudo ufw allow 'Nginx Full'");

        file_put_contents("ssl-params.conf", include_str!("../../templates/nginx/ssl-params.conf"));

        shell_exec("mv ssl-params.conf /etc/nginx/snippets/ssl-params.conf");

        shell_exec("sudo ufw status");

        shell_exec("sudo ufw allow ssh");

        shell_exec("sudo ufw allow 80");

        shell_exec("sudo ufw allow 443");

        shell_exec("sudo ufw --force enable");

        shell_exec("mkdir -p /etc/ssl/private/ && sudo openssl req -x509 -nodes -days 182500 -newkey rsa:2048 -keyout /etc/ssl/private/nginx-selfsigned.key -out /etc/ssl/certs/nginx-selfsigned.crt -subj '/C=NG/ST=Oyo/L=Ibadan/O=lemp/OU=Org/CN=localhost'");

        shell_exec("sudo apt-get install certbot -y");

        shell_exec("sudo systemctl restart nginx");

    }


    pub fn restart(){
        shell_exec("sudo systemctl restart nginx");
        println!("Restarted nginx service");
    }

}