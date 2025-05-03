use app::{mariadb::Mariadb, nginx::Nginx};
use util::{change_unix_user_password, file_put_contents, read_and_confirm_password, shell_exec};

#[derive(clap::Args)]
pub struct Args{
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    password: Option<String>,

    #[arg(short, long)]
    domain: String,

    #[arg(long)]
    add_unix_user: bool,

    #[arg(long)]
    use_certbot: bool

}




pub fn action(args: Args){

    let domain_name = args.domain;
    let user = args.user;

    if args.add_unix_user{
        
        let password = if args.password.is_some(){
            args.password.unwrap()
        }else{
            read_and_confirm_password(&format!("Password for {user}"))
        };

        println!();

        shell_exec(&format!(r#"adduser --gecos "" --disabled-password {user}"#));

        change_unix_user_password(&user, &password);

        println!("Created user account: {user}");

        println!();

        shell_exec(&format!("chmod +x /home/{user}"));

        println!();

        shell_exec(&format!("mkdir -p /home/{user}/public_html"));

        file_put_contents(&format!("/home/{user}/public_html/error.log"),"");

        Mariadb::configure(&user, &password, None);

        println!();

    }



    let php_fpm_pool_conf = format!(include_str!("../../templates/nginx/php-fpm-pool.conf"), user=user,domain_name=domain_name); 

    file_put_contents(&format!("/etc/php/8.2/fpm/pool.d/{domain_name}.conf"), &php_fpm_pool_conf);

    let root_directory = if args.add_unix_user{
        format!("/home/{user}/public_html")
    }else{
        format!("/var/www/{domain_name}")
    };
    
    let server_block = format!(include_str!("../../templates/nginx/server-block.conf"), domain_name=domain_name, root_directory=root_directory);

    file_put_contents(&format!("/etc/nginx/sites-available/{domain_name}.conf"), &server_block);

    shell_exec(&format!("sudo ln -s /etc/nginx/sites-available/{domain_name}.conf /etc/nginx/sites-enabled/"));

    shell_exec(&format!(r#"mkdir -p "{root_directory}" && chown -R {user}:{user} "{root_directory}""#));

    shell_exec("sudo systemctl restart php8.2-fpm");
    println!("Restarted php8.2-fpm service");

    Nginx::restart();


    if args.use_certbot{
        shell_exec(&format!(r#"certbot -d "{domain_name}" --nginx --non-interactive --agree-tos --register-unsafely-without-email --no-redirect"#));

        Nginx::restart();
    }

}



