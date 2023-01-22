use core::panic;

use crate::lexer::{Token, is_keyword};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Ast {
    Number(i32),
    StringLiteral(String),
    FunctionDefinition {
        name: String,
        body: Vec<Ast>,
    },
    Operation(Token),
    Expressions(Vec<Ast>),
    FunctionCall(String),
    If {
        if_body: Vec<Ast>,
        else_body: Vec<Ast>,
    },
}

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser { tokens, pos: 0 }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub(crate) fn parse(&mut self) -> Ast {
        self.parse_expressions()
    }
    
    fn parse_expressions(&mut self) -> Ast {
        let mut expressions = vec![];
        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                Token::Colon => {
                    let function = self.parse_function();
                    expressions.push(function);
                },
                Token::Number(x) => expressions.push(Ast::Number(x)),
                Token::StringLiteral(ref x) => {
                    expressions.push(Ast::StringLiteral(x.clone()));
                },
                Token::Add => {
                    expressions.push(Ast::Operation(Token::Add));
                },
                Token::Sub => {
                    expressions.push(Ast::Operation(Token::Sub));
                },
                Token::Mul => {
                    expressions.push(Ast::Operation(Token::Mul));
                },
                Token::Div => {
                    expressions.push(Ast::Operation(Token::Div));
                },
                Token::Emit => {
                    expressions.push(Ast::Operation(Token::Emit));
                },
                Token::Dup => {
                    expressions.push(Ast::Operation(Token::Dup));
                },
                Token::Swap => {
                    expressions.push(Ast::Operation(Token::Swap));
                },
                Token::Drop => {
                    expressions.push(Ast::Operation(Token::Drop));
                },
                Token::Lt => {
                    expressions.push(Ast::Operation(Token::Lt));
                },
                Token::Gt => {
                    expressions.push(Ast::Operation(Token::Gt));
                },
                Token::Lte => {
                    expressions.push(Ast::Operation(Token::Lte));
                },
                Token::Gte => {
                    expressions.push(Ast::Operation(Token::Gte));
                },
                Token::Eq => {
                    expressions.push(Ast::Operation(Token::Eq));
                },
                Token::DoubleEq => {
                    expressions.push(Ast::Operation(Token::DoubleEq));
                },
                Token::Identifier(ref name) => {
                    if is_keyword(&name) {
                        expressions.push(self.parse_keyword(name.clone()))
                    }else {
                        expressions.push(Ast::FunctionCall(name.clone()));
                    }
                },
                Token::SemiColon => {
                    panic!("Unexpected semicolon");
                },
                _ => {
                    panic!("Unexpected token");
                }
            }
            
            self.advance();
        }
        Ast::Expressions(expressions)
    }

    fn parse_function(&mut self) -> Ast {
        let func_name: String;
        self.advance(); // advance past colon
        if let Token::Identifier(ref name) = self.tokens[self.pos] {
            func_name = name.clone();
        } else {
            panic!("Expected identifier after colon");
        }
        self.advance(); // advance past identifier
        let mut body = vec![];

        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                Token::Number(x) => body.push(Ast::Number(x)),
                Token::StringLiteral(ref x) => body.push(Ast::StringLiteral(x.clone())),
                Token::Add => body.push(Ast::Operation(Token::Add)),
                Token::Sub => body.push(Ast::Operation(Token::Sub)),
                Token::Mul => body.push(Ast::Operation(Token::Mul)),
                Token::Div => body.push(Ast::Operation(Token::Div)),
                Token::Emit => body.push(Ast::Operation(Token::Emit)),
                Token::Dup => body.push(Ast::Operation(Token::Dup)),
                Token::Swap => body.push(Ast::Operation(Token::Swap)),
                Token::Drop => body.push(Ast::Operation(Token::Drop)),
                Token::Lt => body.push(Ast::Operation(Token::Lt)),
                Token::Gt => body.push(Ast::Operation(Token::Gt)),
                Token::Lte => body.push(Ast::Operation(Token::Lte)),
                Token::Gte => body.push(Ast::Operation(Token::Gte)),
                Token::Eq => body.push(Ast::Operation(Token::Eq)),
                Token::DoubleEq => body.push(Ast::Operation(Token::DoubleEq)),
                Token::Identifier(ref name ) => body.push(Ast::FunctionCall(name.clone())),
                Token::SemiColon => break,
                Token::If => body.push(self.parse_if()),
                Token::Else => panic!("Unexpected else"),
                Token::Then => panic!("Unexpected then"),
                Token::Colon => panic!("Unexpected colon"),
            }
            self.advance();
        }

        Ast::FunctionDefinition {
            name: func_name,
            body: body,
        }
    }

    fn parse_if(&mut self) -> Ast {
        self.advance(); // advance past IF
        let mut if_body = vec![];
        let mut else_body = vec![];

        while self.pos < self.tokens.len() {
            self.advance(); // advance current token
            match self.tokens[self.pos-1] {
                Token::Number(x) => if_body.push(Ast::Number(x)),
                Token::StringLiteral(ref x) => if_body.push(Ast::StringLiteral(x.clone())),
                Token::Add => if_body.push(Ast::Operation(Token::Add)),
                Token::Sub => if_body.push(Ast::Operation(Token::Sub)),
                Token::Mul => if_body.push(Ast::Operation(Token::Mul)),
                Token::Div => if_body.push(Ast::Operation(Token::Div)),
                Token::Emit => if_body.push(Ast::Operation(Token::Emit)),
                Token::Dup => if_body.push(Ast::Operation(Token::Dup)),
                Token::Swap => if_body.push(Ast::Operation(Token::Swap)),
                Token::Drop => if_body.push(Ast::Operation(Token::Drop)),
                Token::Lt => if_body.push(Ast::Operation(Token::Lt)),
                Token::Gt => if_body.push(Ast::Operation(Token::Gt)),
                Token::Lte => if_body.push(Ast::Operation(Token::Lte)),
                Token::Gte => if_body.push(Ast::Operation(Token::Gte)),
                Token::Eq => if_body.push(Ast::Operation(Token::Eq)),
                Token::DoubleEq => if_body.push(Ast::Operation(Token::DoubleEq)),
                Token::Identifier(ref name ) => if_body.push(Ast::FunctionCall(name.clone())),
                Token::Else => else_body = self.parse_else(),
                Token::If => if_body.push(self.parse_if()),
                Token::Then => break,
                Token::Colon => if_body.push(self.parse_function()),
                Token::SemiColon => panic!("Unexpected semicolon"),
            }
        }


        Ast::If {
            if_body,
            else_body
        }
    }

    fn parse_keyword(&mut self, keyword: String) -> Ast {
        match keyword.as_str() {
            "IF" => {
                return self.parse_if();
            },
            "LOOP" => {
                return self.parse_loop();
            },
            _ => {
                panic!("Unexpected keyword: {}", keyword);
            }
        }
    }

    fn parse_loop(&mut self) -> Ast {
        todo!()
    }

    fn parse_else(&mut self) -> Vec<Ast> {
        let mut body = vec![];

        while self.pos < self.tokens.len() {
            self.advance(); // advance past current token
            match self.tokens[self.pos-1] {
                Token::Number(x) => body.push(Ast::Number(x)),
                Token::StringLiteral(ref x) => body.push(Ast::StringLiteral(x.clone())),
                Token::Add => body.push(Ast::Operation(Token::Add)),
                Token::Sub => body.push(Ast::Operation(Token::Sub)),
                Token::Mul => body.push(Ast::Operation(Token::Mul)),
                Token::Div => body.push(Ast::Operation(Token::Div)),
                Token::Emit => body.push(Ast::Operation(Token::Emit)),
                Token::Dup => body.push(Ast::Operation(Token::Dup)),
                Token::Swap => body.push(Ast::Operation(Token::Swap)),
                Token::Drop => body.push(Ast::Operation(Token::Drop)),
                Token::Lt => body.push(Ast::Operation(Token::Lt)),
                Token::Gt => body.push(Ast::Operation(Token::Gt)),
                Token::Lte => body.push(Ast::Operation(Token::Lte)),
                Token::Gte => body.push(Ast::Operation(Token::Gte)),
                Token::Eq => body.push(Ast::Operation(Token::Eq)),
                Token::DoubleEq => body.push(Ast::Operation(Token::DoubleEq)),
                Token::Identifier(ref name ) => body.push(Ast::FunctionCall(name.clone())),
                Token::SemiColon => panic!("Unexpected semicolon"),
                Token::If => body.push(self.parse_if()),
                Token::Else => panic!("Unexpected else"),
                Token::Then => break,
                Token::Colon => panic!("Unexpected colon"),
            }
        }
        body
    }
}
