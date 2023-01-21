use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Number(i32),
    StringLiteral(String),
    Identifier(String),
    Add,
    Sub,
    Mul,
    Div,
    Emit,
    Dup,
    Swap,
    Drop,
    Lt,
    Gt,
    Lte,
    Gte,
    Eq,
    DoubleEq,
}

pub(crate) struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn lex(input: &'a str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = vec![];
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
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

            if c == '"' {
                return Some(self.string());
            }

            if c == '>' {
                return Some(self.greater_than());
            }

            if c == '<' {
                return Some(self.less_than());
            }

            if c == '=' {
                return Some(self.equal());
            }

            // parse single character tokens
            self.chars.next();
            match c {
                '+' => return Some(Token::Add),
                '-' => return Some(Token::Sub),
                '*' => return Some(Token::Mul),
                '/' => return Some(Token::Div),
                '.' => return Some(Token::Emit),
                _ => return None,
            }
        }

        None
    }

    fn number(&mut self) -> Token {
        let mut number = String::new();
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
        let mut string = String::new();
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
        let mut identifier = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.chars.next();
            } else {
                break;
            }
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