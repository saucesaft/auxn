use crate::uxn::UXN;
use nih_plug_egui::egui::Color32;

#[derive(Debug)]
pub enum DrawOperation {
	Pixel {x: u16, y: u16, color: Color32}
}

pub struct ScreenDevice {
	width: u32,
	height: u32,

	x: u16,
	y: u16,
	// color: i8,
	// layer: i8,

	// pub fg: Vec<i8>,
	// pub bg: Vec<i8>,
}

impl ScreenDevice {
	pub fn new(w: u32, h: u32) -> Self {
		ScreenDevice {
			width: w,
			height: h,

			// coordinates to position the next thing to draw
			x: 0,
			y: 0,
			// color: 0,
			// layer: 0,

			// layers of pixels
			// -1 means no pixel, any other number represents the color (e.g 0, 1, 2, 3)
			// fg: vec![-1; (w*h) as usize],
			// bg: vec![-1; (w*h) as usize],
		}
	}
}

pub fn screen(uxn: &mut UXN, port: usize, val: u8) {
		let rel = port & 0x0F;

		match rel {
		0x0 | 0x1 => {
			if rel == 0x1 {
				let a = (uxn.ram[uxn.dev + port-1] as i32) << 8;
				let b = (uxn.ram[uxn.dev + port] as i32);

				// execute all the instructions from the instruction (a | b)
				// every frame, until we hit a BRK

				// set it as an entry in the struct, and make it public
				// to the main loop which will run each vector every instance of the loop

				// for this, we will insert a JMP into the code?
				// new idea, we could just change the pc variable according to each vector

				println!("Set Screen Vector: {}", a | b);			
			}
		}

		0x3 => {
				println!("screen set width");
		}

		0x5 => {
				println!("screen set height");
		}

		0x8 | 0x9 => {
			if rel == 0x9 {
				let a = (uxn.ram[uxn.dev + port-1] as i32) << 8;
				let b = (uxn.ram[uxn.dev + port] as i32);

				uxn.screen.x = (a + b) as u16;
			}
		}

		0xa | 0xb => {
			if rel == 0xb {
				let a = (uxn.ram[uxn.dev + port-1] as i32) << 8;
				let b = (uxn.ram[uxn.dev + port] as i32);

				uxn.screen.y = (a + b) as u16;
			}
		}

		// write a pixel to the screen
		0xe => {
				// let x = uxn.screen.x;
				// let y = uxn.screen.y;
				// let layer = (uxn.ram[uxn.dev + port] & 0x40);
				let color = uxn.system.get_color((uxn.ram[uxn.dev + port] & 0x3) as i8);
				// let width = uxn.screen.width;

				let p = DrawOperation::Pixel {
					x: uxn.screen.x,
					y: uxn.screen.y,
					color: color,
				};

				uxn.sender.send(p).unwrap();
				println!("pixel sent :)");

				// if layer == 0 {
				// 	blit(&mut uxn.screen.bg, x.into(), y.into(), color, width);
				// } else {
				// 	blit(&mut uxn.screen.fg, x.into(), y.into(), color, width);
				// }
		}

		0xf => {
				println!("screen draw sprite");
		}

		_ => {
			println!("Screen - Unknown DEO - {:x?}", port);
		}
	}
}

// fn blit(layer: &mut Vec<i8>, x: u32, y: u32, color: u8, width: u32) {
// 	layer[(x + width * y) as usize] = color as i8;
// }