use crate::uxn::UXN;
use nih_plug_egui::egui::Color32;

pub struct SystemDevice {
	// system colors
	pub color0: Color32,
	pub color1: Color32,
	pub color2: Color32,
	pub color3: Color32,
}

impl SystemDevice {
	pub fn new() -> Self {
		SystemDevice {
			color0: Color32::WHITE,
			color1: Color32::LIGHT_GRAY,
			color2: Color32::DARK_GRAY,
			color3: Color32::BLACK,
		}
	}

	pub fn get_color(&self, index: u8) -> Color32 {
		match index {
			0 => { return self.color0 },
			1 => { return self.color1 },
			2 => { return self.color2 },
			3 => { return self.color3 },
			_ => { return Color32::TRANSPARENT },
		}
	}
}

pub fn system(uxn: &mut UXN, port: usize, val: u8) {
		let rel = port & 0x0F;

		match rel {
		0x0 | 0x1 => {
			println!("Set System Vector");
		}

		// Set the red spectrum color for color0 and color1
		0x8 => {
			let (c0_red, c1_red) = palette(uxn.dev_get(port));

			let c0 = uxn.system.color0;
			uxn.system.color0 = Color32::from_rgb(c0_red, c0.g(), c0.b());

			let c1 = uxn.system.color1;
			uxn.system.color1 = Color32::from_rgb(c1_red, c1.g(), c1.b());
		}

		// Set the red spectrum color for color2 and color3
		0x9 => {
			let (c2_red, c3_red) = palette(uxn.dev_get(port));

			let c2 = uxn.system.color2;
			uxn.system.color2 = Color32::from_rgb(c2_red, c2.g(), c2.b());

			let c3 = uxn.system.color3;
			uxn.system.color3 = Color32::from_rgb(c3_red, c3.g(), c3.b());
		}

		// Set the green spectrum color for color0 and color1
		0xa => {
			let (c0_green, c1_green) = palette(uxn.dev_get(port));

			let c0 = uxn.system.color0;
			uxn.system.color0 = Color32::from_rgb(c0.r(), c0_green, c0.b());

			let c1 = uxn.system.color1;
			uxn.system.color1 = Color32::from_rgb(c1.r(), c1_green, c1.b());
		}

		// Set the green spectrum color for color2 and color3
		0xb => {
			let (c2_green, c3_green) = palette(uxn.dev_get(port));

			let c2 = uxn.system.color2;
			uxn.system.color2 = Color32::from_rgb(c2.r(), c2_green, c2.b());

			let c3 = uxn.system.color3;
			uxn.system.color3 = Color32::from_rgb(c3.r(), c3_green, c3.b());
		}

		// Set the blue spectrum color for color0 and color1
		0xc => {
			let (c0_blue, c1_blue) = palette(uxn.dev_get(port));

			let c0 = uxn.system.color0;
			uxn.system.color0 = Color32::from_rgb(c0.r(), c0.g(), c0_blue);

			let c1 = uxn.system.color1;
			uxn.system.color1 = Color32::from_rgb(c1.r(), c1.g(), c1_blue);
		}

		// Set the blue spectrum color for color2 and color3
		0xd => {
			let (c2_blue, c3_blue) = palette(uxn.dev_get(port));

			let c2 = uxn.system.color2;
			uxn.system.color2 = Color32::from_rgb(c2.r(), c2.g(), c2_blue);

			let c3 = uxn.system.color3;
			uxn.system.color3 = Color32::from_rgb(c3.r(), c3.g(), c3_blue);
		}

		0xf => {
			println!("\nProgram Ended");
		}

		_ => {
			println!("System - Unknown DEO - {:x?}", port);
		}
	}
}

fn palette(mem_color: u8) -> (u8, u8) {
	let c1 = (mem_color >> (1 << 2)) & 0x0F;
	let c2 = (mem_color >> (0 << 2)) & 0x0F;

	return (c1 + (c1 << 4), c2 + (c2 << 4))
}