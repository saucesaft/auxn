use crate::uxn::UXN;
use nih_plug_egui::egui::{Color32, ColorImage, TextureHandle, Context};
use image::{RgbImage, Rgb, ImageBuffer, DynamicImage, GenericImage};

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

	pub buffer: ColorImage,
	pub display: Option<TextureHandle>,

	pub vector: usize,

	pub change: bool
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

			buffer: ColorImage::new([w as usize, h as usize], Color32::BLACK),
			display: None::<TextureHandle>,

			vector: 0,

			change: false,
		}
	}

	pub fn generate(&mut self, ctx: &Context) {
		// let width = self.width;
		// let height = self.height;

		// let size = [width as usize, height as usize];

		// let image_buffer = self.buffer.to_rgba8();
		// let pixels = image_buffer.as_flat_samples();

		// self.display = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        self.display = Some(ctx.load_texture(
			"buffer",
            self.buffer.clone(),
            Default::default(),
        ));
	}

	pub fn vector(&self) -> usize {
		return self.vector;
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

				// println!("Set Screen Vector: {:#x?}", a | b);

				uxn.screen.vector = (a | b) as usize;	
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
				// println!("x: {}", uxn.screen.x);
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
				let x = uxn.screen.x;
				let y = uxn.screen.y;
				let color = uxn.system.get_color(uxn.ram[uxn.dev + port] & 0x3);

				let width = uxn.screen.width as u16;

				uxn.screen.buffer.pixels[(x + width * y) as usize] = color;
				uxn.screen.change = true;
		}

		0xf => {
				println!("screen draw sprite");
		}

		_ => {
			println!("Screen - Unknown DEO - {:x?}", port);
		}
	}
}

// fn blit(layer: &mut Vec<u8>, x: u32, y: u32, color: u8, width: u32) {
// 	layer[(x + width * y) as usize] = color;
// }