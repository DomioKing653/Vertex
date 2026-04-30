use crate::backend::compiler::instructions::Instructions;
use crate::backend::linker::obj_file::ObjFile;
pub fn patch_objs_jumps(sorted_objects: Vec<ObjFile>, program: &mut Vec<Instructions>) {
    let mut offset: usize = 0;
    for obj in sorted_objects {
        let mut patched = Vec::new();

        for instr in obj.instructions {
            let new_instr = match instr {
                Instructions::Jump(addr) => Instructions::Jump(addr + offset),

                Instructions::JumpIfTrue(addr) => Instructions::JumpIfTrue(addr + offset),

                Instructions::JumpIfFalse(addr) => Instructions::JumpIfFalse(addr + offset),

                other => other,
            };

            patched.push(new_instr);
        }

        offset += patched.len();
        program.extend(patched);
    }
}
