use crate::uxn::UXN;

mod stack;
pub use stack::*;

mod device;
pub use device::*;

mod opcodes;
pub use opcodes::*;

pub fn screen_dev(uxn: &mut UXN, port: usize, val: u8) {
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

		0xe => {
				println!("screen draw pixel");
		}

		0xf => {
				println!("screen draw sprite");
		}

		_ => {
			println!("Screen - Unknown DEO - {}", port);
		}
	}
}

pub fn console_dev(uxn: &mut UXN, port: usize, val: u8) {
		let rel = port & 0x0F;

		match rel {
		0x0 | 0x1 => {
			println!("Set Console Vector");
		}

		0x2 => {
			if val != 0 {
				uxn.wst = (val as usize) * 0x100;
			} else {
				uxn.wst = 0x10000;
			}
		}

		0x8 => {
			// if val == 0x0a{
			// 	println!();
			// } else {

				// match char::from_u32(val.into()) {
				// 	Some(c) => print!("{}", c),
				// 	None => {},
				// }

				print!("{:02x} ", val);
			// }
		}

		_ => {
			println!("Console - Unknown DEO - {}", port);
		}
	}
}

fn palette(mem_color: u8) -> (u8, u8) {
	let c1 = (mem_color >> (1 << 2)) & 0x0F;
	let c2 = (mem_color >> (0 << 2)) & 0x0F;

	return ( c1 + (c1 << 4), c2 + (c2 << 4) )
}

pub fn system_dev(uxn: &mut UXN, port: usize, val: u8) {
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
			println!("System - Unknown DEO - {}", port);
		}
	}
}