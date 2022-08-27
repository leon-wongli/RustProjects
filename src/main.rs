use std::{env};
use hello_cargo::{Config};
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();

    //读取这个环境变量 CASE_INSENSITIVE=1 cargo run 
    // let CASE_INSENSITIVE = env::var("CASE_INSENSITIVE").is_err();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{}", err);
        // let defaultvec = vec!["".to_string(), "error".to_string(), "error".to_string()];
        // Config::new(&defaultvec).unwrap()
        process::exit(1);
    });

    if let Err(e) = hello_cargo::run(config){
        print!("Application error:{}",e);
        process::exit(1);
    };
}

