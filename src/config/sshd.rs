

#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){

    util::awk_update("PasswordAuthentication", "yes", "/etc/ssh/sshd_config", None, None);
    println!();

    util::awk_update("PermitRootLogin", "yes", "/etc/ssh/sshd_config", None, None);
    println!();

    util::awk_update("PubkeyAuthentication", "yes", "/etc/ssh/sshd_config", None, None);
    println!();

    util::shell_exec("systemctl restart ssh");
    println!("Restarted sshd service");
    println!();
    
        
    //util::shell_exec("passwd");

    println!();

}



