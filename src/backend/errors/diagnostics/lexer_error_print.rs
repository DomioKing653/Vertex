use crate::backend::errors::lexer_errors::LexerError;
use crate::clrprintln;

pub fn print_lexer_err(err: LexerError, src_file: String) {
    clrprintln!("$red|error:$reset| {}", err);

    let lines: Vec<&str> = src_file.lines().collect();
    let line_idx = err.line.saturating_sub(1);
    let col_idx = err.char.saturating_sub(1);

    let line_width = (err.line + 1).to_string().len();

    if let Some(prev) = line_idx.checked_sub(1).and_then(|i| lines.get(i)) {
        clrprintln!(
            "$cyan|{:>width$}$reset| | {}",
            line_idx,
            prev,
            width = line_width
        );
    }
    if let Some(line) = lines.get(line_idx) {
        let highlighted: String = line
            .chars()
            .enumerate()
            .map(|(i, ch)| {
                if i == col_idx {
                    format!("$red|{}$reset|", ch)
                } else {
                    ch.to_string()
                }
            })
            .collect();

        clrprintln!(
            "$cyan|{:>width$}$reset| | {}",
            err.line,
            highlighted,
            width = line_width
        );

        let mut pointer = String::new();
        for _ in 0..col_idx {
            pointer.push(' ');
        }
        pointer.push_str("$red|^$reset|");

        clrprintln!(
            "{:>width$} | {}",
            "",
            pointer,
            width = line_width
        );
    }

    if let Some(next) = lines.get(line_idx + 1) {
        clrprintln!(
            "$cyan|{:>width$}$reset| | {}",
            err.line + 1,
            next,
            width = line_width
        );
    }
}
