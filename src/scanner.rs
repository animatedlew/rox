use literals::LiteralType;
use rox::Rox;
use token::Token;
use tokens::TokenType;

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
        self.add_token(TokenType::Eof, LiteralType::Eof);
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
            '(' => self.add_token(TokenType::LeftParen, LiteralType::Text("(")),
            ')' => self.add_token(TokenType::RightParen, LiteralType::Text(")")),
            '{' => self.add_token(TokenType::LeftBrace, LiteralType::Text("{")),
            '}' => self.add_token(TokenType::RightBrace, LiteralType::Text("}")),
            ',' => self.add_token(TokenType::Comma, LiteralType::Text(",")),
            '.' => self.add_token(TokenType::Dot, LiteralType::Text(".")),
            '-' => self.add_token(TokenType::Minus, LiteralType::Text("-")),
            '+' => self.add_token(TokenType::Plus, LiteralType::Text("+")),
            ';' => self.add_token(TokenType::Semicolon, LiteralType::Text(";")),
            '*' => self.add_token(TokenType::Star, LiteralType::Text("*")),
            '!' => if self._match('=') {
                self.add_token(TokenType::BangEqual, LiteralType::Text("!="))
            } else {
                self.add_token(TokenType::Bang, LiteralType::Text("!"))
            },
            '=' => if self._match('=') {
                self.add_token(TokenType::EqualEqual, LiteralType::Text("=="))
            } else {
                self.add_token(TokenType::Equal, LiteralType::Text("="))
            },
            '<' => if self._match('=') {
                self.add_token(TokenType::LessEqual, LiteralType::Text("<="))
            } else {
                self.add_token(TokenType::Less, LiteralType::Text("<"))
            },
            '>' => if self._match('=') {
                self.add_token(TokenType::GreaterEqual, LiteralType::Text(">="))
            } else {
                self.add_token(TokenType::Greater, LiteralType::Text(">"))
            },
            '/' => {
                let mut comment = String::new();
                if self._match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        comment.push(self.advance());
                    }
                    self.add_token(
                        TokenType::Comment,
                        LiteralType::Custom(comment.as_str().trim().to_string()),
                    );
                } else {
                    self.add_token(TokenType::Slash, LiteralType::Text("/"));
                }
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            c @ _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier(c, rox);
                } else {
                    Rox::error(self.line, format!("Unrecognized character: {}", c), rox);
                }
            }
        }
    }
    fn identifier(&mut self, c: char, rox: &mut Rox) {
        let mut id = String::new();
        id.push(c);
        while self.is_alphanumeric(self.peek()) {
            id.push(self.advance());
        }
        let _type = rox
            .keywords
            .get(id.as_str())
            .unwrap_or(&TokenType::Identifier);
        self.add_token(*_type, LiteralType::Custom(id));
    }
    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let n = self.parse_double(self.source[self.start..self.current].to_string());
        self.add_token(TokenType::Number, LiteralType::Number(n));
    }
    fn parse_double(&self, n: String) -> f64 {
        n.parse().unwrap()
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.get_source_chars().len() {
            '\0'
        } else {
            self.get_source_char(self.current + 1)
        }
    }
    fn add_token(&mut self, _type: TokenType, literal: LiteralType) {
        self.tokens.push(Token {
            _type: _type,
            lexeme: None,
            literal: literal,
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
            self.add_token(TokenType::String, LiteralType::Custom(content));
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
