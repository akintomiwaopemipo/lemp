use util::shell_exec;

pub struct Nginx;

impl Nginx{

    pub fn install(){

        shell_exec("sudo apt install nginx -y");

        shell_exec("sudo apt install ufw");
        
        shell_exec("sudo ufw allow 'Nginx HTTP'");

        shell_exec("sudo ufw status");

        shell_exec("sudo ufw allow ssh");

        shell_exec("sudo ufw allow 80");

        shell_exec("sudo ufw allow 443");

        shell_exec("sudo ufw --force enable");

        shell_exec("sudo systemctl reload nginx");

    }

}