use core::panic;
use std::io::Write;

use crate::context::Context;
use crate::parser::Ast;
use crate::stack_machine::{Entity, StackMachine};

pub(crate) struct Interpreter {
    stack_machine: StackMachine,
    context: Context,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            stack_machine: StackMachine::new(),
            context: Context::new(None),
        }
    }

    pub(crate) fn run(&mut self, ast: &Ast, output: &mut impl Write) {
        self.interpret(&ast, &mut self.context.clone(), output);
    }

    fn interpret(&mut self, ast: &Ast, context: &mut Context, output: &mut impl Write) {
        match ast {
            Ast::Number(x) => self.stack_machine.push(Entity::Number(x.clone())),
            Ast::Operation(op) => self.stack_machine.execute(op, output),
            Ast::Expressions(nodes) => {
                for node in nodes {
                    self.interpret(node, context, output);
                }
            }
            Ast::WordDefinition { name, body } => {
                context.set(name.clone(), body.to_vec());
            }
            Ast::FunctionCall(ref name) => {
                let body = context.get(name.clone());
                self.interpret(&Ast::Expressions(body), context, output);
            }
            Ast::StringLiteral(x) => {
                let length = x.len();
                for i in 0..length {
                    self.stack_machine
                        .push(Entity::Number(x.chars().nth(i).unwrap() as i32));
                }
                self.stack_machine.push(Entity::Number(length as i32));
            }
            Ast::If { if_body, else_body } => {
                let condition = self.stack_machine.pop();
                match condition {
                    Some(Entity::Number(c)) => {
                        if c == 1 {
                            self.interpret(&Ast::Expressions(if_body.to_vec()), context, output);
                        } else {
                            self.interpret(&Ast::Expressions(else_body.to_vec()), context, output);
                        }
                    }
                    Some(_) => panic!("Cannot use non Number value as condition"),
                    None => panic!("No conditional value for if"),
                }
            }
        }
    }
}
