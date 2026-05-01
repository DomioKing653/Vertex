/*! Custom formatter with configurable tag delimiter.

New syntax (default delimiter: `|`, configurable via `format_color_with_delim`):
- Terminated tags: `$<tag><delim>`
  - Builtin: `$red|`, `$green|`, `$reset|`
  - RGB: `$.255/0/0|`

Behavior:
- Tag delimiter is configurable. Use `format_color(input)` for the default delimiter (`'|'`)
  or `format_color_with_delim(input, delim)` to supply a custom delimiter (e.g. `'#'`).
- Color tags are prefix-style and hold context until the next color tag.
  Example: `$red| hello $green| world` — "hello" is red, "world" is green.
- `$reset|` (or `$reset<delim>`) resets the current color context (emits ANSI reset).
- Escape literal: `$$` → `$` (an escaped `$` prevents a following sequence from being parsed as a tag).

Validation and errors:
- Unknown builtin → `ColorError::UnknownBuiltin`
- RGB numbers must be 0..255 → `ColorError::OutOfRange`
- RGBA (4-component) is rejected → `ColorError::InvalidFormat`
- Malformed tags (e.g. unterminated) → `ColorError::InvalidFormat`

CREATOR: Lightmayo (Alias) — refactored.
*/
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    // kept for compatibility with existing code that may expect an alpha field;
    // parser rejects rgba so this remains unused for parsing.
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn to_ansi_fg(self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone)]
pub enum ColorError {
    EmptyTag,
    UnknownBuiltin(String),
    InvalidFormat(String),
    InvalidNumber(String),
    OutOfRange(String),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::EmptyTag => write!(f, "empty color tag"),
            ColorError::UnknownBuiltin(s) => write!(f, "unknown builtin color: {s}"),
            ColorError::InvalidFormat(s) => write!(f, "invalid format: {s}"),
            ColorError::InvalidNumber(s) => write!(f, "invalid number: {s}"),
            ColorError::OutOfRange(s) => write!(f, "out of range: {s}"),
        }
    }
}

impl std::error::Error for ColorError {}

#[derive(Debug, Clone, Copy)]
enum Control {
    Fg(Color),
    Reset,
}

impl Control {
    fn to_ansi(self) -> String {
        match self {
            Control::Fg(c) => c.to_ansi_fg(),
            Control::Reset => "\x1b[0m".into(),
        }
    }
}

/// Format using default delimiter `|`.
pub fn format_color(input: &str) -> Result<String, ColorError> {
    format_color_with_delim(input, '|')
}

/// Format the input string, replacing `$...<delim>` color tags with ANSI color sequences.
///
/// `delim` is the character that terminates a tag. Examples:
/// - default: `delim = '|'` -> `$red|`, `$.255/0/0|`
/// - custom: `delim = '#'` -> `$red#`, `$.255/0/0#`
///
/// Returns an error for malformed tags, unknown builtins, invalid numbers, or rgba usage.
pub fn format_color_with_delim(input: &str, delim: char) -> Result<String, ColorError> {
    let mut out = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            // Escape literal `$$` => single `$`
            if let Some(&'$') = chars.peek() {
                chars.next();
                out.push('$');
                continue;
            }

            // Collect until terminating `delim`
            let mut tag = String::new();
            let mut closed = false;
            for next_ch in chars.by_ref() {
                if next_ch == delim {
                    closed = true;
                    break;
                }
                tag.push(next_ch);
            }

            if !closed {
                return Err(ColorError::InvalidFormat("unterminated tag".into()));
            }

            let tag = tag.trim();
            if tag.is_empty() {
                return Err(ColorError::EmptyTag);
            }

            // Parse tag and emit ANSI sequence (or reset).
            let ctrl = parse_tag(tag)?;
            out.push_str(&ctrl.to_ansi());
        } else {
            out.push(ch);
        }
    }

    Ok(out)
}

fn parse_tag(tag: &str) -> Result<Control, ColorError> {
    let tag = tag.trim();
    if tag.is_empty() {
        return Err(ColorError::EmptyTag);
    }

    if tag == "reset" {
        return Ok(Control::Reset);
    }

    // RGB syntax: starts with '.' and uses `/` separators, e.g. `.255/0/0`
    if let Some(rest) = tag.strip_prefix('.') {
        let parts: Vec<&str> = rest.split('/').collect();
        match parts.len() {
            3 => {
                let r = parse_u8(parts[0])?;
                let g = parse_u8(parts[1])?;
                let b = parse_u8(parts[2])?;
                return Ok(Control::Fg(Color::rgb(r, g, b)));
            }
            4 => {
                // Explicitly reject rgba (alpha unsupported in ANSI)
                return Err(ColorError::InvalidFormat("rgba not supported".into()));
            }
            _ => return Err(ColorError::InvalidFormat(tag.into())),
        }
    }

    // builtin names
    builtin(tag)
        .map(Control::Fg)
        .ok_or_else(|| ColorError::UnknownBuiltin(tag.into()))
}

#[macro_export]
macro_rules! clrprintln {
    ($s:expr) => {
        println!("{}", $crate::backend::errors::formatter::colorformat::format_color($s).unwrap());
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!("{}", $crate::backend::errors::formatter::colorformat::format_color(&format!($fmt, $($arg)*)).unwrap());
    };
}

#[macro_export]
macro_rules! clrprint {
    ($s:expr) => {
        print!("{}", $crate::backend::errors::formatter::colorformat::format_color($s).unwrap());
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!("{}", $crate::backend::errors::formatter::colorformat::format_color(&format!($fmt, $($arg)*)).unwrap());
    };
}
fn builtin(name: &str) -> Option<Color> {
    match name {
        "red" => Some(Color::rgb(239, 68, 68)),
        "green" => Some(Color::rgb(0, 255, 0)),
        "sgreen" => Some(Color::rgb(128, 255, 128)),
        "blue" => Some(Color::rgb(0, 0, 255)),
        "yellow" => Some(Color::rgb(255, 255, 0)),
        "cyan" => Some(Color::rgb(0, 255, 255)),
        "magenta" => Some(Color::rgb(255, 0, 255)),
        "white" => Some(Color::rgb(255, 255, 255)),
        "black" => Some(Color::rgb(0, 0, 0)),
        "gray" => Some(Color::rgb(128, 128, 128)),
        _ => None,
    }
}

fn parse_u8(s: &str) -> Result<u8, ColorError> {
    let n = s
        .trim()
        .parse::<i32>()
        .map_err(|_| ColorError::InvalidNumber(s.into()))?;
    if !(0..=255).contains(&n) {
        return Err(ColorError::OutOfRange(s.into()));
    }
    Ok(n as u8)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_default_delim() {
        let s = format_color("$.255/0/0|").unwrap();
        assert!(s.contains("\x1b[38;2;255;0;0m"));
    }

    #[test]
    fn test_rgba_rejected_default_delim() {
        assert!(format_color("$.255/0/0/0.5|").is_err());
    }

    #[test]
    fn test_unknown_builtin_default_delim() {
        assert!(format_color("$unknown|").is_err());
    }

    #[test]
    fn test_hold_context_default_delim() {
        let s = format_color("$red| hello $green| world").unwrap();
        let red_seq = "\x1b[38;2;255;0;0m";
        let green_seq = "\x1b[38;2;0;255;0m";
        let i_red = s.find(red_seq).expect("red seq present");
        let i_green = s.find(green_seq).expect("green seq present");
        assert!(i_red < i_green, "red must come before green");
        // ensure text between them exists (hello)
        let between = &s[i_red + red_seq.len()..i_green];
        assert!(between.contains("hello"));
    }

    #[test]
    fn test_escape_dollar_default_delim() {
        assert_eq!(format_color("$$").unwrap(), "$");
        assert_eq!(format_color("$$dollar").unwrap(), "$dollar");
        // combining escape with a tag: $$red| should be treated as a literal "$red|"
        let s = format_color("$$red|").unwrap();
        assert_eq!(s, "$red|");
    }

    #[test]
    fn test_unterminated_dollar_default_delim() {
        assert!(format_color("$").is_err());
        assert!(format_color("$red").is_err());
    }

    #[test]
    fn test_reset_builtin_default_delim() {
        // red applied, then reset, then plain world
        let s = format_color("$red|hello$reset|world").unwrap();
        let red_seq = "\x1b[38;2;255;0;0m";
        let reset_seq = "\x1b[0m";
        let i_red = s.find(red_seq).expect("red seq present");
        let i_reset = s.find(reset_seq).expect("reset seq present");
        assert!(i_red < i_reset, "reset should come after red");
        // after reset there should be the plain text 'world' without a following color sequence
        let after_reset = &s[i_reset + reset_seq.len()..];
        assert!(after_reset.contains("world"));
        // ensure no trailing color sequence after reset and before world
        assert!(!after_reset.starts_with("\x1b[38;2;"));
    }

    #[test]
    fn test_custom_delimiter_hash() {
        // use '#' as delimiter: tags like $red#
        let s = format_color_with_delim("$red# hello $green# world", '#').unwrap();
        assert!(s.contains("\x1b[38;2;255;0;0m"));
        assert!(s.contains("\x1b[38;2;0;255;0m"));

        // rgb with custom delimiter
        let s2 = format_color_with_delim("$.0/128/255# test", '#').unwrap();
        assert!(s2.contains("\x1b[38;2;0;128;255m"));
    }

    #[test]
    fn test_escape_with_custom_delim() {
        // $$ escapes dollar; with custom delim the literal is not re-parsed as tag
        let s = format_color_with_delim("$$red#", '#').unwrap();
        assert_eq!(s, "$red#");
    }
}
