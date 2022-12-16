mod uxn;
mod system;
mod operations;

use uxn::*;

fn main() {
    let rom = include_bytes!("../tests/arithmetic.rom").to_vec();
    // let rom = include_bytes!("../tests/literals.rom").to_vec();

    let mut vm = UXN::new();
    vm.load(rom);
    vm.eval(0x100);
}
