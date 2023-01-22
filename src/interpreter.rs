use crate::context::Context;
use crate::stack_machine::{StackMachine, Entity};
use crate::parser::Ast;

pub(crate) struct Interpreter {
    stackMachine: StackMachine,
    context: Box<Context>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            stackMachine: StackMachine::new(),
            context: Box::new(Context::new(None)),
        }
    }

    pub(crate) fn run(&mut self, ast: &Ast) {
        self.interpret(&ast, &self.context);
    }

    fn interpret(&mut self, ast: &Ast, context: &Box<Context>) {
        match ast {
            Ast::Number(x) => self.stackMachine.push(Entity::Number(x.clone())),
            Ast::Operation(op) => self.stackMachine.execute(op),
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
            _ => {}
        }
    }
}
