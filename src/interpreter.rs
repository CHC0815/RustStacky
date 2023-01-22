use crate::context::Context;
use crate::stack_machine::{StackMachine, Entity};
use crate::parser::Ast;

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

    pub(crate) fn run(&mut self, ast: &Ast) {
        self.interpret(&ast, &mut self.context.clone());
    }

    fn interpret(&mut self, ast: &Ast, context: &mut Context) {
        match ast {
            Ast::Number(x) => self.stack_machine.push(Entity::Number(x.clone())),
            Ast::Operation(op) => self.stack_machine.execute(op),
            Ast::Expressions(nodes) => {
                for node in nodes {
                    self.interpret(node, context);
                }
            }
            Ast::WordDefinition { name, body } => {
                context.set(name.clone(), body.to_vec());
            }
            Ast::FunctionCall(ref name) => {
                let body = context.get(name.clone());
                self.interpret(&Ast::Expressions(body), context);
            }
            Ast::StringLiteral(x) => {
                let length = x.len();
                for i in 0..length {
                    self.stack_machine.push(Entity::Number(x.chars().nth(i).unwrap() as i32));
                }
                self.stack_machine.push(Entity::Number(length as i32));
            },
            _ => {}
        }
    }
}
