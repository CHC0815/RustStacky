use crate::{
    interpreter::Interpreter,
    lexer::{Lexer, Token},
    parser::{Ast, Parser},
};

pub(crate) struct Stacky<'a> {
    lexer: Lexer<'a>,
    parser: Parser<'a>,
    interpreter: Interpreter,
}

impl<'a> Stacky<'a> {
    pub(crate) fn new() -> Self {
        Self {
            lexer: Lexer::new(),
            parser: Parser::new(),
            interpreter: Interpreter::new(),
        }
    }

    pub(crate) fn lex(&mut self, input: &'a str) -> Vec<Token> {
        self.lexer.lex(input)
    }

    pub(crate) fn parse(&mut self, tokens: &'a [Token]) -> Ast {
        self.parser.parse(tokens)
    }

    pub(crate) fn run(&mut self, ast: &Ast) {
        self.interpreter.run(ast);
    }
}
