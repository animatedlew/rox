use rox::Rox;
use token::Token;
use token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self, rox: &mut Rox) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(rox); // uses Rox::error
        }
        self.add_token(TokenType::Eof);
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.get_source_chars().len()
    }
    pub fn print_tokens(&self) {
        println!("tokens: {:?}", self.tokens);
    }
    fn scan_token(&mut self, rox: &mut Rox) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => if self._match('=') {
                self.add_token(TokenType::BangEqual)
            } else {
                self.add_token(TokenType::Bang)
            },
            '=' => if self._match('=') {
                self.add_token(TokenType::EqualEqual)
            } else {
                self.add_token(TokenType::Equal)
            },
            '<' => if self._match('=') {
                self.add_token(TokenType::LessEqual)
            } else {
                self.add_token(TokenType::Less)
            },
            '>' => if self._match('=') {
                self.add_token(TokenType::GreaterEqual)
            } else {
                self.add_token(TokenType::Greater)
            },
            c @ _ => Rox::error(self.line, format!("Unrecognized character: {}", c), rox),
        }
    }
    fn add_token(&mut self, _type: TokenType) {
        self.tokens.push(Token {
            _type: _type,
            lexeme: None,
            literal: None,
            line: self.line,
        });
    }
    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.get_source_char(self.current - 1)
    }
    fn _match(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_source_chars()[self.current] != c {
            return false;
        }
        self.current = self.current + 1;
        true
    }
    fn get_source_char(&self, index: usize) -> char {
        self.get_source_chars()[index]
    }
    fn get_source_chars(&self) -> Vec<char> {
        self.source.chars().collect()
    }
}
