use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Number(i32),
    StringLiteral(String),
    Identifier(String),
    LoopVariable(u8),
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
    Arrow,
    At,
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

    fn require_whitespace(&mut self) {
        if let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                panic!("Expected whitespace");
            }
        }
        self.chars.next(); // consume whitespace
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
                continue;
            }

            if c.is_numeric() {
                return Some(self.number(false));
            }

            if c.is_ascii_alphabetic() {
                return Some(self.identifier());
            }

            match c {
                '"' => return Some(self.string()),
                '>' => return Some(self.greater_than()),
                '<' => return Some(self.less_than()),
                '=' => return Some(self.equal()),
                '-' => return Some(self.sub()),
                _ => {}
            }

            // parse single character tokens
            self.chars.next();
            match c {
                '+' => return Some(Token::Add),
                '*' => return Some(Token::Mul),
                '/' => return Some(Token::Div),
                '.' => return Some(Token::Emit),
                ':' => return Some(Token::Colon),
                ';' => return Some(Token::SemiColon),
                '@' => return Some(Token::At),
                _ => {}
            }
        }

        None
    }

    fn number(&mut self, is_negative: bool) -> Token {
        let mut number = String::default();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() {
                number.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let num = number.parse().unwrap();

        if is_negative {
            return Token::Number(num * -1);
        }
        Token::Number(num)
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
        self.require_whitespace();
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

        self.require_whitespace();

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
            "I" => return Token::LoopVariable(0),
            "J" => return Token::LoopVariable(1),
            "K" => return Token::LoopVariable(2),
            "L" => return Token::LoopVariable(3),
            "M" => return Token::LoopVariable(4),
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

    fn sub(&mut self) -> Token {
        self.chars.next(); // consume the initial -
        if let Some(&c) = self.chars.peek() {
            if c == '>' {
                self.chars.next(); // consume the >
                return Token::Arrow;
            } else if c.is_numeric() {
                return self.number(true);
            }
        }
        Token::Sub
    }
}
