//!Main vertex package manager and linker
use vertex::backend::saving_bytes::save::build_directory;
use serde::Deserialize;
use std::{
    env::{self},
    fs::{self, File, remove_dir_all},
    io::Write,
    process
};

#[derive(Deserialize)]
struct Config {
    name: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "help"=>{
                println!(
                r#"vertex — project manager for the Vertex language

                USAGE:
                    vertex <COMMAND>

                COMMANDS:

                    build
                        Build project into bytecode using configuration
                        from prj.toml. Output is placed into ./out/

                    new <PROJECT_NAME>
                        Create a new Vertex project:

                            <PROJECT_NAME>/
                                src/
                                    main.vtx
                                prj.toml

                        main.vtx contains a Hello World example.

                    clear
                        Remove all build artifacts inside ./out/

                DESCRIPTION:
                    vertex is the official project manager and build tool
                    for the Vertex programming language.
                "#
                );

            }
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
            "clear" => {
                remove_dir_all("./out").unwrap();
            }
            _ => {}
        }
    } else {
        println!("Expected argument");
    }
}

fn parse_flags(args: Vec<String>) -> bool {
    for arg in args {
        if arg == "-d" {
            return true;
        }
    }
    false
}
