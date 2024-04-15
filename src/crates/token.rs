use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    ASSIGN,
    PLUS,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    SEMICOLON,
    EOF,
    LET,
    IDENT,
    INT,
    FUNCTION,
    ILLEGAL,
    BANG,
    MINUS,
    SLASH,
    ASTERISK,
    LT,
    GT,
    IF,
    RETURN,
    TRUE,
    FALSE,
    ELSE,
    EQ,
    NOT_EQ,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub t: TokenType,
    pub ch: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "12 Token23({})", self.ch)
    }
}

lazy_static! {
    static ref KEY_WORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", TokenType::FUNCTION);
        m.insert("let", TokenType::LET);
        m.insert("if", TokenType::IF);
        m.insert("return", TokenType::RETURN);
        m.insert("true", TokenType::TRUE);
        m.insert("false", TokenType::FALSE);
        m.insert("else", TokenType::ELSE);
        m
    };
}

impl Token {
    pub fn look_up_ident(ident: &String) -> TokenType {
        match KEY_WORDS.get(ident.as_str()) {
            Some(value) => *value,
            None => TokenType::IDENT,
        }
    }
}
