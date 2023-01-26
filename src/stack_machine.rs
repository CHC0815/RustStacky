use std::io::Write;

use crate::{lexer::Token, parser::Ast};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Entity {
    Number(i32),
    String(String),
    Pointer(u32),
    Function(Ast),
}

pub(crate) struct StackMachine {
    stack: Vec<Entity>,
}

impl StackMachine {
    pub(crate) fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub(crate) fn push(&mut self, entity: Entity) {
        self.stack.push(entity);
    }

    pub(crate) fn pop(&mut self) -> Option<Entity> {
        self.stack.pop()
    }

    pub(crate) fn _get(&mut self, pointer: u32) -> Option<Entity> {
        self.stack.get(pointer as usize).cloned()
    }

    pub(crate) fn execute(&mut self, op: &Token, output: &mut impl Write) {
        match op {
            Token::Gt => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        if a > b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(a), Some(b)) => panic!("Cannot compare non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough arguments for comparison gt"),
                }
            }
            Token::Gte => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        if a >= b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(a), Some(b)) => panic!("Cannot compare non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough arguments for comparison gte"),
                }
            }
            Token::Lt => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        if a < b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(a), Some(b)) => panic!("Cannot compare non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough arguments for comparison lt"),
                }
            }
            Token::Lte => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        if a <= b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(a), Some(b)) => panic!("Cannot compare non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough arguments for comparison lte"),
                }
            }
            Token::Eq => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        if a == b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(Entity::String(ref a)), Some(Entity::String(ref b))) => {
                        if a == b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(Entity::Number(ref a)), Some(Entity::Pointer(ref b))) => {
                        if *a as u32 == *b {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(Entity::Pointer(ref a)), Some(Entity::Number(ref b))) => {
                        if *a == *b as u32 {
                            self.push(Entity::Number(1));
                        } else {
                            self.push(Entity::Number(0));
                        }
                    }
                    (Some(_), None) | (None, Some(_)) => {
                        panic!("Not enough items on stack to compare");
                    }
                    _ => {
                        // by default unequal
                        self.push(Entity::Number(0));
                    }
                }
            }
            Token::Add => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        self.push(Entity::Number(a + b));
                    }
                    (Some(Entity::String(a)), Some(Entity::String(b))) => {
                        self.push(Entity::String(format!("{}{}", b, a)));
                    }
                    (Some(a), Some(b)) => panic!("Cannot add non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough items on stack to add"),
                }
            }
            Token::Sub => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        self.push(Entity::Number(b - a));
                    }
                    (Some(a), Some(b)) => panic!("Cannot subtract non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough items on stack to subtract"),
                }
            }
            Token::Mul => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        self.push(Entity::Number(a * b));
                    }
                    (Some(a), Some(b)) => panic!("Cannot multiply non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough items on stack to multiply"),
                }
            }
            Token::Div => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(Entity::Number(_)), Some(Entity::Number(0))) => {
                        panic!("Cannot divide by zero");
                    }
                    (Some(Entity::Number(a)), Some(Entity::Number(b))) => {
                        self.push(Entity::Number(a / b));
                    }
                    (Some(a), Some(b)) => panic!("Cannot divide non-numbers {:?} {:?}", a, b),
                    _ => panic!("Not enough items on stack to divide"),
                }
            }
            Token::Emit => {
                let a = self.pop();
                match a {
                    Some(Entity::Number(a)) => {
                        write!(output, "{}", a);
                    }
                    Some(Entity::String(a)) => {
                        write!(output, "{}", a);
                    }
                    Some(Entity::Pointer(a)) => {
                        write!(output, "#{:X}", a);
                    }
                    Some(Entity::Function(a)) => {
                        if let Ast::WordDefinition { name, body: _ } = a {
                            write!(output, "FUNC: {:?}", name);
                        } else {
                            panic!("Something went wrong");
                        }
                    }
                    None => {
                        panic!("Not enough items on stack to emit");
                    }
                }
            }
            Token::Dup => {
                let a = self.pop();
                match a {
                    Some(a) => {
                        self.push(a.clone());
                        self.push(a);
                    }
                    None => panic!("Not enough items on stack to dup"),
                }
            }
            Token::Swap => {
                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(a), Some(b)) => {
                        self.push(a);
                        self.push(b);
                    }
                    _ => panic!("Not enough items on stack to swap"),
                }
            }
            Token::Drop => {
                let a = self.pop();
                match a {
                    Some(_) => {}
                    None => panic!("Not enough items on stack to drop"),
                }
            }
            Token::Puts => {
                let length = self.pop();
                match length {
                    Some(Entity::Number(length)) => {
                        let mut string = String::new();
                        for _ in 0..length {
                            let c = self.pop();
                            match c {
                                Some(Entity::Number(c)) => {
                                    string.push(c as u8 as char);
                                }
                                _ => panic!("Cannot puts non-number"),
                            }
                        }
                        write!(output, "{}", string.chars().rev().collect::<String>());
                    }
                    _ => panic!("Cannot puts non-number"),
                }
            }
            _ => panic!("Cannot execute token: {:?}", op),
        }
    }
}
