use app::maintenance_mode::MaintenanceMode;



#[derive(clap::Args)]
pub struct Args{
    
    #[arg(short, long)]
    domain_name: Option<String>,

    #[arg(long)]
    off: bool

}


pub fn action(args: Args){

    let domain_name = args.domain_name.unwrap();

    let maintenance_mode = MaintenanceMode::new(&domain_name);

    if args.off{
        maintenance_mode.off();
    }else{
        maintenance_mode.on();
    }

}



