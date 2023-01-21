mod opcode;
mod lexer;

use std::fs::{self, read_to_string};

fn main() {
    // read file input as text and print it
    let input = read_to_string("test.f").unwrap();

    let mut tokens = lexer::Lexer::lex(&input);
    
    println!("{:?}", tokens);
}
