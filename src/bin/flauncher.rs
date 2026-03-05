use std::env::{self};
use std::fs::{self,File};
use std::io::Write;
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
                    let mut main_file = File::create(format!("{}/main.flare",&project_name)).unwrap();

                  
                    main_file.write_all(b"writeln!(\"Hello world\");\n").unwrap(); 

                
                }
            }

           _=>{

           } 
        }
    }    
    else {
        println!("[1;32m Expected argument [  ");
    }
}
