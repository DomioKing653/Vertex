use crate::backend::linker::link::Linker;
use crate::backend::linker::obj_file::ObjFile;
use crate::backend::saving_bytes::save::compile_instr_to_bytes;
use crate::backend::{
    ast::parser::Parser,
    compiler::{
        byte_code::{Compilable, Compiler},
        instructions::Instructions,
    },
    errors::diagnostics::lexer_error_print::print_lexer_err,
    lexer::{tokenizer::Lexer, tokens::Token},
    saving_bytes::binary_compilation,
};
use crate::clrprintln;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
    time::{Duration, Instant},
};
use walkdir::WalkDir;

pub static DEPENDECIES_REFS: std::sync::OnceLock<HashMap<String, String>> =
    std::sync::OnceLock::new();

pub fn set_config(map: HashMap<String, String>) {
    DEPENDECIES_REFS.set(map).expect("Config already set");
}

pub fn get_modules() -> &'static HashMap<String, String> {
    DEPENDECIES_REFS.get().expect("Config not initialized")
}
fn is_config_set() -> bool {
    DEPENDECIES_REFS.get().is_some()
}
fn create_spinner(msg: String) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg);
    pb
}

fn debug_print(tokens: &Vec<Token>, ast: Box<dyn Compilable>, instructions: &Vec<Instructions>) {
    for token in tokens {
        println!("{:?}", token);
    }
    println!("{:?}", ast);
    for instruction in instructions {
        println!("{:?}", instruction);
    }
}
pub fn compile_file_to_bytecode(
    module_name: String,
    parsed_files: HashMap<String, Box<dyn Compilable>>,
) -> ObjFile {
    let file_start = Instant::now();
    let pb = create_spinner(format!("Compiling {}", module_name));

    /*
     * AST Retrieval
     */
    let mut parsed_ast = parsed_files
        .get(&module_name)
        .unwrap_or_else(|| panic!("AST missing for module: {}", module_name))
        .clone();

    /*
     * Lookup
     */
    let mut compiler = Compiler::new();
    compiler.context.parsed_files = parsed_files;

    if let Err(e) = parsed_ast.add_to_lookup(&mut compiler) {
        pb.finish_and_clear();
        clrprintln!("$red|Error at:{}", &module_name);
        clrprintln!("$red|{}", e);
        process::exit(-3);
    }

    /*
     * Type check
     */
    parsed_ast.add_to_type_check(&mut compiler).unwrap();

    /*
     * Bytecode
     */
    if let Err(e) = parsed_ast.compile(&mut compiler) {
        pb.finish_and_clear();
        clrprintln!("$red|Error at $reset|:$cyan|{}", &module_name);
        clrprintln!("$red|{}", e);
        println!("\x1b[1mTry:vertexC error <error code> for fix\x1b[0m");
        process::exit(-3);
    }

    pb.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        format!("Compiled {}", module_name),
        file_start.elapsed().as_secs_f32()
    );

    ObjFile {
        instructions: compiler.out,
        name: module_name,
        imports: compiler.imports.clone(),
    }
}

async fn lex_project_parallel(
    dir: &String,
    main_vtx_files: &Vec<String>,
) -> HashMap<String, Vec<Token>> {
    let mut files_to_lex = Vec::new();

    // Add main project files
    for file in main_vtx_files {
        files_to_lex.push((file.clone(), dir.clone(), None));
    }

    // Add dependency files
    if is_config_set() {
        for (name, path) in get_modules() {
            if PathBuf::from(path).exists() {
                let dep_src = format!("{}/src", path);
                for file in get_vertex_files_recursive(&dep_src) {
                    files_to_lex.push((file, dep_src.clone(), Some(name.clone())));
                }
            }
        }
    }

    let mut tokens_map: HashMap<String, Vec<Token>> = HashMap::new();
    let mut lexer_join_set = tokio::task::JoinSet::new();

    for (file, base_dir, prefix) in files_to_lex {
        lexer_join_set.spawn(async move {
            let content =
                fs::read_to_string(&file).unwrap_or_else(|_| panic!("Cannot find module {}", file));
            let abs_path = fs::canonicalize(&file).unwrap_or_else(|_| PathBuf::from(&file));
            let main_lexer: Lexer = Lexer::new(content.clone(), abs_path); // pass file path
            // also so that Lexer can have also have the file that was generating error.
            let result = main_lexer.tokenize();
            (file, base_dir, prefix, content, result)
        });
    }

    while let Some(res) = lexer_join_set.join_next().await {
        let (file, base_dir, prefix, content, result) = res.unwrap();
        match result {
            Err(e) => {
                clrprintln!("$red|Error at {}:", file);
                print_lexer_err(e, content);
                process::exit(-1);
            }
            Ok(tokens) => {
                let rel_path = if file.starts_with(&base_dir) {
                    file.strip_prefix(&base_dir)
                        .unwrap()
                        .trim_start_matches('/')
                        .to_string()
                } else {
                    file.clone()
                };

                let key = match prefix {
                    Some(p) => format!("{}/{}", p, rel_path),
                    None => rel_path,
                };

                tokens_map.insert(key, tokens);
            }
        }
    }
    tokens_map
}

async fn parse_project_parallel(
    tokens_map: &HashMap<String, Vec<Token>>,
) -> HashMap<String, Box<dyn Compilable>> {
    let mut parser_join_set = tokio::task::JoinSet::new();
    let mut parsed_ast_map: HashMap<String, Box<dyn Compilable>> = HashMap::new();

    for (file_name, tokens) in tokens_map {
        let tokens_clone = tokens.clone();
        let name_clone = file_name.clone();
        parser_join_set.spawn(async move {
            let mut main_parser = Parser::new(tokens_clone);
            (name_clone, main_parser.parse())
        });
    }

    while let Some(res) = parser_join_set.join_next().await {
        let (file_name, parsed_file) = res.unwrap();
        match parsed_file {
            Err(e) => {
                println!("Error at {}:", &file_name);
                println!("\x1b[1;31m{}\x1b[0m", e);
                process::exit(-2);
            }
            Ok(ast) => {
                parsed_ast_map.insert(file_name, ast);
            }
        }
    }
    parsed_ast_map
}

//NOTE:This is just entry point for the compilation process, and it
//shouldn't be used any further in the compilation process
pub async fn build_prj(dir: String, out: String, debug: bool) {
    let total_start = Instant::now();

    let src_path = Path::new(&dir)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&dir));

    println!(
        "\n\x1b[1;32mBuilding\x1b[0m {} -> out/{}\n",
        src_path.display(),
        out
    );

    ensure_target_dir();
    let main_vtx_files = get_vertex_files_recursive(&dir);

    /*
     * Lexing phase
     */
    let pb_lexing = create_spinner("Lexing project".to_string());
    let lex_start = Instant::now();
    let tokens_map = lex_project_parallel(&dir, &main_vtx_files).await;
    pb_lexing.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Lexed project",
        lex_start.elapsed().as_secs_f32()
    );

    /*
     * Parsing phase
     */
    let pb_parsing = create_spinner("Parsing project".to_string());
    let parse_start = Instant::now();
    let parsed_ast_map = parse_project_parallel(&tokens_map).await;
    pb_parsing.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Parsed project",
        parse_start.elapsed().as_secs_f32()
    );

    /*
     * Compile Phase
     */
    let mut objs: Vec<ObjFile> = Vec::new();

    for file in main_vtx_files {
        let key = if file.starts_with(&dir) {
            file.strip_prefix(&dir)
                .unwrap()
                .trim_start_matches('/')
                .to_string()
        } else {
            file.clone()
        };

        objs.push(compile_file_to_bytecode(key, parsed_ast_map.clone()));
    }

    /*
     * Linking
     */
    let pb_linking = create_spinner("Linking".to_string());
    let link_start = Instant::now();
    let mut final_file = Linker::link(&mut objs); // Link all Obj files
    final_file = Compiler::optimize(final_file); // Optimize the final bytecode emmited by the Linker

    if debug {
        pb_linking.suspend(|| {
            println!("\n--- BYTECODE ---");
            for (i, instr) in final_file.iter().enumerate() {
                println!("{}->{:?}", i, instr);
            }
            println!("----------------\n");
        });
    }

    pb_linking.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Linked",
        link_start.elapsed().as_secs_f32()
    );

    /*
     * Write output
     */
    let pb_writing = create_spinner("Writing output".to_string());
    let write_start = Instant::now();

    let out_path = format!("out/{}", out);

    compile_instr_to_bytes(out_path, &mut final_file).expect("Cannot load binary file");

    pb_writing.finish_and_clear();
    println!(
        "\x1b[32m✔\x1b[0m {:<50} in {:.4}s",
        "Finished writing",
        write_start.elapsed().as_secs_f32()
    );

    binary_compilation::compile_to_binary(&out);

    println!(
        "\n\x1b[1;32mBuild finished\x1b[0m in {:.3}s",
        total_start.elapsed().as_secs_f32()
    );
}

fn ensure_target_dir() {
    let target = std::env::current_dir().unwrap().join("out/bin");

    if !&target.exists() {
        fs::create_dir_all(target).expect("Could not create the binary output directory");
    }
}

fn get_vertex_files_recursive(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.expect("Cannot read entry");
        if entry.file_type().is_file()
            && let Some(ext) = entry.path().extension()
            && ext == "vtx"
        {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }

    files
}
