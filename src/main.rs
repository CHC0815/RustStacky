mod context;
mod interpreter;
mod lexer;
mod parser;
mod stack_machine;
mod stacky;
mod tests;

use std::{fs::{File, read_to_string}, io::Write};

fn main() {
    let emit_tokens = true;
    let emit_ast = true;

    // read file input as text and print it
    let text = read_to_string("test_data/test.f").unwrap();
    let mut stacky = stacky::Stacky::new();

    let tokens = stacky.lex(&text);
    if emit_tokens {
        let mut file = File::create("debug/tokens.txt").unwrap();
        file.write_fmt(format_args!("{:#?}", tokens)).unwrap();
    }

    let ast = stacky.parse(&tokens);
    if emit_ast {
        let mut file = File::create("debug/ast.txt").unwrap();
        file.write_fmt(format_args!("{:#?}", ast)).unwrap();
    }

    stacky.run(&ast, &mut std::io::stdout());
}
