// MATH
pub const ADD:u8 = 1;
pub const SUB:u8 = 2;
pub const DIV:u8 = 3;
pub const MUL:u8 = 4;
pub const MODULO:u8 = 5;

//COMP
pub const GREATER:u8 = 6;
pub const LESS:u8 = 7;
pub const EQUAL:u8 = 8;
//JUMPS
pub const JUMP:u8 =40;
pub const JUMP_IF_FALSE:u8 = 41;
pub const JUMP_IF_TRUE:u8 = 42;
pub const JUMP_LAST_ON_STACK:u8 = 43;

// VALS
pub const PUSH_STR:u8 = 20;
pub const PUSH_BOOL:u8 = 21;
pub const PUSH_NUMB:u8 = 22;
pub const PUSH_USIZE:u8=23;
pub const DROP:u8 = 125;
// IO
pub const WRITE_LN:u8 = 30;
pub const WRITE:u8 = 31;

pub const READ_INPUT:u8 = 60;
pub const PROCESS_EXIT:u8 = 61;

// VARS
pub const STORE_VAR:u8 = 50;
pub const LOAD_VAR:u8 = 51;
pub const ASSIGN_VAR:u8 = 52;
//HALT

pub const HALT:u8 = 255;
#[derive(Debug, Clone,PartialEq)]
pub enum Instructions {
    Add,
    Sub,
    Div,
    Mul,
    Modulo ,
    //Comparison
    GreaterThan,
    LessThan ,
    Equal,
    //Variables
    LoadVar(String),
    SaveVar(String),
    AssignVar(String),
    //Values
    PushString(String),
    PushBool(bool),
    PushNumber(f32),
    PushJmpAdress(usize),
    ReadInput,
    Drop(String),
    //Printing
    WriteLnLastOnStack,
    WriteLastOnStack,
    //Process
    ProcessExit,
    //Control flow
    JumpOnLastOnStack,
    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    // Halt
    Halt,
}
impl Instructions {
    pub fn opcode(&self) -> u8 {
        match self {
            Instructions::Add => ADD,
            Instructions::Sub => SUB,
            Instructions::Div => DIV,
            Instructions::Mul => MUL,
            Instructions::Modulo => MODULO,

            Instructions::GreaterThan => GREATER,
            Instructions::LessThan => LESS,
            Instructions::Equal => EQUAL,
            
            Instructions::PushString(_) => PUSH_STR,
            Instructions::PushBool(_) => PUSH_BOOL,
            Instructions::PushNumber(_) => PUSH_NUMB,
            Instructions::Drop(_) => DROP,
            Instructions::PushJmpAdress(_)=>PUSH_USIZE,

            Instructions::WriteLnLastOnStack => WRITE_LN,
            Instructions::WriteLastOnStack => WRITE,

            Instructions::ReadInput => READ_INPUT,
            Instructions::ProcessExit => PROCESS_EXIT,

            Instructions::LoadVar(_) => LOAD_VAR,
            Instructions::SaveVar(_) => STORE_VAR,
            Instructions::AssignVar(_) => ASSIGN_VAR,

            Instructions::Jump(_) => JUMP,
            Instructions::JumpIfFalse(_) => JUMP_IF_FALSE,
            Instructions::JumpIfTrue(_) => JUMP_IF_TRUE,
            Instructions::JumpOnLastOnStack=>JUMP_LAST_ON_STACK,
            Instructions::Halt => HALT,
            
        }
    }
}
