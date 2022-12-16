// use crate::uxn::UXN;

// use arrayvec::ArrayVec;

// pub struct Stack {
//     pub stack: ArrayVec::<u8, 256>,
//     kstack: ArrayVec::<u8, 256>,
// }

// impl Stack {
//     pub fn new() -> Self {
//         Stack {
//             stack: ArrayVec::<u8, 256>::from([0; 256]),
//             kstack: ArrayVec::<u8, 256>::new(),
//         }
//     }

//     pub fn keep(&mut self, mode: bool) {
//         if mode {
//             self.kstack = self.stack.clone();
//         }
//     }

//     pub fn ptr(&self) -> u8 {
//         return self.stack[0xff];
//     }

//     // fn pop(&mut self) -> Option<u8> {
//     //   self.stack.pop()
//     // }

//     // fn push(&mut self, item: u8) {
//     //   self.stack.push(item)
//     // }

//     // fn peek(&self) -> Option<&u8> {
//     //     self.stack.last()
//     // }

//     pub fn PUSH8(&mut self, u: &UXN, x: usize) {
//         if self.ptr() == 0xff {
//             panic!("panic");
//             //self.halt(2); // UNWRAP???
//         }
//         // let pos: usize = self.ptr().wrapping_add(1).into();

//         // self.stack[pos] = x as u8;
//     }

//     pub fn PUSH16(&mut self, u: &UXN, x: usize) {
        
//     }

//     pub fn PUSH(&mut self, u: &UXN, x: usize) {
//         if u.bs != 0 {
//             self.PUSH16(u, x);
//         } else {
//             self.PUSH8(u, x);
//         }
//     }

//     pub fn PEEK16(&self, u: &UXN, o: &mut u8, x: usize) {
//         *o = (((u.ram[x] as i32) << 8) + (u.ram[x + 1] as i32)) as u8;
//     }

//     pub fn PEEK(&self, u: &UXN, o: &mut u8, x: usize) {
//         if u.bs != 0 {
//             self.PEEK16(u, o, x);
//         } else {
//             *o = u.ram[x];
//         }
//     }

// }