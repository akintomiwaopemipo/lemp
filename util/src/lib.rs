use std::fs;
use std::iter::FromIterator;
use reqwest::header;
use serde_json::Value;
use serde::Serialize;
use im;
use std::env;
use std::path::{ Path, PathBuf };
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use indexmap::*;
use json::*;
use sha2::{Sha256, Digest};
//use glob::glob;
use colored::Colorize;
use subprocess::Exec;
use std::process::{exit, Command};
use rustyline::{ Editor, error::ReadlineError };
use dirs;
use tinytemplate::TinyTemplate;




pub fn _t(_str: &str)->String{
    String::from(_str)
}


pub fn _th(hashmap: im::HashMap<&str,&str>)->im::HashMap<String,String>{
    let mut h = im::HashMap::new();

    for (k,v) in hashmap{
        h.insert(_t(k), _t(v));
    }
    h
}


pub fn _tv(vec: Vec<&str>)->Vec<String>{
    let mut _vec = vec![];

    for t in vec{
        _vec.push(_t(t));
    }
    _vec
}


pub fn file_get_contents(path: &str) -> String{
    fs::read_to_string(path).expect("An error occcured")
}


pub fn file_put_contents(path: &str, content: &str){
    let mut file = fs::File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}



pub fn json_encode(json: &Value)->String{
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();
    String::from_utf8(ser.into_inner()).unwrap()
}


pub fn json_decode(json_string: &str)->Value{
    serde_json::from_str(json_string).expect("Decoding failed")
}


pub fn hashmap_keys(hashmap: &im::HashMap<String,String>)->Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    for (key,_value) in hashmap{
        vec.push(String::from(key));
    }
    vec
}


pub fn hashmap_values(hashmap: &im::HashMap<String,String>)->Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    for (_key,value) in hashmap{
        vec.push(_t(value));
    }
    vec
}




pub fn indexmap_keys(indexmap: &IndexMap<&str,&str>)->Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    for (key,_value) in indexmap{
        vec.push(_t(key));
    }
    vec
}


pub fn indexmap_values(indexmap: &IndexMap<&str,&str>)->Vec<String>{
    let mut vec: Vec<String> = Vec::new();
    for (_key,value) in indexmap{
        vec.push(_t(value));
    }
    vec
}



pub fn vec_merge(vec1: &Vec<String>,vec2: &Vec<String>)->Vec<String>{
    let mut vec = vec1.to_owned();
    for i in vec2{
        vec.push(String::from(i));
    }
    vec
}


pub fn vec_multiply(vec: Vec<String>)
->im::HashMap<String,String>{
    let mut hashmap = im::HashMap::new();
    for t in vec{
        hashmap.insert(t.clone(), t.clone());
    }
    hashmap
}


pub fn serde_to_hashmap(serde: &serde_json::Value)->im::HashMap<String,String>{
    let mut h = im::HashMap::new();

    for (k,v) in serde.as_object().unwrap(){
        if v.as_str().is_some(){
            h.insert(String::from(k.as_str()), String::from(v.as_str().unwrap()));
        }
    }
    h
}


pub async fn fetch_url(url: &str)-> eyre::Result<String>{

    let mut headers = header::HeaderMap::new();
        
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0"));

    
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let request = client.request(reqwest::Method::GET, url);

    let response = request.send().await?;
    Ok(response.text().await?)
}





pub fn file_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);

    if metadata.is_ok(){
        metadata.unwrap().is_file()
    }else{
        false
    }
}


pub fn directory_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);

    if metadata.is_ok(){
        metadata.unwrap().is_dir()
    }else{
        false
    }
}


pub fn current_dir()->String{
    env::current_dir().unwrap().into_os_string().into_string().unwrap()
}


pub fn dirname(path_str: &str)->String{
    let path = Path::new(path_str);
    _t(path.parent().unwrap().to_str().unwrap())
}



pub fn mkdir(path: &str){
    std::fs::create_dir_all(path).expect("Error occured while creating directory");
}


pub fn document_root()->String{
    let mut _dirname = current_dir();

    loop{
        if file_exists(&format!(r#"{}/settings.json"#,&_dirname)){
            return _dirname;
        }else{
            _dirname = dirname(&_dirname);
        }
    }
}



pub fn random_varchar(charset: &[u8], length: usize)->String{
    let mut rng = rand::thread_rng();

    (0..length).map(|_| {
            let idx = rng.gen_range(0, charset.len());
            charset[idx] as char
        })
        .collect()
    
}



pub fn random_digits(length: usize)->String{
    random_varchar(b"0123456789", length)
    
}


pub fn random_characters(length: usize)->String{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}



pub fn unique_from_vec(vec: Vec<String>, length: usize, context: &str)->String{
    let  mut content: String;
    loop{
        if context=="digits"{
            content = random_digits(length)
        }else{
            content = random_characters(length);
        }

        
        if !vec.contains(&content){
            return content;
        }
    }
    
}



pub fn unique_digits_from_vec(vec: Vec<String>, length: usize)->String{
    unique_from_vec(vec, length, "digits")
}


pub fn unique_characters_from_vec(vec: Vec<String>, length: usize)->String{
    unique_from_vec(vec, length, "characters")
}



pub fn unique_from_fs(directory_path: &str, length: usize, context: &str)->String{
    let  mut content: String;
    loop{
        if context=="digits"{
            content = random_digits(length)
        }else{
            content = random_characters(length);
        }

        
        if !directory_exists(&format!("{}/{}",directory_path,content)){
            return content;
        }
    }
    
}



pub fn unique_digits_from_fs(directory_path: &str, length: usize)->String{
    unique_from_fs(directory_path, length, "digits")
}



pub fn unique_characters_from_fs(directory_path: &str, length: usize)->String{
    unique_from_fs(directory_path, length, "characters")
}


pub fn timestamp()->u64{
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}


// pub fn stdin(title: &str)->String{
//     let mut input = String::new();
//     print!("{}",title.bright_cyan());
//     std::io::stdout().flush().unwrap();
//     std::io::stdin().read_line(&mut input).unwrap();
//     std::io::stdout().flush().unwrap();
//     let output = input.replace("\r\n", "");
//     _t(output.trim())

// }





pub fn stdin(title: &str)->String{
        
    let mut rl = Editor::<()>::new();
    let history_file = Path::new(&home_dir()).join(".history.txt");
    if rl.load_history(&history_file).is_err() {
        //println!("No previous history.");
    }
    let mut input: String = _t("");
    loop {
        let readline = rl.readline(&format!("{}",title.bright_magenta()));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                input = line;
                rl.save_history(&history_file).unwrap();
                break
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                std::process::exit(-1);
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(&history_file).unwrap();
    input
}

pub fn get_info(info_title: &str)->String{
    stdin(&format!("\n{} > ",info_title))
}


pub fn base64_decode(string: &str)->String{
    String::from_utf8(base64::decode(string).unwrap()).unwrap()
}


pub fn args()->std::env::Args{
    std::env::args()
}


pub fn base64_arg()->String{
    let mut args = args();
    let arg = args.nth(2);
    base64_decode(&arg.unwrap())
}


pub fn json_base64_arg()->Value{
    json_decode(&base64_arg())
}


pub fn explode(string: &str, delimiter: &str)->Vec<String>{
    let split = string.split(delimiter);
    let mut accumulator: Vec<String> = vec![];
    for s in split {
        accumulator.push(_t(s.trim()));
    }
    if accumulator.len()==1 && accumulator[0].trim().len()==0{
        accumulator = vec![];
    }
    accumulator
}



pub fn _get()->JsonValue{
    json::parse(&base64_arg()).unwrap()
}


pub fn sha256_hash(key: String)->String{
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(key);

    // read hash digest and consume hasher
    format!("{:X}",hasher.finalize())
}


pub fn shell_exec_as_string(command: &str)->String{
    
    _t(std::str::from_utf8(&Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process").stdout).unwrap()
        .trim_end()
    )
}


pub fn shell_exec(command: &str){
    Exec::shell(command).join().unwrap();  
}


pub fn bash_exec(command: &str){
    shell_exec(&format!(r#"bash -c "{command}" "#));  
}


pub fn sed_edit_sshd_like_variable(variable_name: &str, search_value: &str , replace_value: &str,file_path: &str){
    shell_exec(&format!("sed -i '/^{}/s/{}/{}/' {}",variable_name,search_value,replace_value,file_path));
    
    let from = format!("{}: {}",variable_name,search_value);
    let arrow = format!("->");
    let to = format!("{}: {}",variable_name,replace_value);
    
    println!("{} {} {}",from.bright_green(),arrow.bright_red(),to.bright_cyan());
}


pub fn sed_uncomment_then_edit_sshd_like_variable(variable_name: &str, search_value: &str , replace_value: &str,file_path: &str){ 
    shell_exec(&format!("sed -e 's/#.*{} {}/{} {}/' {} > {}.tmp && mv -f {}.tmp {}",variable_name,search_value,variable_name,replace_value,file_path,file_path,file_path,file_path));
    
    let from = format!("#{}: {}",variable_name,search_value);
    let arrow = format!("->");
    let to = format!("{}: {}",variable_name,replace_value);
    
    println!("{} {} {}",from.bright_green(),arrow.bright_red(),to.bright_cyan());
}




pub fn sed_edit_mysqld_like_variable(variable_name: &str ,replace_value: &str,file_path: &str){
    shell_exec(&format!(r#"sudo sed -i "s/.*{}.*/{} = {}/" {}"#,variable_name,variable_name,replace_value,file_path));
    
    //let from = format!("{}: {}",variable_name,search_value);
    // let arrow = format!("->");
    let var = format!("{}: {}",variable_name,replace_value);
    
    println!("{}",var.bright_green());
}



pub fn sed_uncomment_then_edit_mysqld_like_variable(variable_name: &str ,replace_value: &str,file_path: &str){
    shell_exec(&format!(r#"sudo sed -i "s/#.*{}.*/{} = {}/" {}"#,variable_name,variable_name,replace_value,file_path));
    
    //let from = format!("{}: {}",variable_name,search_value);
    // let arrow = format!("->");
    let var = format!("{} = {}",variable_name,replace_value);
    
    println!("{}",var.bright_green());
}



pub fn sed_add_new_line(value: &str, file_path: &str){
    shell_exec(&format!(r#"sed -i 's/*/{}/' {}"#,value,file_path));
    
    println!("{} append to {}",value.bright_cyan(),file_path.bright_green());
}

pub fn awk_update(property: &str, value: &str, file_path: &str, comment_character: Option<&str>, is_assignment: Option<bool>){
    let comment_character_str = if comment_character.is_some() { comment_character.unwrap() } else { "#" };
    let is_assignment_bool = if is_assignment.is_some() { is_assignment.unwrap() } else { false };

    if is_assignment_bool{

        shell_exec(&format!(r#"awk -i inplace '/^{}{}/{{gsub(/^{}{}/,"{}",$0);$3="{}"}}{{print}}' {}"#, comment_character_str, property, comment_character_str, property, property, value, file_path));

        shell_exec(&format!(r#"awk -i inplace '/^{}/{{$3="{}"}}{{print}}' {}"#, property, value, file_path));
    }else{
        shell_exec(&format!(r#"awk -i inplace '/^{}{}/{{gsub(/^{}{}/,"{}",$0);$2="{}"}}{{print}}' {}"#, comment_character_str, property, comment_character_str, property, property, value, file_path));
        shell_exec(&format!(r#"awk -i inplace '/^{}/{{$2="{}"}}{{print}}' {}"#, property, value, file_path));
    }

    let from = format!("{}: {}", property, value);
    let separator = format!("->");
    let to = format!("{}",file_path);
    
    println!("{} {} {}",from.bright_green(), separator.bright_red(),to.bright_cyan());

}


pub fn awk_add_line_after_match(pattern: &str, value: &str, file_path: &str){
    shell_exec(&format!("awk '/{}/{{print;print '{}';next}}1' {}", pattern, value, file_path));
}


pub fn add_new_line(value: &str, file_path: &str){
    let mut file_content = file_get_contents(file_path);
    
    file_content = format!("{}\n{}",file_content,value);

    file_put_contents(file_path,&file_content);

    println!("{} append to {}",value.bright_cyan(),file_path.bright_green());
}



pub fn change_unix_user_password(user: &str,password: &str){
    shell_exec(&format!(r#"echo "{username}:{password}" | chpasswd"#,username=user,password=password))
}


pub fn change_current_unix_user_password(password: &str){
    file_put_contents("passwd.sh", include_str!("../../templates/bash/passwd.sh"));
    shell_exec(&format!("chmod +x passwd.sh && ./passwd.sh {} && rm -rf passwd.sh",password));
}



pub fn stdin_lest_is_none(stdin_title: &str, option: Option<&str>)->String{
    let response: String;

    if option.is_some(){
        response = _t(option.unwrap());
    }else{
        response = stdin(stdin_title);
    }
    response
}

pub fn pathbuf_to_string(pathbuf: PathBuf)->String{
    pathbuf.into_os_string().into_string().unwrap()
}

pub fn home_dir()->String{
    pathbuf_to_string(dirs::home_dir().unwrap())
}


pub fn render_template<T>(template: &str, context: T) -> String
    where T:Serialize
{
    let mut tt = TinyTemplate::new();
    tt.add_template("template", template).unwrap();
    tt.render("template", &context).unwrap()
}



pub fn json_stringify<T: serde::Serialize>(value: T) -> String{
    let string_value = serde_json::to_string(&value).unwrap();
    return json::stringify(json::parse(&string_value).unwrap())
}


pub fn json_stringify_pretty<T: serde::Serialize>(value: T) -> String{
    let string_value = serde_json::to_string(&value).unwrap();
    return json::stringify_pretty(json::parse(&string_value).unwrap(), 4);
}

pub fn user_home_dir(user: &str) -> String{

    let home_dir_parent = if cfg!(target_os="linux") {
        "/home"
    } else if cfg!(target_os="windows") {
        "C:/users"
    } else if cfg!(target_os="macos") {
        "/Users"
    }else{
        ""
    };

    PathBuf::from_iter([
        home_dir_parent,
        user
    ]).into_os_string().into_string().unwrap()
}



pub fn round(value: f64, precision: usize) -> f64{
    format!("{:.1$}", value, precision).parse::<f64>().unwrap()
}



pub fn divide(numerator: f64, denominator: f64) -> f64{
    if denominator != 0.0{
        numerator/denominator
    }else{
        0.0
    }
}



pub fn read_password() -> String{
    rpassword::prompt_password("Password: ".bright_cyan()).unwrap()
}


pub fn read_and_confirm_password(prompt_message: &str) -> String{
    let password = rpassword::prompt_password(&format!("{prompt_message}: ").bright_cyan()).unwrap();
    let confirm_password = rpassword::prompt_password("Confirm password: ".bright_cyan()).unwrap();

    if password != confirm_password{
        println!();
        println!("Password mismatched");
        println!();
        exit(1);
    }

    password
}



pub fn command_exists(command: &str) -> bool{
    !shell_exec_as_string(&format!("which {command}")).trim().is_empty()
}


pub fn ensure_strong_password(password: &str){

    if command_exists("cracklib-check"){
    
        let escaped_password = password.replace(r#"""#, r#"\""#);
        
        let password_check_response = shell_exec_as_string(&format!(r#"echo "{escaped_password}" | cracklib-check"#));

        if password_check_response != format!("{password}: OK"){
            
            println!("Bad password. Password not strong enough.\nYou can generate a strong password using:\nopenssl rand -base64 21");

            exit(1);

        }
        
    }


}



pub fn print_header_block(title: &str){
    println!();
    println!("### ----------------------------------------------------------");
    println!("###              {title}");
    println!("### ----------------------------------------------------------");
}



pub fn pretty_print_version(bin: &str){
    println!();

    println!("$ {bin} --version");

    shell_exec(&format!("{bin} --version"));

    println!();
}