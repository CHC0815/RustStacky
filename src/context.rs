use std::collections::HashMap;

use crate::{parser::Ast, stack_machine::Entity};

#[derive(Debug, Clone)]
pub(crate) enum Variable {
    Function {body: Vec<Ast>},
    Variable (Entity),
}

#[derive(Debug, Clone)]
pub(crate) struct Context {
    parent: Option<Box<Context>>,
    variables: HashMap<String, Variable>,
}

impl Context {
    pub(crate) fn new(parent: Option<Box<Context>>) -> Self {
        Self {
            parent: parent,
            variables: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, name: String) -> Variable {
        match self.variables.get(&name) {
            Some(var) => var.clone(),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => panic!("Word not found"),
            },
        }
    }

    pub(crate) fn set(&mut self, name: String, var: Variable) {
        self.variables.insert(name, var);
    }
}
