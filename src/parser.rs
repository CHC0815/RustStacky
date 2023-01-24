use core::panic;

use crate::lexer::{Token, is_keyword};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Ast {
    Number(i32),
    StringLiteral(String),
    WordDefinition {
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
    pub(crate) fn new() -> Parser<'a> {
        Parser { tokens: &vec!(), pos: 0 }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub(crate) fn parse(&mut self, tokens: &'a [Token]) -> Ast {
        self.tokens = tokens;
        
        let mut nodes = vec![];

        loop {
            let token = self.tokens[self.pos].clone();
            if token == Token::EOF {
                break;
            }
            nodes.push(self.get_node(token));
        }

        Ast::Expressions(nodes)
    }

    fn _is_conditional(&mut self, token: Token) -> bool {
        [Token::Gt, Token::Gte, Token::Lt, Token::Lte, Token::Eq].contains(&token)
    }

    fn get_node(&mut self, token: Token) -> Ast {
        self.advance();
        match token {
            Token::Number(x) => Ast::Number(x),
            Token::Add => Ast::Operation(Token::Add),
            Token::Sub => Ast::Operation(Token::Sub),
            Token::Emit => Ast::Operation(Token::Emit),
            Token::Mul => Ast::Operation(Token::Mul),
            Token::Div => Ast::Operation(Token::Div),
            Token::Dup => Ast::Operation(Token::Dup),
            Token::Swap => Ast::Operation(Token::Swap),
            Token::Drop => Ast::Operation(Token::Drop),
            Token::Puts => Ast::Operation(Token::Puts),
            Token::Eq => Ast::Operation(Token::Eq),
            Token::StringLiteral(ref x) => Ast::StringLiteral(x.clone()),
            Token::Identifier(ref x) => Ast::FunctionCall(x.clone()),
            Token::Colon => self.get_word(),
            Token::If => self.get_if(),
            Token::SemiColon => {
                panic!("Unexpected semicolon")
            },
            _ => panic!("Not yet implemented {:?}", token),
        }
    }

    fn get_if(&mut self) -> Ast {
        let mut if_body = vec![];
        let mut else_body = vec![];
        
        let mut token = self.tokens[self.pos].clone();
        while token != Token::Then && token != Token::Else {
            if_body.push(self.get_node(token.clone()));
            token.clone_from(&self.tokens[self.pos])
        }
        if token == Token::Else {
            self.advance(); // advance past else
            token.clone_from(&self.tokens[self.pos]);
            while token != Token::Then {
                else_body.push(self.get_node(token.clone()));
                token.clone_from(&self.tokens[self.pos]);
            }
        }
        self.advance(); // advance past then
        
        Ast::If { if_body, else_body }
    }

    fn get_word(&mut self) -> Ast {
        let mut body = vec![];
        let mut name: String = "undefined".to_string();
        let mut token = self.tokens[self.pos].clone();
        if let Token::Identifier(ref x) = token {
            if is_keyword(x) {
                panic!("Cannot use keyword as identifier");
            }
            name.clone_from(&x);
        } else {
            panic!("Expected identifier");
        }
        self.advance(); // advance past identifier
        token.clone_from(&self.tokens[self.pos]);
        
        while token != Token::SemiColon {
            body.push(self.get_node(token.clone()));
            token.clone_from(&self.tokens[self.pos]);
        }
        self.advance(); // advance past semicolon
        Ast::WordDefinition { name, body }
    }

}