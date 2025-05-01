use app::phpmyadmin::Phpmyadmin;

#[derive(clap::Args)]
pub struct Args;



pub fn action(_args: Args){
    
    println!();

    Phpmyadmin::install();

    
}



