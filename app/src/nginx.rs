use std::fs;

use regex::Regex;
use util::{file_exists, file_put_contents, shell_exec};

use crate::maintenance_mode::MaintenanceMode;

pub struct Nginx;

impl Nginx{

    pub fn install(){

        shell_exec("sudo apt-get install nginx -y");

        shell_exec("sudo apt-get install ufw -y");
        
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

        shell_exec("sudo apt-get install python3-certbot-nginx -y");

        shell_exec("sudo systemctl restart nginx");

    }


    pub fn configure(){
        println!("Writing /etc/nginx/conf.d/lemp.conf");
        file_put_contents("/etc/nginx/conf.d/lemp.conf", include_str!("../../templates/nginx/config.conf"));



        println!("Rewriting nginx default config");

        if !file_exists("/etc/nginx/sites-available/default.original"){
            shell_exec("mv /etc/nginx/sites-available/default /etc/nginx/sites-available/default.original");
        }

        file_put_contents("/etc/nginx/sites-available/default", include_str!("../../templates/nginx/default.conf"));

        shell_exec("mkdir -p /usr/share/nginx/html");

        file_put_contents("/usr/share/nginx/html/index.html", include_str!("../../templates/nginx/default-index.html"));

       file_put_contents("/usr/share/nginx/html/phpinfo.php", include_str!("../../templates/nginx/phpinfo.php"));

       MaintenanceMode::create_root();

        Self::restart();
    }


    pub fn restart(){
        println!("Restarting nginx service");
        shell_exec("sudo systemctl restart nginx");
    }



    pub fn reload(){
        println!("Reloading nginx service");
        shell_exec("sudo systemctl reload nginx");
    }


    pub fn enabled_domains() -> Vec<String> {
        let sites_enabled_path = "/etc/nginx/sites-enabled";
        let mut domains = Vec::new();

        // Regex to capture domains from `server_name` line
        let re = Regex::new(r"(?m)^\s*server_name\s+([^;]+);").unwrap();

        if let Ok(entries) = fs::read_dir(sites_enabled_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        for cap in re.captures_iter(&contents) {
                            let names = cap[1]
                                .split_whitespace()
                                .map(|s| s.to_string());
                            domains.extend(names);
                        }
                    }
                }
            }
        }

        let mut lemp_domains = Vec::new();

        for domain in domains{
            
            if 
                !lemp_domains.contains(&domain.replace("www.", "")) && 
                domain != "_" &&
                !domain.contains("phpmyadmin.com")
            {
                lemp_domains.push(domain);
            }
        }

        lemp_domains
        
    }

}