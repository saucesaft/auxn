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
    	let mut buffer = self.bg.clone();

    	for (i, p) in self.fg.pixels.iter().enumerate() {
    		if *p != Color32::TRANSPARENT {
    			buffer.pixels[i] = *p;
    		}
    	}

        self.display = Some(ctx.load_texture("buffer", buffer, Default::default()));
    }

    pub fn pixel(&mut self, x: usize, y: usize, color: Color32, layer: u8) {
        // check that the coordiantes are actually aplicable to our screen
        // if not, we simply ignore them, this is a default behaviour
        if 0 <= x && x < (self.width as usize) {
        	if 0 <= y && y < (self.height as usize) {

        		 // write to the foreground buffer
        		if layer != 0x00 {
	            	self.fg[(x, y)] = color;
	            	self.redraw = true;

        		 // write to the background buffer
        		} else {
	            	self.bg[(x, y)] = color;
	            	self.redraw = true;
        		}

        	}
        }
    }

    pub fn resize(&mut self) {
        self.fg = ColorImage::new([self.width as usize, self.height as usize], Color32::TRANSPARENT);
        self.bg = ColorImage::new([self.width as usize, self.height as usize], Color32::TRANSPARENT);
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
        0x2 | 0x3 => {
        	if rel == 0x3 {
		        let w = {
		            let a = (uxn.ram[uxn.dev + 0x22] as i32) << 8;
		            let b = (uxn.ram[uxn.dev + 0x23] as i32);

		            (a+b) as usize
		        };

		        uxn.screen.width = w as u32;

		        uxn.screen.resize();
        	}
        }

        // register - set screen height
        0x4 | 0x5 => {
        	if rel == 0x5 {
		        let h = {
		            let a = (uxn.ram[uxn.dev + 0x24] as i32) << 8;
		            let b = (uxn.ram[uxn.dev + 0x25] as i32);

		            (a+b) as usize
		        };

		        uxn.screen.height = h as u32;

		        uxn.screen.resize();
        	}
        }

        // register auto-mode
        // we will handle this accordingly
        // in the pixel or sprite cases
        0x6 => {
        	// println!("addr! {}", val);
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

            uxn.screen.pixel(x, y, color, layer);

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

            // println!("addr: {:#x?}", addr);
            // log::info!("addr {:?}", uxn.ram[addr+13]);

            let twobpp = {
            	if (uxn.dev_get(port) & 0x80) != 0 {
            		1
            	} else {
            		0
            	}
            };

            // println!("twobpp: {:?}", twobpp);

            let n = (uxn.dev_get(section + 0x6) >> 4) as usize;
            let dx = ((uxn.dev_get(section + 0x6) & 0x01) << 3) as usize;
            let dy = ((uxn.dev_get(section + 0x6) & 0x02) << 2) as usize;

            // println!("n: {:?}", n);
            // println!("dx: {:?}", dx);
            // println!("dy: {:?}", dy);
            // println!("----------");

            if addr > 0x10000 - ((n + 1) << (3 + twobpp)) as usize {
            	return
            }

            while i <= n {

            	let mut v = 0;
            	let mut h: i8 = 7;

            	// println!("i: {}", i);

            	let opaque: u8 = blending[4][(uxn.dev_get(port) & 0xf) as usize];

            	// println!("opaque: {:?}", opaque);

            	while v < 8 {

            		// println!("v: {:?}", v);

            		let two = {
            			if twobpp == 1 {
            				uxn.ram[addr + v + 8]
            			} else {
            				0
            			}
            		};

            		// println!("two: {:?}", two);

            		let mut c: u16 = (uxn.ram[addr + v] as i32 | ((two as i32) << 8)) as u16;

            		h = 7;
            		while h >= 0 {

            			// println!("h: {:?}", h);

            			let ch: u8 = (((c as i32) & 1) | (((c as i32) >> 7) & 2)) as u8;

            			// println!("ch: {:?}", ch);

            			if opaque != 0 || ch != 0 {

            				let nx = {
            					if (uxn.dev_get(port) & 0x10) != 0 {
            						(x + dy * i) + (7 - h) as usize
            					} else {
            						(x + dy * i) + (h) as usize
            					}
            				};

            				let ny = {
            					if (uxn.dev_get(port) & 0x20) != 0 {
            						(y + dx * i) + (7 - v) as usize
            					} else {
            						(y + dx * i) + (v) as usize
            					}
            				};

            				let color = uxn.system.get_color(blending[ch as usize][(uxn.dev_get(port) & 0xf) as usize]);

            				uxn.screen.pixel(nx, ny, color, layer);

            			}

            			h -= 1;
            			c = ((c as i32) >> 1) as  u16;
            		}

            		v += 1;
            	}

            	addr += ((uxn.dev_get(section + 0x6) & 0x4) << (1 + twobpp)) as usize;

            	// println!("addr: {:?}", addr);
            	// println!("first: {:?} twobpp: {:?}", (uxn.dev_get(section + 0x6) & 0x4), twobpp);
            	// println!("second {:?}", (((uxn.dev_get(section + 0x6) & 0x4) as i32) << (1 + twobpp) as i32));

            	i += 1;
            }

            // println!("addr: {:?}", addr);
            // println!("addr: {:?}", addr as u16);
            // println!("----");

            uxn.dev_poke(section + 0xc, addr as u16);
            uxn.screen.addr = addr;

            uxn.dev_poke(section + 0x8, (x + dx) as u16);
            uxn.screen.x = (x + dx) as u16;

            uxn.dev_poke(section + 0xa, (y + dy) as u16);
            uxn.screen.y = (y + dy) as u16;

        }

        _ => {
            println!("Screen - Unknown DEO - {:x?}", port);
        }
    }
}
