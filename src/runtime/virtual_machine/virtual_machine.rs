use {
    crate::{
        backend::compiler::instructions::Instructions,
        runtime::virtual_machine::{
            pre_parsing::BytecodeLoader,
            value::Value::{self, Bool, Number, StringValue},
            variables::variable::Variable,
        },
    },
    std::{collections::HashMap, error::Error, string::String},
};

pub struct VM {
    pub ip: usize,
    pub stack: Vec<Value>,
    pub instructions: Vec<Instructions>,
    pub variables: HashMap<String, Variable>,
}

impl VM {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let instructions = BytecodeLoader::from_bytes(bytes)?;
        Ok(Self {
            ip: 0,
            stack: Vec::new(),
            instructions,
            variables: std::collections::HashMap::new(),
        })
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(bytes)
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            if self.ip >= self.instructions.len() {
                return Err("Unexpected EOF".into());
            }

            let current_instruction = self.instructions[self.ip].clone();
            match current_instruction {
                Instructions::Add => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    let result = match (left, right) {
                        (Number(a), Number(b)) => Number(a + b),
                        (StringValue(a), StringValue(b)) => StringValue(a + &b),
                        _ => {
                            return Err(
                                "Type error: '+' expects number+number or string+string".into()
                            );
                        }
                    };
                    self.stack.push(result);
                    self.ip += 1;
                }

                Instructions::Sub => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (left, right) {
                        (Number(a), Number(b)) => {
                            self.stack.push(Number(a - b));
                        }
                        _ => return Err("Type error: '-' expects numbers".into()),
                    }
                    self.ip += 1;
                }

                Instructions::Mul => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (left, right) {
                        (Number(a), Number(b)) => {
                            self.stack.push(Number(a * b));
                        }
                        _ => return Err("Type error: '*' expects numbers".into()),
                    }
                    self.ip += 1;
                }

                Instructions::Div => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (left, right) {
                        (Number(a), Number(b)) => {
                            if b == 0.0 {
                                return Err("Cannot divide by zero".into());
                            }
                            self.stack.push(Number(a / b));
                        }
                        _ => return Err("Type error: '/' expects numbers".into()),
                    }
                    self.ip += 1;
                }
                Instructions::Modulo => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (left, right) {
                        (Number(a), Number(b)) => {
                            self.stack.push(Number(a % b));
                        }
                        _ => return Err("Type error: '%' expects numbers".into()),
                    }
                    self.ip += 1;
                }
                Instructions::PushString(s) => {
                    self.stack.push(StringValue(s));
                    self.ip += 1;
                }

                Instructions::LoadVar(name) => {
                    let variable = self
                        .variables
                        .get(&name)
                        .ok_or_else(|| format!("Variable '{}' not found", name))?;
                    self.stack.push(variable.value.clone());
                    self.ip += 1;
                }

                Instructions::SaveVar(name) => {
                    let value = self.pop()?;
                    let var = Variable { value };
                    self.variables.insert(name, var);
                    self.ip += 1;
                }

                Instructions::PushBool(b) => {
                    self.stack.push(Bool(b));
                    self.ip += 1;
                }

                Instructions::PushNumber(n) => {
                    self.stack.push(Number(n));
                    self.ip += 1;
                }

                Instructions::WriteLnLastOnStack => {
                    let val = self.pop()?;
                    match val {
                        StringValue(s) => println!("{}", s),
                        Number(n) => println!("{}", n.to_string()),
                        _ => unreachable!(),
                    }
                    self.ip += 1;
                }

                Instructions::WriteLastOnStack => {
                    let val = self.pop()?;
                    match val {
                        StringValue(s) => print!("{}", s),
                        Number(n) => print!("{}", n),
                        _ => unreachable!(),
                    }
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    self.ip += 1;
                }

                Instructions::ProcessExit => match self.pop()? {
                    Number(n) => {
                        println!("Exited with code {}", n);
                        break;
                    }
                    _ => unreachable!(),
                },

                Instructions::JumpIfTrue(addr) => {
                    let cond = self.pop()?;
                    match cond {
                        Bool(true) => {
                            self.ip = addr;
                        }
                        Bool(false) => {
                            self.ip += 1;
                        }
                        _ => return Err("JumpIfFalse expects boolean".into()),
                    }
                }
                Instructions::Jump(addr) => {
                    self.ip = addr;
                }

                Instructions::JumpIfFalse(addr) => {
                    let cond = self.pop()?;
                    match cond {
                        Bool(false) => {
                            self.ip = addr;
                        }
                        Bool(true) => {
                            self.ip += 1;
                        }
                        _ => return Err("JumpIfFalse expects boolean".into()),
                    }
                }
                Instructions::GreaterThan => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (left, right) {
                        (Number(l), Number(r)) => {
                            self.stack.push(Bool(l > r));
                            self.ip += 1;
                        }
                        _ => return Err("GreaterThan expects numbers".into()),
                    }
                }
                Instructions::LessThan => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    match (left, right) {
                        (Number(l), Number(r)) => {
                            self.stack.push(Bool(l < r));
                            self.ip += 1;
                        }
                        _ => return Err("LessThan expects numbers".into()),
                    }
                }
                Instructions::Equal => {
                    let right = self.pop()?;
                    let left = self.pop()?;
                    if left == right {
                        self.stack.push(Bool(true));
                    } else {
                        self.stack.push(Bool(false));
                    }
                    self.ip += 1;
                }
                Instructions::ReadInput => {
                    let mut input = String::new();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    self.stack.push(StringValue(input.trim().to_string()));
                    self.ip += 1;
                }
                Instructions::Halt => {
                    if !self.stack.is_empty() {
                        println!("{:?}", self.stack[0]);
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Stack underflow".into())
    }
}
