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
        self.add_token(TokenType::Eof, "\0".to_string());
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.get_source_chars().len()
    }
    pub fn print_tokens(&self) {
        println!("tokens: {:?}", self.tokens);
    }
    fn scan_token(&mut self, rox: &mut Rox) {
        match self.advance() {
            '"' => self.string(rox),
            '(' => self.add_token(TokenType::LeftParen, "(".to_string()),
            ')' => self.add_token(TokenType::RightParen, ")".to_string()),
            '{' => self.add_token(TokenType::LeftBrace, "{".to_string()),
            '}' => self.add_token(TokenType::RightBrace, "}".to_string()),
            ',' => self.add_token(TokenType::Comma, ",".to_string()),
            '.' => self.add_token(TokenType::Dot, ".".to_string()),
            '-' => self.add_token(TokenType::Minus, "-".to_string()),
            '+' => self.add_token(TokenType::Plus, "+".to_string()),
            ';' => self.add_token(TokenType::Semicolon, ";".to_string()),
            '*' => self.add_token(TokenType::Star, "*".to_string()),
            '!' => if self._match('=') {
                self.add_token(TokenType::BangEqual, "!=".to_string())
            } else {
                self.add_token(TokenType::Bang, "!".to_string())
            },
            '=' => if self._match('=') {
                self.add_token(TokenType::EqualEqual, "==".to_string())
            } else {
                self.add_token(TokenType::Equal, "=".to_string())
            },
            '<' => if self._match('=') {
                self.add_token(TokenType::LessEqual, "<=".to_string())
            } else {
                self.add_token(TokenType::Less, "<".to_string())
            },
            '>' => if self._match('=') {
                self.add_token(TokenType::GreaterEqual, ">=".to_string())
            } else {
                self.add_token(TokenType::Greater, ">".to_string())
            },
            '/' => {
                let mut comment = String::new();
                if self._match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        comment.push(self.advance());
                    }
                    self.add_token(TokenType::Comment, comment.as_str().trim().to_string());
                } else {
                    self.add_token(TokenType::Slash, "/".to_string());
                }
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            c @ _ => Rox::error(self.line, format!("Unrecognized character: {}", c), rox),
        }
    }
    fn add_token(&mut self, _type: TokenType, literal: String) {
        self.tokens.push(Token {
            _type: _type,
            lexeme: None,
            literal: Some(literal),
            line: self.line,
        });
    }
    fn string(&mut self, rox: &mut Rox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Rox::error(self.line, "Unterminated string.".to_string(), rox);
        } else {
            self.advance(); // The closing "
            let content = self.trim_quotes();
            self.add_token(TokenType::String, content)
        }
    }
    fn trim_quotes(&self) -> String {
        self.source[self.start + 1..self.current - 1].to_string()
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
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.get_source_char(self.current)
        }
    }
    fn get_source_char(&self, index: usize) -> char {
        self.get_source_chars()[index]
    }
    fn get_source_chars(&self) -> Vec<char> {
        self.source.chars().collect()
    }
}
