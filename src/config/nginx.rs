

#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){
        
    util::add_new_line(include_str!("../../templates/nginx/config-append.conf"), "/etc/nginx/nginx.conf");

    util::shell_exec("sudo systemctl restart nginx");

    println!("Restarted apache service");
}



