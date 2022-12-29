#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // literals
    Identifier(String),
    Number(f64),

    // ops
    Binary,
    Unary,
    Op(char),

    // keywords (declare)
    Def,
    Var,
    Extern,

    // keywords (control)
    If,
    Then,
    Else,
    For,
    In,

    // symbols
    Comma,
    LParen,
    RParen,
    Comment,

    // misc
    Illegal(String),
    EOF,
}
