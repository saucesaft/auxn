use crate::uxn::UXN;
use nih_plug_egui::egui::{Color32, ColorImage, Context, TextureHandle};

static blending: [[u8; 16]; 5] = [
    [0,0,0,0,1,0,1,1,2,2,0,2,3,3,3,0,],
    [0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3,],
    [1,2,3,1,1,2,3,1,1,2,3,1,1,2,3,1,],
    [2,3,1,2,2,3,1,2,2,3,1,2,2,3,1,2,],
    [1,1,1,1,1,0,1,1,1,1,0,1,1,1,1,0,],
];

pub struct ScreenDevice {
    pub width: u32,
    pub height: u32,

    x: u16,
    y: u16,

    pub fg: ColorImage,
    pub bg: ColorImage,

    pub display: Option<TextureHandle>,

    pub vector: usize,
    pub addr: usize,

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
            addr: 0,

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
        0x6 => {
        	println!("addr! {}", val);
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

        // register sprite addr
        0xc | 0xd => {
        	if rel == 0xd {
        		let a = (uxn.ram[uxn.dev + port-1] as i32) << 8;
        		let b = (uxn.ram[uxn.dev + port] as i32);

        		uxn.screen.addr = (a + b) as usize;
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
            if 0 <= x && x < (uxn.screen.width as usize) {
            	if 0 <= y && y < (uxn.screen.height as usize) {

            		if layer == 0x40 { // write to fg
		            	uxn.screen.fg[(x, y)] = color;
		            	uxn.screen.redraw = true;
            		} else if layer == 0x00 { // write to bg
		            	uxn.screen.bg[(x, y)] = color;
		            	uxn.screen.redraw = true;
            		}

            	}
            }

            // POKE 16
            if (uxn.dev_get(section + 0x6) & 0x01) != 0 {
            	uxn.ram[uxn.dev + (section + 0x8)] = (x + 1) as u8;
            	println!("auto x+1");
            }

            // POKE 16
            if (uxn.dev_get(section + 0x6) & 0x02) != 0 {
            	uxn.ram[uxn.dev + (section + 0xa)] = (y + 1) as u8;
            	println!("auto y+1");
            }
        }

        0xf => {
        	let mut i: usize = 0;
			let x = uxn.screen.x as usize;
            let y = uxn.screen.y as usize;

            // println!("x: {} y: {}", x, y);

            let layer = uxn.ram[uxn.dev + port] & 0x40;
            let mut addr = uxn.screen.addr;

            let twobpp = {
            	if (uxn.ram[uxn.dev + port] & 0x80) != 0 {
            		1
            	} else {
            		0
            	}
            };

            let n = (uxn.dev_get(section + 0x6) >> 4) as usize;
            let dx = ((uxn.dev_get(section + 0x6) & 0x01) << 3) as usize;
            let dy = ((uxn.dev_get(section + 0x6) & 0x02) << 2) as usize;

            if addr > 0x10000 - ((n + 1) << (3 + twobpp)) as usize {
            	return
            }

            while i <= n {

            	let mut v = 0;
            	let mut h: i8 = 7;

            	// println!("i: {}", i);

            	let opaque: u8 = blending[4][(uxn.ram[uxn.dev + port] & 0xf) as usize];

            	// println!("opaque: {}", opaque);

            	while v < 8 {

            		// println!("v: {}", v);

            		let two = {
            			if twobpp == 1 {
            				uxn.ram[addr + v + 8]
            			} else {
            				0
            			}
            		};

            		let mut c: u16 = uxn.ram[addr + v] as u16 | ((two as i32) << 8) as u16;

            		while h >= 0 {

            			// println!("h: {}", h);

            			let ch: u8 = (c & 1) as u8 | ((c >> 7) & 2) as u8;

            			if opaque != 0 || ch != 0 {

            				let nx = {
            					if (uxn.ram[uxn.dev + port] & 0x10) != 0 {
            						(x + dy * i) + (7 - h) as usize
            					} else {
            						(x + dy * i) + h as usize
            					}
            				};

            				let ny = {
            					if (uxn.ram[uxn.dev + port] & 0x20) != 0 {
            						(y + dx + i) + (7 - v) as usize
            					} else {
            						(y + dx + i) + v as usize
            					}
            				};

            				// let color = uxn.system.get_color(blending[ch as usize][(uxn.ram[uxn.dev + port] & 0x0f) as usize]);
            				let color = uxn.system.color2;

            				// println!("nx: {} ny: {}", nx, ny);

				            if 0 <= nx && nx < (uxn.screen.width as usize) {
				            	if 0 <= ny && ny < (uxn.screen.height as usize) {

				            		if layer == 0x40 { // write to fg
						            	uxn.screen.fg[(nx, ny)] = color;
						            	uxn.screen.redraw = true;
				            		} else if layer == 0x00 { // write to bg
						            	uxn.screen.bg[(nx, ny)] = color;
						            	uxn.screen.redraw = true;
				            		}

				            	}
				            }

            			}

            			h = h - 1;
            			c = c >> 1;
            		}

            		v += 1;
            	}

            	addr += ((uxn.dev_get(section + 0x6) & 0x04) << (1 + twobpp)) as usize;

            	i += 1;
            }

            uxn.dev_poke(section + 0xc, addr as u16);
            uxn.dev_poke(section + 0x8, (x + dx) as u16);
            uxn.dev_poke(section + 0xa, (y + dy) as u16);

        }

        _ => {
            println!("Screen - Unknown DEO - {:x?}", port);
        }
    }
}
