use crate::uxn::UXN;
use nih_plug_egui::egui::{Color32, ColorImage, Context, TextureHandle};

static blending: [[u8; 16]; 5] = [
    [0,0,0,0,1,0,1,1,2,2,0,2,3,3,3,0],
    [0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3],
    [1,2,3,1,1,2,3,1,1,2,3,1,1,2,3,1],
    [2,3,1,2,2,3,1,2,2,3,1,2,2,3,1,2],
    [1,1,1,1,1,0,1,1,1,1,0,1,1,1,1,0],
];

pub struct ScreenDevice {
    // width and height of the device
    pub width: u32,
    pub height: u32,

    // both background and foreground buffers
    pub fg: ColorImage,
    pub bg: ColorImage,

    // the actual texture stored on the GPU
    pub display: Option<TextureHandle>,

    // address of the vector
    pub vector: usize,
    
    // address in memory of the sprite to draw
    pub addr: usize,

    // coords of the next pixel to write
    x: u16,
    y: u16,

    // this boolean is true when we need to update
    pub redraw: bool,

    // system colors
    pub color0: Color32,
    pub color1: Color32,
    pub color2: Color32,
    pub color3: Color32,
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

            color0: Color32::BLACK,
            color1: Color32::DARK_GRAY,
            color2: Color32::LIGHT_GRAY,
            color3: Color32::WHITE,
        }
    }

    pub fn get_color(&self, index: u8) -> Color32 {
        match index {
            0 => self.color0,
            1 => self.color1,
            2 => self.color2,
            3 => self.color3,
            _ => Color32::TRANSPARENT,
        }
    }

    // load the buffer to video memory
    pub fn generate(&mut self, ctx: &Context) {
    	let mut buffer = self.bg.clone();

        // "mix" both buffers into one, but only temporarly
    	for (i, p) in self.fg.pixels.iter().enumerate() {
    		if *p != Color32::TRANSPARENT {
    			buffer.pixels[i] = *p;
    		}
    	}

        // upload that buffer as a texture to the GPU
        self.display = Some(ctx.load_texture("buffer", buffer, Default::default()));
    }

    pub fn screen_blit(&mut self, layer: u8, x: u16, y: u16, sprite: &[u8], color: usize, flipx: u8, flipy: u8, twobpp: u8, opaque: u8) {
        let mut v: u16 = 0;
        let mut h: i8 = 7;

        while v < 8 {

            let two = {
                if twobpp == 1 {
                    sprite[(v + 8) as usize]
                } else {
                    0
                }
            };

            let mut c: u16 = (sprite[v as usize] as i32 | (two as i32) << 8) as u16;

            h = 7;
            while h >= 0 {

                let ch: u8 = ((c & 1) | ((c >> 7) & 2)) as u8;

                if opaque != 0 || ch != 0 {
                    let nx = {
                        if flipx != 0 {
                            (x + (7 - h) as u16)
                        } else {
                            // println!("{:?} + {:?}", x, h);
                            // println!("----------");

                            (x + (h as u16))
                        }
                    };

                    let ny = {
                        if flipy != 0 {
                            (y + (7 - v) as u16)
                        } else {
                            (y + v)
                        }
                    };

                    let pcolor = self.get_color(blending[ch as usize][color]);
                    self.screen_write(nx as usize, ny as usize, pcolor, layer);
                }

                h -= 1;
                c = ((c as i32) >> 1) as u16;
            }

            v += 1;
        }
    }

    pub fn screen_write(&mut self, x: usize, y: usize, mut color: Color32, layer: u8) {
        // check that the coordiantes are actually aplicable to our screen
        // if not, we simply ignore them, this is a default behaviour
        if x < (self.width as usize) {
        	if y < (self.height as usize) {

        		 // write to the foreground buffer
        		if layer != 0x00 {
                    // simulate alpha blending
                    if color == self.color0 {
                        color = Color32::TRANSPARENT;
                    }

        			if color != self.fg[(x, y)] {
		            	self.fg[(x, y)] = color;
		            	self.redraw = true;
        			}

        		 // write to the background buffer
        		} else {
        			if color != self.bg[(x, y)] {
		            	self.bg[(x, y)] = color;
		            	self.redraw = true;
        			}
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
            let color = uxn.screen.get_color(uxn.ram[uxn.dev + port] & 0x3);
            let layer = uxn.ram[uxn.dev + port] & 0x40;

            uxn.screen.screen_write(x, y, color, layer);

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
        	let mut i = 0;
			let x = uxn.screen.x;
            let y = uxn.screen.y;
            let color = (uxn.dev_get(port) & 0xf) as usize;

            let layer = uxn.ram[uxn.dev + port] & 0x40;
            let mut sprite_addr = uxn.screen.addr;

            let twobpp = {
            	if (uxn.dev_get(port) & 0x80) != 0 {
            		1
            	} else {
            		0
            	}
            };

            // println!("x: {:?} y {:?}", x, y);

            let n = (uxn.dev_get(section + 0x6) >> 4);
            let dx = ((uxn.dev_get(section + 0x6) & 0x01) << 3);
            let dy = ((uxn.dev_get(section + 0x6) & 0x02) << 2);

            if sprite_addr > 0x10000 - ((n + 1) << (3 + twobpp)) as usize {
            	return
            }

            while i <= n {

                // println!("x: {:?} dy*i: {:?}", x, u16::from(dy * i));

                let sprite_x: u16 = x + u16::from(dy * i);
                // let sprite_x: u16 = x;

                // println!("sprite_x: {:?}", sprite_x);
                // println!("sprite_x: {:?}", x + u16::from(dy * i));
                // println!("----------");

                let sprite_y: u16 = y + u16::from(dx * i);
                let flipx = (uxn.dev_get(port) & 0x10);
                let flipy = (uxn.dev_get(port) & 0x20);

                let opaque: u8 = blending[4][color];

                let sprite: &[u8] = &uxn.ram[sprite_addr..];

                uxn.screen.screen_blit(layer, sprite_x, sprite_y, sprite, color, flipx, flipy, twobpp, opaque);

            	sprite_addr += ((uxn.dev_get(section + 0x6) & 0x4) << (1 + twobpp)) as usize;

            	i += 1;
            }

            uxn.dev_poke(section + 0xc, sprite_addr as u16);
            uxn.screen.addr = sprite_addr;

            uxn.dev_poke(section + 0x8, x + u16::from(dx));
            uxn.screen.x = x + u16::from(dx);

            uxn.dev_poke(section + 0xa, y + u16::from(dy));
            uxn.screen.y = y + u16::from(dy);
        }

        _ => {
            println!("Screen - Unknown DEO - {:x?}", port);
        }
    }
}
