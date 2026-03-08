// NOTE:
// This is the main Flare compiler CLI used until `flauncher` is ready for production.
// It is intended to compile a single file without external dependencies.
// Currently it does not have a working linker.
// Once `flauncher` is ready, this tool will likely be replaced or deprecated and not be ready for
// production.



use flare::backend::{
    compiler::saving_bytes::save::build_directory,
    errors::cli_errors::CommandLineError::{
        self, BuildHasJustTwoArg, NoFileSpecifiedForBuild, NoSuchCommand,
    },
    errors::compiler::error_explain::ERROR_EXPLAIN,
};
use flare::runtime::runner::running_vm::run_code;
use std::env;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!(
            "Fatal error:{:?}!",
            match e {
                BuildHasJustTwoArg => "Build command has just two arguments",
                NoFileSpecifiedForBuild => "No file specified for build",
                NoSuchCommand => "No such command",
            }
            .to_string()
        );
    }
}

fn run_cli() -> Result<(), CommandLineError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(NoSuchCommand);
    }

    match args[1].as_str() {
        "build" => {
            let (debug, source, output) = parse_build_args(&args[2..])?;
            build_directory(source, output, debug);
            Ok(())
        }
        "run" => {
            if args.len() != 3 {
                return Err(NoFileSpecifiedForBuild);
            }
            run_code(&args[2].clone());
            Ok(())
        }
        "exec" => {
            let (debug, source, output) = parse_build_args(&args[2..])?;
            build_directory(source.clone(), output.clone(), debug);
            run_code(&format!("out/{}", &output));
            Ok(())
        }
        "error" => {
            if args.len() != 3 {
                eprintln!("Usage: flarec explain <ERROR_CODE>");
                return Ok(());
            }

            let code = args[2].as_str();

            match ERROR_EXPLAIN.get(code) {
                Some(text) => println!("{}", text),
                None => eprintln!("Unknown error code: {}", code),
            }

            Ok(())
        }
        _ => Err(NoSuchCommand),
    }
}

fn parse_build_args(args: &[String]) -> Result<(bool, String, String), CommandLineError> {
    let mut debug = false;
    let mut source = String::new();
    let mut output = String::new();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-d" => {
                debug = true;
                i += 1;
            }
            _ => {
                if source.is_empty() {
                    source = args[i].clone();
                } else if output.is_empty() {
                    output = args[i].clone();
                } else {
                    return Err(BuildHasJustTwoArg);
                }
                i += 1;
            }
        }
    }

    if source.is_empty() || output.is_empty() {
        return Err(BuildHasJustTwoArg);
    }

    Ok((debug, source, output))
}
