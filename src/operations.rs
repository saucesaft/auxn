use crate::uxn::UXN;
// use crate::system::Stack;

impl UXN {
	pub fn rel(&self, val: usize) -> usize {
		if val > 0x80 {
			val - 256
		} else {
			val
		}
	}

	pub fn DEI(&self, port: usize) -> u8 {
		return self.ram[self.dev + port]
	}

	pub fn DEO(&mut self, port: usize, val: u8) {
		self.ram[self.dev + port] = val;

		match port {
			0x10 | 0x11 => {
				println!("Set Console Vector");
			}

			0x00 | 0x01 => {
				println!("Set System Vector");
			}

			0x02 => {
				if val != 0 {
					self.wst = (val as usize) * 0x100;
				} else {
					self.wst = 0x10000;
				}
			}

			0x18 => {
				println!("0x18 - {}", val);
			}

			0x0f => {
				println!("Prgram Ended");
			}

			_ => {
				println!("Unknown DEO - {}", port);
			}
		}
	}

	pub fn DEVR(&self, port: usize) -> u8 {
		if self.r2 {
			return self.DEI(port).wrapping_shl(8) + self.DEI(port + 1);
		} else {
			return self.DEI(port)
		}
	}

	pub fn DEVW(&mut self, port: usize, val: u8) {
		if self.r2 {
			self.DEO(port, val.wrapping_shr(8));
			self.DEO(port + 1, val & 0xff);
		} else {
			self.DEO(port, val);
		}
	}

	pub fn POKE8(&mut self, addr: usize, val: u8) {
		self.ram[addr] = val;
	}

	pub fn POKE(&mut self, addr: usize, val: u8) {
		if self.r2 {
			self.ram[addr] = val.wrapping_shr(8);
			self.ram[addr + 1] = val;
		} else {
			self.POKE8(addr, val);
		}
	}

	pub fn JUMP(&self, addr: usize) -> usize {
		if self.r2 {
			return addr
		} else {
			return self.pc + self.rel(addr)
		}
	}

	pub fn DST_PUSH16(&mut self, s: u8) {
		self.DST_PUSH8(s.wrapping_shr(0x08));
		self.DST_PUSH8(s & 0xff);
	}

	pub fn DST_PUSH8(&mut self, s: u8) {
		if self.ptr() == 0xff {
			panic!("OVERFLOW");
		}
		let index = self.dst_inc();
		self.ram[self.dst + index] = s;
	}

	pub fn PUSH16(&mut self, s: u8) {
		self.PUSH8(s.wrapping_shr(0x08));
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

	pub fn POP16(&mut self) -> u8 {
		return self.POP8().wrapping_add(self.POP8().wrapping_shl(8));
	}

	pub fn POP8(&mut self) -> u8 {
		println!("POP: {:?}", self.ptr());

		if self.ptr() == 0x00 {
			panic!("UNDERFLOW");
		}

		let index = self.dec();
		return self.ram[self.src + index];
	}

	pub fn POP(&mut self) -> u8 {
		if self.bs != 0 {
            self.POP16()
        } else {
            self.POP8()
        }		
	}

	pub fn PEEK16(&self, x: usize) -> u8 {
		return self.ram[x].wrapping_shl(8).wrapping_add(self.ram[x + 1])
	}

	pub fn PEEK(&self, x: usize) -> u8 {
		if self.bs != 0 {
            self.PEEK16(x)
        } else {
            self.ram[x]
        }
	}
}