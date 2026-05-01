#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f32),
    Bool(bool),
    Usize(usize),
    Array(Vec<Value>),
}
