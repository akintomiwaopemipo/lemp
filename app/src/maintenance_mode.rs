use std::process::exit;

use util::{file_exists, file_put_contents, shell_exec};

use crate::nginx::Nginx;

pub struct MaintenanceMode{
    pub domain_name: String
}

impl MaintenanceMode{

    pub fn new(domain_name: &str) -> Self{
        Self {
            domain_name: domain_name.to_string()
        }
    }


    pub fn config_file(&self) -> String{
        let domain_name = &self.domain_name;
        format!("/etc/nginx/sites-available/{domain_name}.conf")
    }


    pub fn config_backup_file(&self) -> String{
        let domain_name = &self.domain_name;
        shell_exec("mkdir -p /etc/nginx/maintenance-mode");
        format!("/etc/nginx/maintenance-mode/{domain_name}.conf")
    }



    pub fn on(&self){
        let domain_name = &self.domain_name;
        let config_file = self.config_file();
        let config_backup_file = self.config_backup_file();

        if !file_exists(&config_backup_file){
            shell_exec(&format!("mv '{config_file}' '{config_backup_file}'"));
        }else{
            println!("Backup file {config_backup_file} already exists");
            exit(1);
        }

        let ssl_certificate = format!("/etc/letsencrypt/live/{domain_name}/fullchain.pem");
        let ssl_certificate_key = format!("/etc/letsencrypt/live/{domain_name}/privkey.pem");

        let ssl_config = if file_exists(&ssl_certificate) && file_exists(&ssl_certificate_key){
            format!("listen 443 ssl;\n\tssl_certificate {ssl_certificate};\n\tssl_certificate_key {ssl_certificate_key};")
        }else{
            "".to_string()
        };

        let maintenance_mode_config = include_str!("../../templates/maintenance-mode/nginx-server.conf").replace("ssl_config", &ssl_config).replace("domain_name", domain_name);

        file_put_contents(&config_file, &maintenance_mode_config);

        println!("Turned on maintenance mode for {domain_name}");

        Nginx::restart();

    }



    pub fn off(&self){
        let domain_name = &self.domain_name;
        let config_file = self.config_file();
        let config_backup_file = self.config_backup_file();

        if !file_exists(&config_backup_file){
            println!("Maintenance mode backup file does not exists");
            exit(1);
        }

        shell_exec(&format!("mv '{config_backup_file}' '{config_file}'"));

        shell_exec(&format!("rm -f '{config_backup_file}'"));

        println!("Turned off maintenance mode for {domain_name}");

        Nginx::restart();        

    }


    pub fn create_root(){
        shell_exec("mkdir -p /usr/share/nginx/maintenance-mode");
        file_put_contents("/usr/share/nginx/maintenance-mode/index.html", include_str!("../../templates/maintenance-mode/index.html"));
    }

}