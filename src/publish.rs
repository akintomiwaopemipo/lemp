
use std::process;
use std::path::Path;
use std::env;


#[derive(clap::Args)]
pub struct PublishArgs;


pub fn action(_args: PublishArgs){
        
    let current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let application_name = Path::new(&current_dir).file_name().unwrap().to_str().unwrap();
    let source: String;
    let destination: String;

    if cfg!(target_os = "windows") {
    
        source = format!(r#"{}\target\release\{}.exe"#,current_dir,application_name);
        destination = format!(r#"{}\..\..\bin\{}.exe"#,current_dir,application_name); 

        process::Command::new("cmd")
                .args(&["/C", "copy", &source, &destination, "/Y" ])
                .output()
                .expect("failed to execute process");
    } else {
        source = format!(r#"{}/target/release/{}"#,current_dir,application_name);
        destination = format!(r#"/usr/local/bin/{}"#,application_name);

        // Command::new("rm")
        //         .arg(&destination)
        //         .output()
        //         .expect("failed to execute process");

        util::shell_exec(&format!("cp {} {}",source,destination));
        //util::shell_exec(&format!("cp {} wpanel",source));
    };
    
    println!("{} -> {}, {} published",source,destination,application_name);

    println!();

    // let system = json::parse(&util::file_get_contents("system.json")).unwrap();

    // let os_and_version = format!("{}-{}",&system["os"],&system["version"]);

    // let mut easy = Easy::new();
    // easy.url("https://api.wpanel.dev/publish-wpanel").unwrap();

    // let mut form = Form::new();

    // form.part("os_and_version").contents(os_and_version.as_bytes()).add().unwrap();

    // form.part("wpanel").file(&source).add().unwrap();


    // easy.httppost(form).unwrap();

    // let mut transfer = easy.transfer();

    // transfer.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     println!();
    //     Ok(data.len())
    // }).unwrap();

    // transfer.perform().unwrap();
}



