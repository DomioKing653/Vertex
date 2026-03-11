use crate::backend::{ast::parser::Parser, compiler::{
    byte_code::{Compilable, Compiler},
    instructions::Instructions,
}, lexer::{lexer::Lexer, tokens::Token}};

use crate::backend::linker::link::Linker;
use crate::backend::linker::obj_file::ObjFile;
use std::{
    fs,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    process,
    time::Instant,
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
//NOTE:This uses relative path from the compiler
// so you need to cd in first, and then it run program main at the flauncher
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
        fs::read_to_string(&dir)
            .expect(format!("Cannot find module {}", &dir).as_ref()),
    );

    let tokens: &Vec<Token> = match main_lexer.tokenize() {
        Err(e) => {
            println!("Error at {}:", &dir);
            print!("{}", e);
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
    parsed_ast.add_to_lookup(&mut compiler).unwrap();
    /*
     * Type check
     */
    parsed_ast.add_to_type_check(&mut compiler).unwrap();

    /*
     * Bytecode
     */
    if let Err(e) = parsed_ast.compile(&mut compiler) {
        println!("Error at {}:", &dir);
        println!("\x1b[1;31m{}\x1b[0m", e);
        println!("\x1b[1mTry:flarec error <error code> for fix\x1b[0m");
        process::exit(-3);
    }

    compiler.optimize();

    println!(
        "  compiled {:<40} {:.4}s",
        dir,
        file_start.elapsed().as_secs_f32()
    );

    ObjFile {
        instructions: compiler.out,
        name: dir.clone(),
        imports: compiler.imports,
    }
}

//NOTE:This is just entry point for the compilation process, and it
// shouldn't be used any further in the compilation process
pub fn build_directory(dir: String, out: String, debug: bool) {
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

    for file in get_flare_files_recursive(&dir) {
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

    if debug {
        println!("\n--- BYTECODE ---");
        for instr in &final_file {
            println!("{:?}", instr);
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
    compile_instr_to_bytes(out_path, &mut final_file)
        .expect("Cannot load binary file");

    println!(
        "\x1b[32mFinished writing\x1b[0m in {:.4}s\n",
        write_start.elapsed().as_secs_f32()
    );

    /*
     * TOTAL TIME
     */
    println!(
        "\x1b[1;32mBuild finished\x1b[0m in {:.4}s",
        total_start.elapsed().as_secs_f32()
    );
}

fn compile_instr_to_bytes(
    file_name: String,
    byte_code: &mut Vec<Instructions>,
) -> std::io::Result<()> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    for instr in byte_code {
        let opcode = instr.opcode();
        match instr {
            Instructions::Add => writer.write_all(&[opcode])?,
            Instructions::Sub => writer.write_all(&[opcode])?,
            Instructions::Mul => writer.write_all(&[opcode])?,
            Instructions::Div => writer.write_all(&[opcode])?,
            Instructions::Modulo => writer.write_all(&[opcode])?,
            Instructions::PushString(s) => {
                writer.write_all(&[opcode])?;
                let bytes = s.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&s.as_bytes())?
            }

            //Values
            Instructions::PushBool(b) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&[*b as u8])?;
            }
            Instructions::PushNumber(n) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&n.to_le_bytes())?;
            }

            Instructions::WriteLnLastOnStack => {
                writer.write_all(&[opcode])?;
            }
            Instructions::WriteLastOnStack => {
                writer.write_all(&[opcode])?;
            }
            Instructions::ProcessExit => {
                writer.write_all(&[opcode])?;
            }
            Instructions::JumpIfTrue(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }

            Instructions::LoadVar(v) => {
                writer.write_all(&[opcode])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&v.as_bytes())?
            }
            Instructions::SaveVar(v) => {
                writer.write_all(&[opcode])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(&v.as_bytes())?
            }
            Instructions::Jump(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
            Instructions::JumpIfFalse(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
            Instructions::GreaterThan => {
                writer.write_all(&[opcode])?;
            }
            Instructions::LessThan => {
                writer.write_all(&[opcode])?;
            }
            Instructions::Equal => {
                writer.write_all(&[opcode])?;
            }
            Instructions::ReadInput => {
                writer.write_all(&[opcode])?;
            }

            Instructions::Halt => writer.write_all(&[opcode])?,
        }
    }
    Ok(())
}

fn ensure_target_dir() {
    let target = std::env::current_dir().unwrap().join("out");
    if !target.exists() {
        fs::create_dir(target).expect("Cannot create target directory");
    }
}


fn get_flare_files_recursive(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry.expect("Cannot read entry");
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "flare" {
                    files.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }

    files
}
