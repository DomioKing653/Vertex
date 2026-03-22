use strum_macros::{EnumString, Display};

#[derive(EnumString, Display, Debug, Clone,PartialEq)]
pub enum ComptimeValueType {
    #[strum(serialize="int")]
    Int,

    #[strum(serialize="string")]
    StringValue,

    #[strum(serialize="bool")]
    Bool,

    #[strum(serialize="float")]
    Float,

    #[strum(serialize="void")]
    Void,

    #[strum(serialize="array{0}")]
    Array(Box<ComptimeValueType>),
}

impl Default for ComptimeValueType {
    fn default() -> Self {
        ComptimeValueType::Int
    }
}

impl From<ComptimeValueType> for String {
    fn from(t: ComptimeValueType) -> Self {
        t.to_string()     
    }
}

impl ComptimeValueType {
    pub fn from_str_safe(s: &str) -> Option<Self> {
        s.parse::<ComptimeValueType>().ok()
    }
}

