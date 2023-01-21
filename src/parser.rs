use core::panic;

use crate::lexer::Token;

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
                Token::Number(x) => {
                    expressions.push(Ast::Number(x));
                },
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
                    expressions.push(Ast::FunctionCall(name.clone()));
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
                Token::Number(x) => {
                    body.push(Ast::Number(x));
                },
                Token::StringLiteral(ref x) => {
                    body.push(Ast::StringLiteral(x.clone()));
                },
                Token::Add => {
                    body.push(Ast::Operation(Token::Add));
                },
                Token::Sub => {
                    body.push(Ast::Operation(Token::Sub));
                },
                Token::Mul => {
                    body.push(Ast::Operation(Token::Mul));
                },
                Token::Div => {
                    body.push(Ast::Operation(Token::Div));
                },
                Token::Emit => {
                    body.push(Ast::Operation(Token::Emit));
                },
                Token::Dup => {
                    body.push(Ast::Operation(Token::Dup));
                },
                Token::Swap => {
                    body.push(Ast::Operation(Token::Swap));
                },
                Token::Drop => {
                    body.push(Ast::Operation(Token::Drop));
                },
                Token::Lt => {
                    body.push(Ast::Operation(Token::Lt));
                },
                Token::Gt => {
                    body.push(Ast::Operation(Token::Gt));
                },
                Token::Lte => {
                    body.push(Ast::Operation(Token::Lte));
                },
                Token::Gte => {
                    body.push(Ast::Operation(Token::Gte));
                },
                Token::Eq => {
                    body.push(Ast::Operation(Token::Eq));
                },
                Token::DoubleEq => {
                    body.push(Ast::Operation(Token::DoubleEq));
                },
                Token::Identifier(ref name ) => {
                    body.push(Ast::FunctionCall(name.clone()));
                },
                Token::SemiColon => {
                    break;
                },
                _ => {
                    panic!("Unexpected token in function body");
                }
            }
            self.advance();
        }

        Ast::FunctionDefinition {
            name: func_name,
            body: body,
        }
    }
}
