use arrayvec::ArrayVec;

pub struct Stack {
    stack: ArrayVec::<u8, 256>,
    kstack: ArrayVec::<u8, 256>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: ArrayVec::<u8, 256>::from([0; 256]),
            kstack: ArrayVec::<u8, 256>::new(),
        }
    }

    pub fn keep(&mut self, mode: bool) {
        if mode {
            self.kstack = self.stack.clone();
        }
    }

    fn pop(&mut self) -> Option<u8> {
      self.stack.pop()
    }

    fn push(&mut self, item: u8) {
      self.stack.push(item)
    }

    fn peek(&self) -> Option<&u8> {
        self.stack.last()
    }

}