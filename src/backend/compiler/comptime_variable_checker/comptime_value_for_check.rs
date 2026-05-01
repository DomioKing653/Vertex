use strum_macros::{Display, EnumString};

//FIXME:This will probalby be neede to rework later but for now it works
#[derive(EnumString, Display, Debug, Clone, PartialEq, Default)]
pub enum ComptimeValueType {
    #[strum(serialize = "int")]
    #[default]
    Int,

    #[strum(serialize = "string")]
    StringValue,

    #[strum(serialize = "bool")]
    Bool,

    #[strum(serialize = "float")]
    Float,

    #[strum(serialize = "void")]
    Void,

    #[strum(serialize = "array{0}")]
    Array(Box<ComptimeValueType>),
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
