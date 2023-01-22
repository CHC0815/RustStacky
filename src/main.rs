mod lexer;
mod parser;
mod interpreter;
mod stack_machine;
mod context;

use std::{fs::{read_to_string, File}, io::Write};

fn main() {
    let emit_tokens = true;
    let emit_ast = true;


    // read file input as text and print it
    let input = read_to_string("test.f").unwrap();

    let tokens = lexer::Lexer::lex(&input);
    if emit_tokens {
        let mut file = File::create("debug/tokens.txt").unwrap();
        file.write_fmt(format_args!("{:#?}", tokens)).unwrap();
    }

    let ast = parser::Parser::new(&tokens).parse();
    if emit_ast {
        let mut file = File::create("debug/ast.txt").unwrap();
        file.write_fmt(format_args!("{:#?}", ast)).unwrap();
    }

    let mut interprete = interpreter::Interpreter::new();
    interprete.run(&ast);
}
