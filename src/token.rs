use std::fmt;
use token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: Option<String>,
    pub literal: Option<String>, // FIXME: set this to an enum that can accept several types
    pub line: usize,
}

impl Token {}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self._type, self.lexeme, self.literal)
    }
}
