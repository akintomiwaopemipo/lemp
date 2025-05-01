

#[derive(clap::Args)]
pub struct Args{
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    domain: String

}




pub fn action(args: Args){

    let domain_name = args.domain;
    let user = args.user;
    
    let virtual_host = format!(include_str!("../../templates/apache/virtual-host.conf"), domain_name=domain_name, user=user);

    util::file_put_contents(&format!("/etc/apache2/sites-available/{}.conf",domain_name), &virtual_host);

    util::shell_exec(&format!("sudo a2ensite {}.conf",domain_name));

    util::shell_exec("sudo systemctl restart apache2");
    println!("Restarted apache service");
}



