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
				// if val == 0x0a{
				// 	println!();
				// } else {

					// match char::from_u32(val.into()) {
					// 	Some(c) => print!("{}", c),
					// 	None => {},
					// }

					print!("{:02x} ", val);
					self.N = self.N + 1;

				// }
			}

			0x0f => {
				// println!("\nProgram Ended");
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

	pub fn DEVW(&mut self, port: usize, val: u16) {
		if self.r2 {
			self.DEO(port, ((val as i32) >> 8) as u8);
			self.DEO((port + 1) & 0xff, (val & 0xff) as u8);
		} else {
			self.DEO(port, (val & 0xff) as u8);
		}
	}

	pub fn POKE8(&mut self, addr: usize, val: u8) {
		self.ram[addr] = val;
	}

	pub fn POKE(&mut self, addr: usize, val: u16) {
		if self.r2 {
			self.ram[addr] = val.wrapping_shr(8) as u8;
			self.ram[addr + 1] = val as u8;
		} else {
			self.POKE8(addr, val as u8);
		}
	}

	pub fn JUMP(&self, addr: usize) -> usize {
		if self.r2 {
			return addr
		} else {
			return self.pc + self.rel(addr)
		}
	}

	pub fn DST_PUSH16(&mut self, s: u16) {
		self.DST_PUSH8(s.wrapping_shr(0x08) as u8);
		self.DST_PUSH8((s & 0xff) as u8);
	}

	pub fn DST_PUSH8(&mut self, s: u8) {
		if self.dst_ptr() == 0xff {
			panic!("OVERFLOW");
		}
		let index = self.dst_inc();
		self.ram[self.dst + index] = s;
	}

	pub fn PUSH16(&mut self, s: u16) {

		let a = (s as i32) >> 0x08;
		let b = s & 0xff;;

		self.PUSH8(a as u8);
		self.PUSH8(b as u8);
	}

	pub fn PUSH8(&mut self, s: u8) {
		if self.ptr() == 0xff {
			panic!("OVERFLOW");
		}
		let index = self.inc();
		self.ram[self.src + index] = s;
	}

	pub fn PUSH(&mut self, s: u16) {
		if self.bs != 0 {
            self.PUSH16(s);
        } else {
            self.PUSH8(s as u8);
        }		
	}

	pub fn POP16(&mut self) -> u16 {
		let a = self.POP8() as i32;
		let b = (self.POP8() as i32) << 8;

		return (a + b) as u16
	}

	pub fn POP8(&mut self) -> u8 {

		if self.ptr() == 0x00 {
			panic!("UNDERFLOW");
		}

		let index = self.dec();
		return self.ram[self.src + index];
	}

	pub fn POP(&mut self) -> u16 {
		if self.bs != 0 {
            return self.POP16()
        } else {
            return self.POP8() as u16
        }		
	}

	pub fn PEEK16(&self, x: usize) -> u16 {
		let a = (self.ram[x] as i32) << 8;
		let b = (self.ram[x+1] as i32);

		return (a + b) as u16
	}

	pub fn PEEK(&self, x: usize) -> u16 {
		// println!("inside x: {:?}", x);

		if self.bs != 0 {
			// println!("{:?}", self.PEEK16(x));
            return self.PEEK16(x)
        } else {
            return self.ram[x] as u16
        }
	}
}