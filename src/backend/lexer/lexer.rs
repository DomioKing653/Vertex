use crate::backend::lexer::tokens::TokenKind::{COMMA, FALSE, SEMICOLON, TRUE};
use crate::{
    backend::errors::lexer_errors::LexerErrorKind,
    backend::lexer::tokens::{
        Token, TokenKind,
        TokenKind::{
            AS, CLOSINGBRACE, COLON, CONST, DIVIDE, ELSE, EOF, ASSIGN, FLOAT, FNC, IDENTIFIER, IF,
            LEFTPAREN, LOOP, MINUS, MODULO, NUMB, OPENINGBRACE, PLUS, RIGHTPAREN, STR, TIMES, VAR,
            WHILE,EQUAL
        },
    },
};
use crate::backend::errors::lexer_errors::LexerError;
pub struct Lexer {
    current_char: char,
    token_idx: usize,
    token_count: usize,
    source_text: Vec<char>,
    final_tokens: Vec<Token>,
    current_line_char:usize,
    current_line:usize,
    errors:Vec<LexerErrorKind>,

}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            token_idx: 0,
            token_count: text.len(),
            current_char: '0',
            source_text: text.chars().collect(),
            final_tokens: Vec::new(),
            current_line_char:1,
            current_line:1,
            errors:Vec::new()
            
        }
    }
    pub fn tokenize(&mut self) -> Result<&Vec<Token>, LexerError> {
        if self.source_text.is_empty() {
            return Err(LexerError{err: LexerErrorKind::EmptyFile.into(),line:0,char:0});
        }
        self.current_char = self.source_text[0];
        while self.current_char != '\0' {
            match self.current_char {
                ' ' | '\n' | '\t' | '\r' => {
                    if self.current_char == '\n'{
                        self.current_line += 1;
                        self.current_line_char = 0;
                    }
                    self.advance();
                    continue;
                }
                '"' => {
                    let token = self.read_string()?;
                    self.final_tokens.push(token);
                    continue;
                }
                ':' => self.final_tokens.push(Token {
                    token_kind: COLON,
                    token_value: self.current_char.to_string(),
                }),
                '+' => self.final_tokens.push(Token {
                    token_kind: PLUS,
                    token_value: self.current_char.to_string(),
                }),
                ',' => self.final_tokens.push(Token {
                    token_kind: COMMA,
                    token_value: self.current_char.to_string(),
                }),
                ';' => self.final_tokens.push(Token {
                    token_kind: SEMICOLON,
                    token_value: self.current_char.to_string(),
                }),
                '=' => {
                    if self.source_text[self.token_idx+1]=='=' {
                        self.advance();
                        self.final_tokens.push(Token{
                            token_kind:EQUAL,
                            token_value:self.current_char.to_string()
                        });
                        
                    }else {
                        self.final_tokens.push(Token {
                            token_kind: ASSIGN,
                            token_value: self.current_char.to_string(),
                        })
                    }
                },
                '(' => self.final_tokens.push(Token {
                    token_kind: LEFTPAREN,
                    token_value: self.current_char.to_string(),
                }),
                ')' => self.final_tokens.push(Token {
                    token_kind: RIGHTPAREN,
                    token_value: self.current_char.to_string(),
                }),
                '{' => self.final_tokens.push(Token {
                    token_kind: OPENINGBRACE,
                    token_value: self.current_char.to_string(),
                }),
                '}' => self.final_tokens.push(Token {
                    token_kind: CLOSINGBRACE,
                    token_value: self.current_char.to_string(),
                }),
                '-' => self.final_tokens.push(Token {
                    token_kind: MINUS,
                    token_value: self.current_char.to_string(),
                }),
                '*' => self.final_tokens.push(Token {
                    token_kind: TIMES,
                    token_value: self.current_char.to_string(),
                }),
                '/' =>{
                    //FIXME:Add comments synatx
                    self.final_tokens.push(Token {
                    token_kind: DIVIDE,
                    token_value: self.current_char.to_string(),
                })

                },
                '%' => self.final_tokens.push(Token {
                    token_kind: MODULO,
                    token_value: self.current_char.to_string(),
                }),
                '>' => self.final_tokens.push(Token {
                    token_kind: TokenKind::GREATER,
                    token_value: self.current_char.to_string(),
                }),
                '<' => self.final_tokens.push(Token {
                    token_kind: TokenKind::LESS,
                    token_value: self.current_char.to_string(),
                }),
                _ => {
                    if self.current_char.is_alphabetic() {
                        let token = self.create_text_token();
                        self.final_tokens.push(token);
                        continue;
                    } else if self.current_char.is_numeric() {
                        let token = self.create_number_token()?;
                        self.final_tokens.push(token);
                        continue;
                    } else {
                        return Err(LexerError{err:LexerErrorKind::UnknownToken {
                                wrong_token: self.current_char.to_string(),
                            },
                            line:self.current_line,
                            char:self.current_line_char
                        }
                        .into());
                    }
                }
            }
            self.advance();
        }
        self.final_tokens.push(Token {
            token_kind: EOF,
            token_value: "EOF".to_string(),
        });
        Ok(&self.final_tokens)
    }
    //FIXME:Out of bounds error if í,é,č etc is in the identifier name

    fn advance(&mut self) {
        self.token_idx += 1;
        self.current_line_char += 1;
        if self.token_idx >= self.token_count {
            self.current_char = '\0';
        } else {
            self.current_char = self.source_text[self.token_idx];
        }
    }
    fn create_number_token(&mut self) -> Result<Token, LexerError> {
        let mut number_buffer: String = String::new();
        let mut dot_count: usize = 0;
        while self.current_char.is_numeric() || self.current_char == '.' {
            if self.current_char == '.' {
                if dot_count < 1 {
                    dot_count += 1;
                    number_buffer.push('.')
                } else {
                    return Err(LexerError{
                            err: LexerErrorKind::MoreDotInANumber,
                            line:self.current_line,
                            char:self.current_line_char,

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
        })
    }
    fn create_text_token(&mut self) -> Token {
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
            },
            "fnc" => Token {
                token_kind: FNC,
                token_value: text_buffer,
            },
            "str" => Token {
                token_kind: STR,
                token_value: text_buffer,
            },
            "const" => Token {
                token_kind: CONST,
                token_value: text_buffer,
            },
            "true" => Token {
                token_value: text_buffer,
                token_kind: TRUE,
            },
            "false" => Token {
                token_value: text_buffer,
                token_kind: FALSE,
            },
            "if" => Token {
                token_kind: IF,
                token_value: text_buffer,
            },
            "else" => Token {
                token_kind: ELSE,
                token_value: text_buffer,
            },
            "loop" => Token {
                token_kind: LOOP,
                token_value: text_buffer,
            },
            "while" => Token {
                token_kind: WHILE,
                token_value: text_buffer,
            },
            "as" => Token {
                token_kind: AS,
                token_value: text_buffer,
            },
            "undef" => Token {
                token_kind: TokenKind::UNDEF,
                token_value: text_buffer,
            },
            "use" => Token {
                token_kind: TokenKind::USE,
                token_value: text_buffer,
            },
            "exp"=> Token {
                token_kind:TokenKind::EXP,
                token_value: text_buffer,
            },
            "return"=>Token{
                token_kind:TokenKind::RETURN,
                token_value:text_buffer 
            },

            _ => Token {
                token_kind: IDENTIFIER,
                token_value: text_buffer,
            },
        }
    }

    fn read_string(&mut self) -> Result<Token, LexerError> {
        self.advance();
        let starting_line = self.current_line;
        let starting_char = self.current_line_char;

        let mut value = String::new();
        while self.current_char != '"' && self.current_char != '\0' {
            value.push(self.current_char);
            self.advance();
        }
        if self.current_char == '\0' {
            return Err(LexerError{
                err: LexerErrorKind::UnterminatedString { text: value },
                line:starting_line,
                char:starting_char,

            });
        }
        self.advance();
        Ok(Token {

            token_kind: TokenKind::STRING,
            token_value: value,
        })
    }
}
