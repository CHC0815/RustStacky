mod context;
mod interpreter;
mod lexer;
mod parser;
mod stack_machine;

use std::fs::read_to_string;

fn main() {
    // read file input as text and print it
    let input = read_to_string("test.f").unwrap();

    let tokens = lexer::Lexer::lex(&input);
    let ast = parser::Parser::new(&tokens).parse();
    println!("{:#?}", ast);
    let mut interprete = interpreter::Interpreter::new();
    interprete.run(&ast);
}
