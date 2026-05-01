// NOTE:This is the main Vertex compiler CLI used until `vertex` is ready for production.
// It is intended to compile a single file without external dependencies.
// Currently, it does not have a working linker. Once `vertex` is ready, this tool will likely be replaced or deprecated and not be ready for
// production.
use std::env;
use std::path::PathBuf;
use vertex::backend::saving_bytes::compile_tools::build_directory;
use vertex::backend::{
    errors::cli_errors::CommandLineError::{
        self, BuildHasJustTwoArg, NoFileSpecifiedForBuild, NoSuchCommand,
    },
    errors::compiler::error_explain::ERROR_EXPLAIN,
};
use vertex::runtime::runner::running_vm::run_code;
fn main() {
    if let Err(e) = run_cli() {
        eprintln!(
            "{}",
            match e {
                BuildHasJustTwoArg => "Build command has just two arguments",
                NoFileSpecifiedForBuild => "No file specified for build",
                NoSuchCommand => "No such command. Run 'vertexC help for more info'",
                _ => todo!(),
            }
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
            let (debug, source, output, path_to_vm) = parse_build_args(&args[2..])?;
            build_directory(source, output, debug, Some(PathBuf::from(path_to_vm)));
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
            let (debug, source, output, path_to_vm) = parse_build_args(&args[2..])?;
            build_directory(
                source.clone(),
                output.clone(),
                debug,
                Some(PathBuf::from(path_to_vm)),
            );
            run_code(&format!("out/{}", &output));
            Ok(())
        }
        "error" => {
            if args.len() != 3 {
                eprintln!("Usage: vertexC error [ERROR_CODE]");
                return Ok(());
            }

            let code = args[2].as_str();

            match ERROR_EXPLAIN.get(code) {
                Some(text) => println!("{}", text),
                None => eprintln!("Unknown error code: {}", code),
            }

            Ok(())
        }
        "help" => {
            vertex::clrprintln!(
                r#"
            vertexC — compiler tool for Vertex

            vertexC compiles a single source file into Vertex bytecode.
            It will remain available even after the 'vertex' project
            manager is finished, mainly for testing and low-level workflows.

            $green|USAGE:$reset|
                vertexC build $cyan|<INPUT_FILE> <OUTPUT_FILE>$reset|
                    Compile source file into bytecode stored in ./out/
                    flags:
                        -d:show final instructions

                vertexC run $cyan|<BYTECODE>$reset|
                    Execute bytecode using VVM (Vertex Virtual Machine)

                vertexC exec $cyan|<INPUT_FILE> <OUTPUT_FILE>$reset|
                    Compile and immediately run the produced bytecode
                    flags:
                        -d:show final instructions

                vertexC error $cyan|<ERORR_CODE>$reset|
                    Explains erorr more deeply
            "#
            );
            Ok(())
        }
        "build-lib" => {
            eprint!("Building vertex libs is still unimplemented");
            Ok(())
        }
        _ => Err(NoSuchCommand),
    }
}

fn parse_build_args(args: &[String]) -> Result<(bool, String, String, String), CommandLineError> {
    let mut debug = false;
    let mut source = String::new();
    let mut output = String::new();
    let mut path_to_vm = String::new();

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
                } else if output.is_empty() {
                    path_to_vm = args[i].clone();
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

    Ok((debug, source, output, path_to_vm))
}
