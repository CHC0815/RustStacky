use crate::parser::Ast;
use crate::stack_machine::{Entity, StackMachine};

pub(crate) struct Interpreter {
    stackMachine: StackMachine,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            stackMachine: StackMachine::new(),
        }
    }

    pub(crate) fn interpret(&mut self, ast: &Ast) {
        match ast {
            Ast::Number(x) => self.stackMachine.push(Entity::Number(x.clone())),
            Ast::Operation(op) => self.stackMachine.execute(op),
            Ast::Expressions(nodes) => {
                for node in nodes {
                    self.interpret(node);
                }
            }
            // Ast::WordDefinition (ref name, body: Vec<Ast>) => {

            // }
            Ast::FunctionCall(ref name) => {}
            _ => {}
        }
    }
}
