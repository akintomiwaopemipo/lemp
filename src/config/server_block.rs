use util::{file_put_contents, shell_exec};

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

    let php_fpm_pool_conf = format!(include_str!("../../templates/nginx/php-fpm-pool.conf"), user=user,domain_name=domain_name); 

    file_put_contents(&format!("/etc/php/8.2/fpm/pool.d/{domain_name}.conf"), &php_fpm_pool_conf);
    
    let server_block = format!(include_str!("../../templates/nginx/server-block.conf"), domain_name=domain_name);

    file_put_contents(&format!("/etc/nginx/sites-available/{domain_name}.conf"), &server_block);

    shell_exec(&format!("sudo ln -s /etc/nginx/sites-available/{domain_name}.conf /etc/nginx/sites-enabled/"));

    shell_exec(&format!(r#"mkdir -p "/var/www/{domain_name}" && chown -R {user}:{user} "/var/www/{domain_name}""#));

    shell_exec("sudo systemctl restart php8.2-fpm");
    println!("Restarted php8.2-fpm service");

    shell_exec("sudo systemctl restart nginx");
    println!("Restarted nginx service");
}



