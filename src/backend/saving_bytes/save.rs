use crate::backend::compiler::instructions::Instructions;

use std::{
    fs::File,
    io::{BufWriter, Write},
};
///This function takes name of file and Vec of instructions and than convertes them to bytes and saves them the file
/// # Returns
/// Result
pub fn compile_instr_to_bytes(
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
                writer.write_all(s.as_bytes())?
            }
            Instructions::Drop(s) => {
                writer.write_all(&[opcode])?;
                let bytes = s.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(s.as_bytes())?
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
            Instructions::PushJmpAdress(size) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&size.to_le_bytes())?;
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
                writer.write_all(v.as_bytes())?
            }
            Instructions::SaveVar(v) => {
                writer.write_all(&[opcode])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(v.as_bytes())?
            }
            Instructions::AssignVar(v) => {
                writer.write_all(&[opcode])?;
                let bytes = v.as_bytes();
                writer.write_all(&(bytes.len() as u32).to_le_bytes())?;
                writer.write_all(v.as_bytes())?
            }
            Instructions::Jump(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
            Instructions::JumpIfFalse(adr) => {
                writer.write_all(&[opcode])?;
                writer.write_all(&(*adr as u16).to_le_bytes())?;
            }
            Instructions::JumpOnLastOnStack => {
                writer.write_all(&[opcode])?;
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
