

#[derive(clap::Args)]
pub struct UpgradeArgs;


pub fn action(_args: UpgradeArgs){
    util::shell_exec("wget api.wpanel.dev/wpanel/wpanel-ubuntu-20.04 -q && chmod +x wpanel-ubuntu-20.04 && mv wpanel-ubuntu-20.04 /bin/wpanel && wpanel --version");
}



