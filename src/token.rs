use std::fmt;
use token_type::TokenType;

pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: String,
    line: u32,
}

impl Token {
    pub fn new(literal: String) -> Token {
        Token {
            literal: literal.clone(),
            lexeme: literal.clone(),
            _type: TokenType::Eof,
            line: 1,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self._type, self.lexeme, self.literal)
    }
}
