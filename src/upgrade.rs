use duma::download::http_download;
use serde_json::Value;
use util::{fetch_url, home_dir, pretty_print_version, shell_exec, shell_exec_as_string};



#[derive(clap::Args)]
pub struct Args;


pub async fn action(_args: Args){
    
    let latest_lemp_release_string = fetch_url("https://api.github.com/repos/akintomiwaopemipo/lemp/releases/latest").await.unwrap();

    let latest_lemp_release = serde_json::from_str::<Value>(&latest_lemp_release_string).unwrap();

    let latest_version = &latest_lemp_release["name"].as_str().unwrap().replace("v", "");

    let current_version = shell_exec_as_string("lemp --version").replace("lemp ", "");

    if *latest_version == current_version{
        println!("lemp is up-to-date");
        return;
    }

    let asset = &latest_lemp_release["assets"][0];

    let release_url = asset["browser_download_url"].as_str().unwrap();

    let asset_name = asset["name"].as_str().unwrap();

    let lemp_installation_dir= format!("{home_dir}/lemp_installation", home_dir = home_dir());

    shell_exec(&format!(r#"rm -rf "{lemp_installation_dir}" && mkdir -p "{lemp_installation_dir}""#));

    http_download(release_url, &format!("{lemp_installation_dir}/{asset_name}")).unwrap();

    shell_exec(&format!(r#"cd "{lemp_installation_dir}" && unzip "{asset_name}" && mv lemp /usr/local/bin && rm -rf "{lemp_installation_dir}" "#));

    pretty_print_version("lemp");

}



