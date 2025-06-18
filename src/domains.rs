use app::nginx::Nginx;



#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){
    
    Nginx::enabled_domains()
        .iter()
        .for_each(|domain| println!("{domain}"));

}



