use std::collections::HashMap;

use crate::parser::Ast;

#[derive(Debug, Clone)]
pub(crate) struct Context {
    parent: Option<Box<Context>>,
    words: HashMap<String, Vec<Ast>>,
}

impl Context {
    pub(crate) fn new(parent: Option<Box<Context>>) -> Self {
        Self {
            parent: parent,
            words: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, name: String) -> Vec<Ast> {
        match self.words.get(&name) {
            Some(word) => word.to_vec(),
            None => {
                match &self.parent {
                    Some(parent) => parent.get(name),
                    None => panic!("Word not found"),
                }
            }
        }
    }

    pub(crate) fn set(&mut self, name: String, body: Vec<Ast>) {
        self.words.insert(name, body);
    }

}