use crate::uxn::UXN;
use nih_plug_egui::egui::{Color32, ColorImage, Context, TextureHandle};

#[derive(Debug)]
pub enum DrawOperation {
    Pixel { x: u16, y: u16, color: Color32 },
}

pub struct ScreenDevice {
    width: u32,
    height: u32,

    x: u16,
    y: u16,
    pub buffer: ColorImage,
    pub display: Option<TextureHandle>,

    pub vector: usize,

    pub change: bool,
}

impl ScreenDevice {
    pub fn new(w: u32, h: u32) -> Self {
        ScreenDevice {
            width: w,
            height: h,

            // coordinates to position the next thing to draw
            x: 0,
            y: 0,
            buffer: ColorImage::new([w as usize, h as usize], Color32::BLACK),
            display: None::<TextureHandle>,

            vector: 0,

            change: false,
        }
    }

    // load the buffer to memory
    pub fn generate(&mut self, ctx: &Context) {
        self.display = Some(ctx.load_texture("buffer", self.buffer.clone(), Default::default()));
    }

    // return the screen vector
    pub fn vector(&self) -> usize {
        return self.vector;
    }
}

pub fn screen(uxn: &mut UXN, port: usize, val: u8) {
    let rel = port & 0x0F;

    match rel {
        // set the vector address
        0x0 | 0x1 => {
            if rel == 0x1 {
                let a = (uxn.ram[uxn.dev + port - 1] as i32) << 8;
                let b = (uxn.ram[uxn.dev + port] as i32);

                uxn.screen.vector = (a | b) as usize;
            }
        }

        // set screen width - no resizing support yet
        0x3 => {
            println!("screen set width");
        }

        // set screen height - no resizing support yet
        0x5 => {
            println!("screen set height");
        }

        // set x coordinate
        0x8 | 0x9 => {
        	if rel == 0x9 {
        		let a = (uxn.ram[uxn.dev + port-1] as i32) << 8;
        		let b = (uxn.ram[uxn.dev + port] as i32);

        		uxn.screen.x = (a + b) as u16;
        	}
        }

        // set y coordinate
        0xa | 0xb => {
        	if rel == 0xb {
        		let a = (uxn.ram[uxn.dev + port-1] as i32) << 8;
        		let b = (uxn.ram[uxn.dev + port] as i32);

        		uxn.screen.y = (a + b) as u16;
        	}
        }

        // write a pixel to the screen
        0xe => {
            let x = uxn.screen.x as usize;
            let y = uxn.screen.y as usize;
            let color = uxn.system.get_color(uxn.ram[uxn.dev + port] & 0x3);

            // check that the coordiantes are actually aplicable to our screen
            // if not, we simply ignore them, this is a default behaviour
            if 0 < x && x < (uxn.screen.width as usize) {
            	if 0 < y && y < (uxn.screen.height as usize) {
	            	uxn.screen.buffer[(x, y)] = color;
	            	uxn.screen.change = true;
            	}
            }
        }

        0xf => {
            println!("screen draw sprite");
        }

        _ => {
            println!("Screen - Unknown DEO - {:x?}", port);
        }
    }
}
