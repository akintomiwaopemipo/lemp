use std::process::exit;

use app::{maintenance_mode::MaintenanceMode, nginx::Nginx};
use util::{print_header_block, stdin};



#[derive(clap::Args)]
pub struct Args{
    
    #[arg(short, long)]
    domain_name: Option<String>,

    #[arg(short, long)]
    all: bool,

    #[arg(long)]
    off: bool

}


pub fn action(args: Args){

    if args.all && args.domain_name.is_some(){
        println!("Domain name and all options passed together");
        exit(1);
    }

    if args.all{
        
        for domain_name in Nginx::enabled_domains(){

            print_header_block(&format!("{domain_name}"));

            let maintenance_mode = MaintenanceMode::new(&domain_name);
        
            if args.off{
                maintenance_mode.off();
            }else{
                maintenance_mode.on();
            }

        }

        println!();

    }else{
        
        let domain_name = args.domain_name.unwrap_or_else(|| stdin("Domain name: "));

        if !Nginx::enabled_domains().contains(&domain_name){
            println!("{domain_name} not an enabled domain name");
            exit(1);
        }

        let maintenance_mode = MaintenanceMode::new(&domain_name);
        
        if args.off{
            maintenance_mode.off();
        }else{
            maintenance_mode.on();
        }

    }

}



