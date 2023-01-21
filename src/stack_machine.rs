#[derive(Debug, PartialEq, Clone)]
enum Entity {
    Number(i32),
    String(String),
    Pointer(u32),
}

struct StackMachine {
    stack: Vec<Entity>,
}

impl StackMachine {
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