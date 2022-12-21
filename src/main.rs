mod uxn;
mod system;
mod operations;

use nih_plug::prelude::*;

use talsnd::Gain;

// fn main() {
//     // let rom = include_bytes!("../tests/arithmetic.rom").to_vec();
//     // let rom = include_bytes!("../tests/literals.rom").to_vec();
//     // let rom = include_bytes!("../tests/jumps.rom").to_vec();
//     // let rom = include_bytes!("../tests/memory.rom").to_vec();
//     // let rom = include_bytes!("../tests/stack.rom").to_vec();

//     let rom = include_bytes!("../pixel.rom").to_vec();

//     let mut vm = UXN::new();
//     vm.load(rom);
//     vm.eval(0x100);
// }

fn main() {
    nih_export_standalone::<Gain>();
}