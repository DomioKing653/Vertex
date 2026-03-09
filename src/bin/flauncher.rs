//!Main flare package manager and linker
use flare::backend::compiler::saving_bytes::save::build_directory;
use serde::Deserialize;
use std::env::{self};
use std::fs::{self, File};
use std::io::Write;
use std::process;

#[derive(Deserialize)]
struct Config {
    name: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "create" => {
                if args.len() > 2 {
                    let project_name = args[2].clone();

                    fs::create_dir(&project_name).unwrap();

                    fs::create_dir(format!("{}/src", &project_name)).unwrap();

                    let mut config = File::create(format!("{}/prj.toml", &project_name)).unwrap();
                    config
                        .write_all(format!("name = \"{}\"", &project_name).as_bytes())
                        .unwrap();
                    let mut main_file =
                        File::create(format!("{}/src/main.flare", &project_name)).unwrap();
                    main_file
                        .write_all(b"writeLn!(\"Hello world\");\n")
                        .unwrap();
                }
            }
            "build" => {
                let debug = parse_flags(args);
                
                let tex = fs::read_to_string("prj.toml").unwrap();
                let config: Config = match toml::from_str(&tex) {
                    Err(e) => {
                        print!("{}", e);
                        process::exit(-1);
                    }
                    Ok(c) => c,
                };

                File::open("src/main.flare").unwrap_or_else(|e| {
                    print!("cannot find main.flare in ./src");
                    process::exit(-1);
                });
                build_directory("src/".to_string(), config.name, debug);
            }
            _ => {}
        }
    } else {
        println!("Expected argument");
    }
}

fn parse_flags(args:Vec<String>) -> bool {
    for arg in args {
        if arg == "-d" {
            return true
        }
    }
    false
}