use crate::stack_machine::StackMachine;
use crate::parser::Ast;

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
        
    }
}
