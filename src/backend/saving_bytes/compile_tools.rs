use crate::backend::{
    ast::parser::Parser, compiler::{
        byte_code::{Compilable, Compiler},
        instructions::Instructions,
    }, errors::diagnostics::diagnostics::print_lexer_err, lexer::{lexer::Lexer, tokens::Token}, saving_bytes::binary_compilation
};
use crate::backend::linker::link::Linker;
use crate::clrprintln;
use crate::backend::linker::obj_file::ObjFile;
use crate::backend::saving_bytes::save::compile_instr_to_bytes;
use std::{
 fs,  path::{Path, PathBuf}, process, time::Instant
};
use walkdir::WalkDir;
fn debug_print(tokens: &Vec<Token>, ast: Box<dyn Compilable>, instructions: &Vec<Instructions>) {
    for token in tokens {
        println!("{:?}", token);
    }
    println!("{:?}", ast);
    for instruction in instructions {
        println!("{:?}", instruction);
    }
}
///This functions does compilation process of one single file. It creates tokens, build ast, create lookup for imported variables, updates types in type table, creates bytecode and optimizes it.
/// # Returns
/// Singular ObjFile
/// # Example
///```
/// let code = "..." //some file
/// let final_obj = compile_file_to_bytecode(code)
/// //now you can do anything with the ObjFile
/// ```
pub fn compile_file_to_bytecode(dir: String) -> ObjFile {
    let file_start = Instant::now();

    /*
     * Lexer
     */
    let mut main_lexer: Lexer = Lexer::new(
        fs::read_to_string(&dir).expect(format!("Cannot find module {}", &dir).as_ref()),
    );

    let tokens: &Vec<Token> = match main_lexer.tokenize() {
        Err(e) => {
            clrprintln!("$red|Error at {}:", &dir);
            print_lexer_err(e,fs::read_to_string(&dir).unwrap()); 
            process::exit(-1);
        }
        Ok(tokens) => tokens,
    };
    /*
     * Parser
     */
    let mut main_parser: Parser = Parser::new(tokens.to_vec());
    let mut parsed_ast = main_parser.parse().unwrap_or_else(|e| {
        println!("Error at {}:", &dir);
        println!("\x1b[1;31m{}\x1b[0m", e);
        process::exit(-2)
    });
    /*
     * Lookup
     */
    let mut compiler = Compiler::new();
    if let Err(e) = parsed_ast.add_to_lookup(&mut compiler) {
        clrprintln!("$red|Error at:{}", &dir);
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
        clrprintln!("$red|Error at $reset|:$cyan|{}", &dir);
        clrprintln!("$red|{}", e);
        println!("\x1b[1mTry:vertexC error <error code> for fix\x1b[0m");
        process::exit(-3);
    }

    println!(
        "  compiled {:<40} {:.4}s",
        dir,
        file_start.elapsed().as_secs_f32()
    );
    ObjFile {
        instructions: compiler.out,
        name: dir.clone().replace("src/", ""),
        imports: compiler.imports.clone(),
    }
}

//NOTE:This is just entry point for the compilation process, and it
//shouldn't be used any further in the compilation process
pub fn build_directory(dir: String, out: String, debug: bool, vm_path:Option<PathBuf>) {
    ensure_target_dir();

    let total_start = Instant::now();

    let src_path = Path::new(&dir)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(&dir));

    println!(
        "\n\x1b[1;32mBuilding\x1b[0m {} -> out/{}\n",
        src_path.display(),
        out
    );

    /*
     * Compile phase
     */
    println!("\x1b[1mCompiling\x1b[0m");

    let compile_start = Instant::now();
    let mut objs: Vec<ObjFile> = Vec::new();

    for file in get_vertex_files_recursive(&dir) {
        objs.push(compile_file_to_bytecode(file));
    }

    println!(
        "\x1b[32mFinished compiling\x1b[0m in {:.4}s\n",
        compile_start.elapsed().as_secs_f32()
    );

    /*
     * Linking
     */

    println!("\x1b[1mLinking\x1b[0m");

    let link_start = Instant::now();

    let mut final_file = Linker::link(objs);

    final_file = Compiler::optimize(final_file);

    if debug {
        println!("\n--- BYTECODE ---");
        let mut i = 0;
        for instr in &final_file {
            println!("{}->{:?}", i, instr);
            i += 1;
        }
        println!("----------------\n");
    }

    println!(
        "\x1b[32mFinished linking\x1b[0m in {:.4}s\n",
        link_start.elapsed().as_secs_f32()
    );

    /*
     * Write output
     */
    println!("\x1b[1mWriting output\x1b[0m");

    let write_start = Instant::now();

    let out_path = format!("out/{}", out);

    compile_instr_to_bytes(out_path, &mut final_file).expect("Cannot load binary file");

    println!(
        "\x1b[32mFinished writing\x1b[0m in {:.4}s\n",
        write_start.elapsed().as_secs_f32()
    );

    binary_compilation::compile_to_binary(&out);


    /*
     * TOTAL TIME
     */
    println!(
        "\x1b[1;32mBuild finished\x1b[0m in {:.3}s",
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
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "vtx" {
                    files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }

    files
}


