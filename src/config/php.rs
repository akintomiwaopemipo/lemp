use app::php::PHP;



#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){
    
    PHP::configure();

}



