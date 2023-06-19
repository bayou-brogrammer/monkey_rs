#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,     // End of file
    Illegal, // Illegal token

    // Identifiers + literals
    Int(String),   // 123
    Ident(String), // foobar

    // Operators
    Plus,     // +
    Bang,     // !
    Minus,    // -
    Slash,    // /
    Assign,   // =
    Asterisk, // *

    // Comparison
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >

    // Delimiters
    Comma,     // ,
    Semicolon, // ;

    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // Keywords
    If,       // if
    Let,      // let
    True,     // true
    Else,     // else
    False,    // false
    Return,   // return
    Function, // fn
}
