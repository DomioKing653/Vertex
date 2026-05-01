use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use crate::clrprintln;
/// You need to have zig toolchain installed to compile this. We are creating
/// temp_launcher and than compiling it with:
/// `zig build-exe tmp_launcher_path.zig runtime_path -lc -lunwind
/// -Doptimize=ReleaseSmall femit-bin=out/bin/out`
pub fn compile_to_binary(out: &str) {
    println!("\x1b[1mCompiling with zig\x1b[0m");
    let compiler_timer = Instant::now();
    let bytecode_path = format!("out/{}", out);
    let temp_launcher = format!(
        r#"
const std = @import("std");
extern fn vm_entry(ptr: [*]const u8, len: usize) void;
var program = @embedFile("{bytecode_path}");
pub fn main() !void {{
    vm_entry(program.ptr, program.len);
}}
"#,
        bytecode_path = bytecode_path
    );
    let tmp_launcher_path = "tmp_launcher.zig";
    fs::write(tmp_launcher_path, temp_launcher).unwrap();
    let runtime_path = find_libvm_runtime(Path::new(".")).unwrap();
    let status = Command::new("zig")
        .args([
            "build-exe",
            "tmp_launcher.zig",
            &runtime_path,
            "-lc",
            "-lunwind",
            "-Doptimize=ReleaseSmall",
            &format!("-femit-bin=out/bin/{}", out),
        ])
        .status()
        .expect("Failed to run zig");
    if !status.success() {
        panic!("zig failed");
    }
    fs::remove_file(tmp_launcher_path).unwrap();
    clrprintln!("$green|");
    clrprintln!(&format!(
        "$green|Finished compiling with zig$reset| in {:.4}",
        compiler_timer.elapsed().as_secs_f32()
    ));
}

fn find_libvm_runtime(start: &Path) -> Option<String> {
    let mut current: Option<&Path> = Some(start);

    while let Some(dir) = current {
        if let Some(found) = find_in_dir_recursive(dir) {
            return Some(found);
        }
        current = dir.parent();
    }

    None
}

fn find_in_dir_recursive(start: &Path) -> Option<String> {
    if !start.is_dir() {
        return None;
    }

    let entries = fs::read_dir(start).ok()?;

    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name()
                && name == "libvm_runtime.a"
            {
                return Some(path.to_string_lossy().to_string());
            }
        } else if path.is_dir()
            && let Some(found) = find_in_dir_recursive(&path)
        {
            return Some(found);
        }
    }

    None
}
