use crate::uxn::UXN;
use nih_plug_egui::egui::{Color32, ColorImage, Context, TextureHandle};

pub struct ScreenDevice {
    pub width: u32,
    pub height: u32,

    x: u16,
    y: u16,

    pub fg: ColorImage,
    pub bg: ColorImage,

    pub display: Option<TextureHandle>,

    pub vector: usize,

    pub redraw: bool,
}

impl ScreenDevice {
    pub fn new(w: u32, h: u32) -> Self {
        ScreenDevice {
            width: w,
            height: h,

            // coordinates to position the next thing to draw
            x: 0,
            y: 0,

            fg: ColorImage::new([w as usize, h as usize], Color32::TRANSPARENT),
            bg: ColorImage::new([w as usize, h as usize], Color32::TRANSPARENT),
            
            display: None::<TextureHandle>,

            vector: 0,

            redraw: false,
        }
    }

    // load the buffer to memory
    pub fn generate(&mut self, ctx: &Context) {
    	let mut mix = self.bg.clone();

    	for (i, p) in self.fg.pixels.iter().enumerate() {
    		if *p != Color32::TRANSPARENT {
    			mix.pixels[i] = *p;
    		}
    	}

        self.display = Some(ctx.load_texture("buffer", mix, Default::default()));
    }

    // return the screen vector
    pub fn vector(&self) -> usize {
        return self.vector;
    }
}

pub fn screen(uxn: &mut UXN, port: usize, val: u8) {
    let rel = port & 0x0F;
    let section = port & 0xF0;

    match rel {
        // set the vector address
        0x0 | 0x1 => {
            if rel == 0x1 {
                let a = (uxn.ram[uxn.dev + port - 1] as i32) << 8;
                let b = (uxn.ram[uxn.dev + port] as i32);

                uxn.screen.vector = (a | b) as usize;
            }
        }

        // register - set screen width
        0x2 | 0x3 => {}

        // register - set screen height
        0x4 | 0x5 => {}

        // register auto-mode
        // we will handle this accordingly
        // in the pixel or sprite cases
        0x6 => {}

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
            let layer = uxn.ram[uxn.dev + port] & 0x40;

            // check that the coordiantes are actually aplicable to our screen
            // if not, we simply ignore them, this is a default behaviour
            if 0 < x && x < (uxn.screen.width as usize) {
            	if 0 < y && y < (uxn.screen.height as usize) {

            		if layer == 0x40 { // write to fg
		            	uxn.screen.fg[(x, y)] = color;
		            	uxn.screen.redraw = true;
            		} else if layer == 0x00 { // write to bg
		            	uxn.screen.bg[(x, y)] = color;
		            	uxn.screen.redraw = true;
            		}

            	}
            }

            if (uxn.dev_get(section + 0x6) & 0x01) != 0 {
            	uxn.ram[uxn.dev + (section + 0x8)] = (x + 1) as u8;
            	println!("auto x+1");
            }

            if (uxn.dev_get(section + 0x6) & 0x02) != 0 {
            	uxn.ram[uxn.dev + (section + 0xa)] = (y + 1) as u8;
            	println!("auto y+1");
            }
        }

        0xf => {
            // println!("screen draw sprite");
        }

        _ => {
            println!("Screen - Unknown DEO - {:x?}", port);
        }
    }
}
