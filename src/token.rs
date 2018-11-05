use literals::LiteralType;
use std::fmt;
use tokens::TokenType;

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: Option<String>,
    pub literal: LiteralType,
    pub line: usize,
}

impl Token {}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self._type, self.lexeme, self.literal)
    }
}
