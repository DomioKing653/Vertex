use crate::backend::compiler::instructions::{self, Instructions};
use std::{error::Error, fs};
pub struct BytecodeLoader {
    bytes: Vec<u8>,
    pos: usize,
}

impl BytecodeLoader {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Vec<Instructions>, Box<dyn std::error::Error>> {
        let mut loader = Self { bytes, pos: 0 };
        loader.parse()
    }
    pub fn from_file(path: &str) -> Result<Vec<Instructions>, Box<dyn Error>> {
        let bytes = fs::read(path).map_err(|e| format!("Unable to load file {}: {}", path, e))?;
        Self::from_bytes(bytes)
    }

    fn parse(&mut self) -> Result<Vec<Instructions>, Box<dyn Error>> {
        let mut instructions = Vec::new();

        while self.pos < self.bytes.len() {
            let opcode = self.read_u8()?;

            let instruction = match opcode {
                instructions::ADD  => Instructions::Add,
                instructions::SUB=> Instructions::Sub,
                instructions::MUL => Instructions::Mul,
                instructions::DIV => Instructions::Div,
                instructions::MODULO => Instructions::Modulo,

                instructions::PUSH_STR => {
                    let len = self.read_u32()? as usize;
                    let s = self.read_string(len)?;
                    Instructions::PushString(s)
                }
                instructions::DROP => {
                    let len = self.read_u32()? as usize;
                    let s = self.read_string(len)?;
                    Instructions::Drop(s)
                }

                instructions::LOAD_VAR => {
                    let len = self.read_u32()? as usize;
                    let name = self.read_string(len)?;
                    Instructions::LoadVar(name)
                }

                instructions::STORE_VAR => {
                    let len = self.read_u32()? as usize;
                    let name = self.read_string(len)?;
                    Instructions::SaveVar(name)
                }

                instructions::ASSIGN_VAR => {
                    let len = self.read_u32()? as usize;
                    let name = self.read_string(len)?;
                    Instructions::AssignVar(name)
                }

                instructions::PUSH_BOOL => {
                    let value = self.read_u8()? != 0;
                    Instructions::PushBool(value)
                }

                instructions::PUSH_NUMB => {
                    let value = self.read_f32()?;
                    Instructions::PushNumber(value)
                }

                instructions::PUSH_USIZE => {
                    let value = self.read_usize()?;
                    Instructions::PushJmpAdress(value)
                }
                instructions::WRITE_LN=> Instructions::WriteLnLastOnStack,
                instructions::WRITE => Instructions::WriteLastOnStack,
                instructions::PROCESS_EXIT => Instructions::ProcessExit,
                instructions::JUMP_IF_TRUE => {
                    let addr = self.read_u16()? as usize;
                    Instructions::JumpIfTrue(addr)
                }
                instructions::JUMP => {
                    let addr = self.read_u16()? as usize;
                    Instructions::Jump(addr)
                }

                instructions::JUMP_IF_FALSE => {
                    let addr = self.read_u16()? as usize;
                    Instructions::JumpIfFalse(addr)
                }
                instructions::JUMP_LAST_ON_STACK=>{
                    Instructions::JumpOnLastOnStack
                }

                instructions::GREATER => Instructions::GreaterThan,
                instructions::LESS => Instructions::LessThan,
                instructions::EQUAL => Instructions::Equal,
                instructions::READ_INPUT => Instructions::ReadInput,

                instructions::HALT => Instructions::Halt,

                _ => {
                    return Err(
                        format!("Unknown opcode: {} at position {}", opcode, self.pos - 1).into(),
                    );
                }
            };

            instructions.push(instruction);
        }
        Ok(instructions)
    }
    /*
     * Readers
     */
    fn read_u8(&mut self) -> Result<u8, Box<dyn Error>> {
        if self.pos >= self.bytes.len() {
            return Err("Unexpected EOF reading u8".into());
        }
        let value = self.bytes[self.pos];
        self.pos += 1;
        Ok(value)
    }

    fn read_u16(&mut self) -> Result<u16, Box<dyn Error>> {
        if self.pos + 2 > self.bytes.len() {
            return Err("Unexpected EOF reading u16".into());
        }
        let bytes: [u8; 2] = self.bytes[self.pos..self.pos + 2]
            .try_into()
            .map_err(|_| "Failed to read u16")?;
        self.pos += 2;
        Ok(u16::from_le_bytes(bytes))
    }

    fn read_u32(&mut self) -> Result<u32, Box<dyn Error>> {
        if self.pos + 4 > self.bytes.len() {
            return Err("Unexpected EOF reading u32".into());
        }
        let bytes: [u8; 4] = self.bytes[self.pos..self.pos + 4]
            .try_into()
            .map_err(|_| "Failed to read u32")?;
        self.pos += 4;
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_usize(&mut self) -> Result<usize, Box<dyn Error>> {
        if self.pos + 8 > self.bytes.len() {
            return Err("Unexpected EOF reading u32".into());
        }
        let bytes: [u8; 8] = self.bytes[self.pos..self.pos + 8]
            .try_into()
            .map_err(|_| "Failed to read usize")?;
        self.pos += 8;
        Ok(usize::from_le_bytes(bytes))
    }

    fn read_f32(&mut self) -> Result<f32, Box<dyn Error>> {
        if self.pos + 4 > self.bytes.len() {
            return Err("Unexpected EOF reading f32".into());
        }
        let bytes: [u8; 4] = self.bytes[self.pos..self.pos + 4]
            .try_into()
            .map_err(|_| "Failed to read f32")?;
        self.pos += 4;
        Ok(f32::from_le_bytes(bytes))
    }

    fn read_string(&mut self, len: usize) -> Result<String, Box<dyn Error>> {
        if self.pos + len > self.bytes.len() {
            return Err(format!("Unexpected EOF reading string of length {}", len).into());
        }
        let s = String::from_utf8(self.bytes[self.pos..self.pos + len].to_vec())
            .map_err(|_| "Invalid UTF-8 in string")?;
        self.pos += len;
        Ok(s)
    }
}
