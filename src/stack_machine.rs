#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Entity {
    Number(i32),
    String(String),
    Pointer(u32),
}

pub(crate) struct StackMachine {
    stack: Vec<Entity>,
}

impl StackMachine {
    pub(crate) fn new() -> Self {
        Self { stack: Vec::new() }
    }
    fn push(&mut self, entity: Entity) {
        self.stack.push(entity);
    }

    fn pop(&mut self) -> Option<Entity> {
        self.stack.pop()
    }

    fn get(&mut self, pointer: u32) -> Option<Entity> {
        self.stack.get(pointer as usize).cloned()
    }
}