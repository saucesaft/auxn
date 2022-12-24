use crate::uxn::UXN;

pub fn system(uxn: &mut UXN, port: usize, val: u8) {
		let rel = port & 0x0F;

		match rel {
		0x0 | 0x1 => {
			println!("Set System Vector");
		}

		0x8 => {
			println!("Set Red Color ");
			let (c1, c2) = palette(uxn.dev_get(port));

			println!("{:#x}", c1);
			println!("{:#x}", c2);
		}

		0x9 => {
			println!("Set Red Color ");
			let (c1, c2) = palette(uxn.dev_get(port));

			println!("{:#x}", c1);
			println!("{:#x}", c2);
		}

		0xa | 0xb => {
			println!("Set Green Color");
		}

		0xc | 0xd => {
			println!("Set Blue Color");
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

	return ( c1 + (c1 << 4), c2 + (c2 << 4) )
}