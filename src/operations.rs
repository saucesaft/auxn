use crate::uxn::UXN;

impl UXN {
    pub fn rel(&self, val: usize) -> usize {
        if val > 0x80 {
            val.wrapping_sub(256)
        } else {
            val
        }
    }

    pub fn DEI(&self, port: usize) -> u8 {
        return self.ram[self.dev + port];
    }

    pub fn DEO(&mut self, port: usize, val: u8) {
        self.ram[self.dev + port] = val;

        match port & 0xF0 {
            0x00 => crate::devices::system(self, port, val),

            0x10 => crate::devices::console(self, port, val),

            0x20 => crate::devices::screen(self, port, val),

            _ => println!("Unknown DEV PORT: {:x?}", port & 0xF0),
        }
    }

    pub fn DEVR(&self, port: usize) -> u16 {
        if self.r2 {
            return (((self.DEI(port) as i32) << 8) + (self.DEI(port + 1) as i32)) as u16;
        } else {
            return self.DEI(port).into();
        }
    }

    pub fn DEVW(&mut self, port: usize, val: u16) {
        if self.r2 {
            self.DEO(port, ((val as i32) >> 8) as u8);
            self.DEO(port + 1, (val & 0xff) as u8);
        } else {
            self.DEO(port, val as u8);
        }
    }

    pub fn POKE8(&mut self, addr: usize, val: u8) {
        self.ram[addr] = val;
    }

    pub fn POKE(&mut self, addr: usize, val: u16) {
        if self.r2 {
            self.ram[addr] = ((val as i32) >> 8) as u8;
            self.ram[addr + 1] = val as u8;
        } else {
            self.POKE8(addr, val as u8);
        }
    }

    pub fn JUMP(&self, addr: usize, pc: usize) -> usize {
        if self.r2 {
            return addr;
        } else {
            return pc.wrapping_add(self.rel(addr));
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
        let b = s & 0xff;

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
        if self.r2 {
            self.PUSH16(s);
        } else {
            self.PUSH8(s as u8);
        }
    }

    pub fn POP16(&mut self) -> u16 {
        let a = self.POP8() as i32;
        let b = (self.POP8() as i32) << 8;

        return (a + b) as u16;
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
            return self.POP16();
        } else {
            return self.POP8() as u16;
        }
    }

    pub fn PEEK16(&self, x: usize) -> u16 {
        let a = (self.ram[x] as i32) << 8;
        let b = (self.ram[x + 1] as i32);

        return (a + b) as u16;
    }

    pub fn PEEK(&self, x: usize) -> u16 {
        if self.r2 {
            return self.PEEK16(x);
        } else {
            return self.ram[x] as u16;
        }
    }
}
