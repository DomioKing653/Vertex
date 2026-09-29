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
    ASSIGN,
    MODULO,
    //BOOLEAN
    GREATER,
    LESS,
    EQUAL,
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
    RETURN,
    //EOF
    EOF,
    VALUE,
}

// Span is for tracking line, column, and file where error occurred.
#[derive(Clone, Debug)]
pub struct Span {
    pub file_name: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub token_value: String,
    pub span: Span,
}
