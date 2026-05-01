//!Main vertex package manager and linker
use serde::Deserialize;
use std::{
    env::{self},
    fs::{self, File, remove_dir_all},
    io::Write,
    process,
};
use vertex::{
    backend::{errors::cli_errors::CommandLineError, saving_bytes::compile_tools::build_directory},
    clrprintln,
};

#[derive(Deserialize)]
struct Config {
    name: String,
}
fn main() {
    if let Err(err) = run_cli() {
        eprintln!("{}", err);
    }
}

fn run_cli() -> Result<(), CommandLineError> {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        return Err(CommandLineError::InvalidCommand);
    }

    if let Some(arg1) = args.get(1) {
        match arg1.as_str() {
            "help" => {
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
            "new" => {
                if let Some(project_name) = args.get(2) {
                    // TODO: Add appropriate directory
                    fs::create_dir(project_name)
                        .map_err(|_| CommandLineError::ErrorCreatingDirectory)?;

                    fs::create_dir(format!("{}/src", &project_name))
                        .map_err(|_| CommandLineError::ErrorCreatingDirectory)?;

                    let mut config = File::create(format!("{}/prj.toml", &project_name)).unwrap();

                    config
                        .write_all(format!("name = \"{}\"", &project_name).as_bytes())
                        .unwrap();
                    let mut main_file = File::create(format!("{}/src/main.vtx", &project_name))
                        .map_err(|_| CommandLineError::ErrorCreatingFile)?;

                    main_file
                        .write_all(b"writeLn!(\"Hello world\");\n")
                        .unwrap();
                }
            }
            "build" => {
                let debug = parse_flags(&args);

                let tex = fs::read_to_string("prj.toml")
                    .map_err(|_| CommandLineError::ErrorFindingDirectory)?;

                let config: Config = match toml::from_str(&tex) {
                    Err(e) => {
                        print!("{}", e);
                        process::exit(-1);
                    }
                    Ok(c) => c,
                };

                File::open("src/main.vtx").unwrap_or_else(|e| {
                    clrprintln!("$red|Linker error -> Cannot find main.vtx in ./src");
                    process::exit(-1);
                });
                build_directory("src/".to_string(), config.name, debug, None)
            }
            "clear" => remove_dir_all("./out").unwrap(),
            _ => return Err(CommandLineError::NoSuchCommand),
        }
    } else {
        return Err(CommandLineError::InvalidCommand);
    }
    Ok(())
}

fn parse_flags(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "-d")
}
