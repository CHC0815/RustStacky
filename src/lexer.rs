use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Number(i32),
    StringLiteral(String),
    Identifier(String),
    Add,
    Sub,
    Mul,
    Div,
    Emit,
    If,
    Dup,
    Swap,
    Drop,
    Else,
    Then,
    Lt,
    Do,
    Loop,
    Gt,
    Lte,
    Gte,
    Eq,
    DoubleEq,
    Colon,
    SemiColon,
    Puts,
    EOF,
}

pub(crate) struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new() -> Lexer<'a> {
        Lexer {
            chars: "".chars().peekable(),
        }
    }

    pub fn lex(&mut self, input: &'a str) -> Vec<Token> {
        self.chars = input.chars().peekable();

        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
                continue;
            }

            if c.is_numeric() {
                return Some(self.number());
            }

            if c.is_ascii_alphabetic() {
                return Some(self.identifier());
            }

            match c {
                '"' => return Some(self.string()),
                '>' => return Some(self.greater_than()),
                '<' => return Some(self.less_than()),
                '=' => return Some(self.equal()),
                _ => {}
            }

            // parse single character tokens
            self.chars.next();
            match c {
                '+' => return Some(Token::Add),
                '-' => return Some(Token::Sub),
                '*' => return Some(Token::Mul),
                '/' => return Some(Token::Div),
                '.' => return Some(Token::Emit),
                ':' => return Some(Token::Colon),
                ';' => return Some(Token::SemiColon),
                _ => {}
            }
        }

        None
    }

    fn number(&mut self) -> Token {
        let mut number = String::default();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() {
                number.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        Token::Number(number.parse().unwrap())
    }

    fn string(&mut self) -> Token {
        self.chars.next(); // consume the initial "
        let mut string = String::default();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                self.chars.next(); // consume the final "
                return Token::StringLiteral(string);
            } else {
                string.push(c);
                self.chars.next();
            }
        }
        Token::StringLiteral(string)
    }

    fn identifier(&mut self) -> Token {
        let mut identifier = String::default();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        // check if the identifier is a keyword
        match identifier.as_str() {
            "IF" => return Token::If,
            "ELSE" => return Token::Else,
            "THEN" => return Token::Then,
            "DUP" => return Token::Dup,
            "SWAP" => return Token::Swap,
            "DROP" => return Token::Drop,
            "DO" => return Token::Do,
            "LOOP" => return Token::Loop,
            "PUTS" => return Token::Puts,
            _ => {}
        }
        Token::Identifier(identifier)
    }

    fn greater_than(&mut self) -> Token {
        self.chars.next(); // consume the initial >
        if let Some(&c) = self.chars.peek() {
            if c == '=' {
                self.chars.next(); // consume the =
                return Token::Gte;
            }
        }
        Token::Gt
    }

    fn less_than(&mut self) -> Token {
        self.chars.next(); // consume the initial <
        if let Some(&c) = self.chars.peek() {
            if c == '=' {
                self.chars.next(); // consume the =
                return Token::Lte;
            }
        }
        Token::Lt
    }

    fn equal(&mut self) -> Token {
        self.chars.next(); // consume the initial =
        if let Some(&c) = self.chars.peek() {
            if c == '=' {
                self.chars.next(); // consume the =
                return Token::DoubleEq;
            }
        }
        Token::Eq
    }
}
