use std::env::{self};
use std::fs::{self,File};
use std::io::Write;
use std::process;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config{
    name:String
}
fn main() {
    let args:Vec<String> = env::args().collect(); 
    if args.len() > 1{
        match  args[1].as_str(){
            "create" =>{
                if args.len() > 2 {
                    
                    let project_name = args[2].clone();

                    fs::create_dir(&project_name).unwrap();

                    fs::create_dir(format!("{}/src",&project_name)).unwrap();

                    let mut config = File::create(format!("{}/prj.config",&project_name)).unwrap();
                    config.write_all(format!("name = {}",&project_name).as_bytes()).unwrap();
                    let mut main_file = File::create(format!("{}/src/main.flare",&project_name)).unwrap();
                    main_file.write_all(b"writeln!(\"Hello world\");\n").unwrap(); 
                
                }
            }
            "build" =>{
                let tex = fs::read_to_string("prj.config").unwrap();
                let config:Config = match toml::from_str(&tex) {
                   Err(e)=>{
                       print!("Cannot find 'prj.config' in cwd");
                       process::exit(-1);

                   }
                   Ok(c) =>{
                       c
                   }
                };                 

            }

           _=>{
                
           } 
        }
    }    
    else {
        println!("[1;32m Expected argument [  ");
    }
}
