use crate::uxn::UXN;

pub struct ScreenDevice {
	width: u32,
	height: u32,

	x: u16,
	y: u16,
	// color: i8,
	// layer: i8,

	fg: Vec<i8>,
	bg: Vec<i8>,
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
			fg: vec![-1; (w*h) as usize],
			bg: vec![-1; (w*h) as usize],
		}
	}
}

pub fn screen(uxn: &mut UXN, port: usize, val: u8) {
		let rel = port & 0x0F;

		match rel {
		0x0 | 0x1 => {
			println!("Set Screen Vector");
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

		0xe => {
				println!("screen draw pixel");

				let x = uxn.screen.x;
				let y = uxn.screen.y;
				println!("on coords: x: {} y:{}", x, y);

				let layer = (uxn.ram[uxn.dev + port] & 0x40);
				println!("on layer: {}", layer);

				let color = (uxn.ram[uxn.dev + port] & 0x3);
				println!("with color: {}", color);

				let width = uxn.screen.width;

				if layer == 0 {
					blit(&mut uxn.screen.bg, x.into(), y.into(), color, width);
				} else {
					blit(&mut uxn.screen.fg, x.into(), y.into(), color, width);
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

fn blit(layer: &mut Vec<i8>, x: u32, y: u32, color: u8, width: u32) {
	layer[(x + width * y) as usize] = color as i8;
}