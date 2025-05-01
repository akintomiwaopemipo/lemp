use app::apache2::Apache2;


#[derive(clap::Args)]
pub struct Args;


pub fn action(_args: Args){

    println!();

    Apache2::install();

    println!();

}



