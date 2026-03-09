#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //MATH
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    LEFTPAREN,
    RIGHTPAREN,
    OPENINGBRACE,
    CLOSINGBRACE,
    EQUAL,
    MODULO,
    //BOOLEAN
    GREATER,
    LESS,
    //VALUES
    NUMB,
    FLOAT,
    STRING,
    IDENTIFIER,
    //MISC
    COMMA,
    COLON,
    SEMICOLON,
    //VALUES
    TRUE,
    FALSE,
    //KEYWORDS
    FNC,
    VAR,
    CONST,
    STR,
    IF,
    ELSE,
    LOOP,
    WHILE,
    UNDEF,
    AS,
    USE,
    EXP,
    //EOF
    EOF,
    VALUE,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub token_value: String,
}
