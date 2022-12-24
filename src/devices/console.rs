use crate::uxn::UXN;

pub struct ConsoleDevice {}

impl ConsoleDevice {
	pub fn new() -> Self {
		ConsoleDevice {}
	}
}

pub fn console(uxn: &mut UXN, port: usize, val: u8) {
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
			println!("Console - Unknown DEO - {:x?}", port);
		}
	}
}