use crate::backend::errors::lexer_errors::LexerError;
use crate::backend::lexer::tokens::TokenKind::{COMMA, FALSE, SEMICOLON, TRUE};
use crate::{
    backend::errors::lexer_errors::LexerErrorKind,
    backend::lexer::tokens::{
        Span, Token, TokenKind,
        TokenKind::{
            AS, ASSIGN, CLOSINGBRACE, COLON, CONST, DIVIDE, ELSE, EOF, EQUAL, FLOAT, FNC,
            IDENTIFIER, IF, LEFTPAREN, LOOP, MINUS, MODULO, NUMB, OPENINGBRACE, PLUS, RIGHTPAREN,
            STR, TIMES, VAR, WHILE,
        },
    },
};
use std::path::PathBuf;

pub struct Lexer {
    current_char: char,
    token_idx: usize,
    token_count: usize,
    source_text: Vec<char>,
    final_tokens: Vec<Token>,
    current_line_char: usize,
    current_line: usize,
    errors: Vec<LexerErrorKind>,
    file_name: PathBuf,
}

impl Lexer {
    pub fn new(text: String, file_name: PathBuf) -> Self {
        Self {
            token_idx: 0,
            token_count: text.len(),
            current_char: '0',
            source_text: text.chars().collect(),
            final_tokens: Vec::new(),
            current_line_char: 1,
            current_line: 1,
            errors: Vec::new(),
            file_name,
        }
    }
    fn consume_char(&mut self) -> Result<(), LexerError> {
        Ok(())
    }
    fn consume_white_spaces(&mut self) -> Result<(), LexerError> {
        self.advance();
        Ok(())
    }

    // Function to create Span
    fn make_span(&self) -> Span {
        let span: Span = Span {
            file_name: self.file_name.clone(), // clone to get a ownership of file_name
            line: self.current_line,
            col: self.current_line_char,
        };
        // println!("Span for {}: {:?}", self.current_char, span); // Uncomment this if want to
        // check out what span looks like.
        span
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        if self.source_text.is_empty() {
            return Err(LexerError {
                err: LexerErrorKind::EmptyFile,
                // line: 0,
                // char: 0,
                span: self.make_span(),
            });
        }
        self.current_char = self.source_text[0];
        while self.current_char != '\0' {
            match self.current_char {
                ' ' | '\n' | '\t' | '\r' => {
                    self.consume_white_spaces()?;
                    continue;
                }
                '#' => {
                    self.consume_comment();
                }

                '"' => {
                    let token = self.read_string()?;
                    self.final_tokens.push(token);
                    continue;
                }
                ':' => self.final_tokens.push(Token {
                    token_kind: COLON,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '+' => self.final_tokens.push(Token {
                    token_kind: PLUS,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                ',' => self.final_tokens.push(Token {
                    token_kind: COMMA,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                ';' => self.final_tokens.push(Token {
                    token_kind: SEMICOLON,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '=' => {
                    if self.source_text[self.token_idx + 1] == '=' {
                        self.advance();
                        self.final_tokens.push(Token {
                            token_kind: EQUAL,
                            token_value: self.current_char.to_string(),
                            span: self.make_span(),
                        });
                    } else {
                        self.final_tokens.push(Token {
                            token_kind: ASSIGN,
                            token_value: self.current_char.to_string(),
                            span: self.make_span(),
                        })
                    }
                }
                '(' => self.final_tokens.push(Token {
                    token_kind: LEFTPAREN,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                ')' => self.final_tokens.push(Token {
                    token_kind: RIGHTPAREN,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '{' => self.final_tokens.push(Token {
                    token_kind: OPENINGBRACE,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '}' => self.final_tokens.push(Token {
                    token_kind: CLOSINGBRACE,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '-' => self.final_tokens.push(Token {
                    token_kind: MINUS,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '*' => self.final_tokens.push(Token {
                    token_kind: TIMES,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '/' => self.final_tokens.push(Token {
                    token_kind: DIVIDE,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '%' => self.final_tokens.push(Token {
                    token_kind: MODULO,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '>' => self.final_tokens.push(Token {
                    token_kind: TokenKind::GREATER,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                '<' => self.final_tokens.push(Token {
                    token_kind: TokenKind::LESS,
                    token_value: self.current_char.to_string(),
                    span: self.make_span(),
                }),
                _ => {
                    if self.current_char.is_alphabetic() {
                        let token = self.create_text_token();
                        self.final_tokens.push(token);
                        // println!("{:?}", self.final_tokens); // Uncomment this line for token dump
                        continue;
                    } else if self.current_char.is_numeric() {
                        let token = self.create_number_token()?;
                        self.final_tokens.push(token);
                        continue;
                    } else {
                        return Err(LexerError {
                            err: LexerErrorKind::UnknownToken {
                                wrong_token: self.current_char.to_string(),
                            },
                            // line: self.current_line,
                            // char: self.current_line_char,
                            span: self.make_span(),
                        });
                    }
                }
            }
            self.advance();
        }
        self.final_tokens.push(Token {
            token_kind: EOF,
            token_value: "EOF".to_string(),
            span: self.make_span(),
        });
        Ok(self.final_tokens)
    }

    fn consume_comment(&mut self) {
        self.advance();
        while self.current_char != '\n' {
            self.advance();
        }
        self.advance();
    }
    //FIXME:Out of bounds error if í,é,č etc is in the identifier name

    fn advance(&mut self) {
        if self.current_char == '\n' {
            self.current_line += 1;
            self.current_line_char = 1;
        } else {
            self.current_line_char += 1;
        }
        self.token_idx += 1;
        if self.token_idx >= self.token_count {
            self.current_char = '\0';
        } else {
            self.current_char = self.source_text[self.token_idx];
        }
    }
    fn create_number_token(&mut self) -> Result<Token, LexerError> {
        let span: Span = self.make_span(); // make_span at the start of number.
        let mut number_buffer: String = String::new();
        let mut dot_count: usize = 0;
        while self.current_char.is_numeric() || self.current_char == '.' {
            if self.current_char == '.' {
                if dot_count < 1 {
                    dot_count += 1;
                    number_buffer.push('.')
                } else {
                    return Err(LexerError {
                        err: LexerErrorKind::MoreDotInANumber,
                        // line: self.current_line,
                        // char: self.current_line_char,
                        span,
                    });
                }
            } else {
                number_buffer.push(self.current_char);
            }
            self.advance();
        }
        Ok(Token {
            token_kind: if dot_count < 1 { NUMB } else { FLOAT },
            token_value: number_buffer,
            span,
        })
    }
    fn create_text_token(&mut self) -> Token {
        let span: Span = self.make_span(); // make_span at the start of text.
        let mut text_buffer: String = String::new();
        while self.current_char.is_alphabetic()
            || self.current_char.is_numeric()
            || self.current_char == '!'
            || self.current_char == '_'
        {
            text_buffer.push(self.current_char);
            self.advance()
        }
        match text_buffer.as_str() {
            "var" => Token {
                token_kind: VAR,
                token_value: text_buffer,
                span,
            },
            "fnc" => Token {
                token_kind: FNC,
                token_value: text_buffer,
                span,
            },
            "str" => Token {
                token_kind: STR,
                token_value: text_buffer,
                span,
            },
            "const" => Token {
                token_kind: CONST,
                token_value: text_buffer,
                span,
            },
            "true" => Token {
                token_value: text_buffer,
                token_kind: TRUE,
                span,
            },
            "false" => Token {
                token_value: text_buffer,
                token_kind: FALSE,
                span,
            },
            "if" => Token {
                token_kind: IF,
                token_value: text_buffer,
                span,
            },
            "else" => Token {
                token_kind: ELSE,
                token_value: text_buffer,
                span,
            },
            "loop" => Token {
                token_kind: LOOP,
                token_value: text_buffer,
                span,
            },
            "while" => Token {
                token_kind: WHILE,
                token_value: text_buffer,
                span,
            },
            "as" => Token {
                token_kind: AS,
                token_value: text_buffer,
                span,
            },
            "undef" => Token {
                token_kind: TokenKind::UNDEF,
                token_value: text_buffer,
                span,
            },
            "use" => Token {
                token_kind: TokenKind::USE,
                token_value: text_buffer,
                span,
            },
            "exp" => Token {
                token_kind: TokenKind::EXP,
                token_value: text_buffer,
                span,
            },
            "return" => Token {
                token_kind: TokenKind::RETURN,
                token_value: text_buffer,
                span,
            },

            _ => Token {
                token_kind: IDENTIFIER,
                token_value: text_buffer,
                span,
            },
        }
    }

    fn read_string(&mut self) -> Result<Token, LexerError> {
        let span: Span = self.make_span(); // make_span before going inside the quotes.
        self.advance();

        let mut value = String::new();
        while self.current_char != '"' && self.current_char != '\0' {
            if self.current_char == '\\' {
                self.advance();
                match self.current_char {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\0' => break,
                    other => {
                        // Keep unrecognized escapes unchanged for compatibility.
                        value.push('\\');
                        value.push(other);
                    }
                }
            } else {
                value.push(self.current_char);
            }
            self.advance();
        }
        if self.current_char == '\0' || (self.current_line != span.line && self.current_char == '"')
        {
            return Err(LexerError {
                err: LexerErrorKind::UnterminatedString { text: value },
                span,
            });
        }
        self.advance();
        Ok(Token {
            token_kind: TokenKind::STRING,
            token_value: value,
            span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
        Lexer::new(source.to_owned(), PathBuf::from("test.vtx")).tokenize()
    }

    #[test]
    fn missing_opening_quote_reports_actual_unterminated_string_position() {
        let error = tokenize("writeLn!(Hello World!\");\nwrite!(\"Hello World!\");\n")
            .expect_err("missing quote must be rejected");

        assert!(matches!(
            error.err,
            LexerErrorKind::UnterminatedString { .. }
        ));
        assert_eq!((error.span.line, error.span.col), (2, 21));
        assert_eq!(
            error.err.to_string(),
            "Unterminated string: expected a closing double quote"
        );
    }

    #[test]
    fn multiline_string_preserves_value_and_tracks_following_token_position() {
        let tokens = tokenize("\"first\nsecond\" next").expect("valid multiline string");

        assert_eq!(tokens[0].token_kind, TokenKind::STRING);
        assert_eq!(tokens[0].token_value, "first\nsecond");
        assert_eq!((tokens[0].span.line, tokens[0].span.col), (1, 1));
        assert_eq!(tokens[1].token_kind, IDENTIFIER);
        assert_eq!((tokens[1].span.line, tokens[1].span.col), (2, 9));
    }

    #[test]
    fn whitespace_counts_each_newline_once() {
        for source in ["first\nnext", "first\r\nnext"] {
            let tokens = tokenize(source).expect("valid identifiers");
            assert_eq!((tokens[1].span.line, tokens[1].span.col), (2, 1));
        }
    }

    #[test]
    fn unterminated_string_reports_its_opening_quote() {
        let error = tokenize("  \"oops").expect_err("missing closing quote");

        assert!(matches!(
            error.err,
            LexerErrorKind::UnterminatedString { .. }
        ));
        assert_eq!((error.span.line, error.span.col), (1, 3));
    }

    #[test]
    fn escaped_newline_decodes_without_changing_source_line() {
        let tokens = tokenize(r#""hello\nworld" next"#).expect("valid escaped newline");

        assert_eq!(tokens[0].token_value, "hello\nworld");
        assert_eq!((tokens[1].span.line, tokens[1].span.col), (1, 16));
    }

    #[test]
    fn common_string_escapes_are_decoded() {
        let tokens = tokenize(r#""tab\treturn\rquote\"slash\\""#).expect("valid string escapes");

        assert_eq!(tokens[0].token_value, "tab\treturn\rquote\"slash\\");
    }

    #[test]
    fn escaped_backslash_keeps_literal_newline_sequence() {
        let tokens = tokenize(r#""hello\\nworld""#).expect("valid literal backslash");

        assert_eq!(tokens[0].token_value, r"hello\nworld");
    }

    #[test]
    fn unrecognized_escapes_are_preserved() {
        let tokens = tokenize(r#""path\q""#).expect("compatible unknown escape");

        assert_eq!(tokens[0].token_value, r"path\q");
    }

    #[test]
    fn unfinished_escape_and_escaped_closing_quote_are_unterminated() {
        for source in ["  \"oops\\", "  \"oops\\\""] {
            let error = tokenize(source).expect_err("missing unescaped closing quote");

            assert!(matches!(
                error.err,
                LexerErrorKind::UnterminatedString { .. }
            ));
            assert_eq!((error.span.line, error.span.col), (1, 3));
        }
    }
}
