use std::time::Instant;

use crate::runtime::virtual_machine::vm::VM;

pub fn run_code(path: &str) {
    let program_time_start = Instant::now();
    let mut vm: VM = VM::from_file(path).unwrap();
    vm.run().unwrap();
    let elapsed = program_time_start.elapsed();
    let seconds = elapsed.as_secs_f32();
    println!(
        "\n\x1b[1;32mProgram finished in {:.3} seconds\x1b[0m",
        seconds
    )
}
