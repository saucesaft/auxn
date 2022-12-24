use crate::uxn::UXN;

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

		0xe => {
				println!("screen draw pixel");
		}

		0xf => {
				println!("screen draw sprite");
		}

		_ => {
			println!("Screen - Unknown DEO - {:x?}", port);
		}
	}
}