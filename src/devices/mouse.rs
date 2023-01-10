use crate::uxn::UXN;

pub struct MouseDevice {
    // address of the vector
    pub vector: usize,
}

impl MouseDevice {
    pub fn new() -> Self {
        MouseDevice {
            vector: 0,
        }
    }

    // return the screen vector
    pub fn vector(&self) -> usize {
        return self.vector;
    }
}

pub fn mouse(uxn: &mut UXN, port: usize, val: u8) {
    let rel = port & 0x0F;

    match rel {
        0x0 | 0x1 => {
            if rel == 0x1 {
                let a = (uxn.ram[uxn.dev + port - 1] as i32) << 8;
                let b = (uxn.ram[uxn.dev + port] as i32);

                uxn.mouse.vector = (a | b) as usize;
            }
        }

        _ => {
            println!("System - Unknown DEO - {:x?}", port);
        }
    }
}

pub fn mouse_pos(uxn: &mut UXN, screen_x: f32, screen_y: f32) {
    uxn.dev_poke(0x92, screen_x as u16);
    uxn.dev_poke(0x94, screen_y as u16);

    let mouse_vector_addr = uxn.mouse.vector();
    uxn.eval(mouse_vector_addr);
}

pub fn mouse_down(uxn: &mut UXN) {
    println!("mouse down!");

    uxn.ram[uxn.dev + 0x96] =

    let mouse_vector_addr = uxn.mouse.vector();
    uxn.eval(mouse_vector_addr);
}

pub fn mouse_up(uxn: &mut UXN) {
    println!("mouse up!");
    let mouse_vector_addr = uxn.mouse.vector();
    uxn.eval(mouse_vector_addr);
}