#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    StringValue(String),
    Number(f32),
    Bool(bool),
    Usize(usize),
    Array(Vec<Value>),
}
