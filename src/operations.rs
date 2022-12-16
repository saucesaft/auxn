use crate::uxn::UXN;
// use crate::system::Stack;

impl UXN {
	pub fn PUSH16(&mut self, s: u8) {
		self.PUSH8(((s) as i32 >> 0x08) as u8);
		self.PUSH8(s & 0xff);
	}

	pub fn PUSH8(&mut self, s: u8) {
		if self.ptr() == 0xff {
			panic!("OVERFLOW");
		}
		let index = self.inc();
		self.ram[self.src + index] = s;
	}

	pub fn PUSH(&mut self, s: u8) {
		if self.bs != 0 {
            self.PUSH16(s);
        } else {
            self.PUSH8(s);
        }		
	}

	pub fn POP16(&mut self, s: u8) {
		self.POP8(((s) as i32 >> 0x08) as u8);
		self.POP8(s & 0xff);
	}

	pub fn POP8(&mut self, s: u8) {
		if self.ptr() == 0xff {
			panic!("OVERFLOW");
		}
		let index = self.inc();
		self.ram[self.src + index] = s;
	}

	pub fn POP(&mut self, s: u8) {
		if self.bs != 0 {
            self.POP16(s);
        } else {
            self.POP8(s);
        }		
	}

	pub fn PEEK16(&self, x: usize) -> u8 {
		(((self.ram[x] as i32) << 8) + (self.ram[x + 1] as i32)) as u8
	}

	pub fn PEEK(&self, x: usize) -> u8 {
		if self.bs != 0 {
            self.PEEK16(x)
        } else {
            self.ram[x]
        }
	}
}