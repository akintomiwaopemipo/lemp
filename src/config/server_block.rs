

#[derive(clap::Args)]
pub struct Args{
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    domain: String

}




pub fn action(args: Args){

    let domain_name = args.domain;
    let _user = args.user;
    
    let server_block = format!(include_str!("../../templates/nginx/server-block.conf"), domain_name=domain_name);

    util::file_put_contents(&format!("/etc/nginx/sites-available/{domain_name}.conf"), &server_block);

    util::shell_exec(&format!("sudo ln -s /etc/nginx/sites-available/{domain_name} /etc/nginx/sites-enabled/"));

    util::shell_exec("sudo systemctl restart nginx");
    println!("Restarted nginx service");
}



