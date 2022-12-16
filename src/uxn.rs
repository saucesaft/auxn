use crate::system::{Device, Opcode};

use arrayvec::ArrayVec;

const MAX_INSTR: u8 = 0x1f;

pub struct UXN {
    pub ram: ArrayVec::<u8, 0x13000>,
    
    wst: usize,
    rst: usize,
    dev: usize,

    pub src: usize,
    dst: usize,

    pub bs: usize,
    pub pk: usize,

    pub r2: bool,
    pub rr: bool,
    pub rk: bool,
}

impl UXN {
    pub fn new() -> Self {
        let u = UXN {
            ram: ArrayVec::<u8, 0x13000>::from([0; 0x13000]),
            
            wst: 0x10000,
            rst: 0x11000,
            dev: 0x12000,

            src: 0,
            dst: 0,

            bs: 0,
            pk: 0,

            r2: false,
            rr: false,
            rk: false,

        };

        let (system_indev, system_outdev) = crate::system::system_devices();
        let (none_indev, none_outdev) = crate::system::none_devices();
        let (file_indev, file_outdev) = crate::system::file_devices();

        // assign ports to each device
        u.port(0x0, &system_indev, &system_outdev);
        u.port(0x1, &none_indev, &none_outdev);
        u.port(0x2, &none_indev, &none_outdev);
        u.port(0x3, &none_indev, &none_outdev);
        u.port(0x4, &none_indev, &none_outdev);
        u.port(0x5, &none_indev, &none_outdev);
        u.port(0x6, &none_indev, &none_outdev);
        u.port(0x7, &none_indev, &none_outdev);
        u.port(0x8, &none_indev, &none_outdev);
        u.port(0x9, &none_indev, &none_outdev);
        u.port(0xa, &file_indev, &file_outdev);
        u.port(0xb, &file_indev, &file_outdev);
        u.port(0xc, &none_indev, &none_outdev);
        u.port(0xd, &none_indev, &none_outdev);
        u.port(0xf, &none_indev, &none_outdev);

        return u;
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for (i, instr) in program.iter().enumerate() {
            self.ram[0x100 + i] = *instr;
        }
    }

    pub fn wst_get(&self, index: usize) -> u8 {
        return self.ram[self.wst + index]
    }

    pub fn rst_get(&self, index: usize) -> u8 {
        return self.ram[self.rst + index]
    }

    pub fn dev_get(&self, index: usize) -> u8 {
        return self.ram[self.dev + index]
    }

    pub fn get(&self, index: usize) -> u8 {
        return self.ram[self.src + index];
    } 

    pub fn ptr(&self) -> u8 {
        return self.get(0xff);
    }

    pub fn inc(&mut self) -> usize {
        self.ram[self.src + 0xff] += 1;

        return self.ram[self.src + 0xff].into();
    }

    pub fn dec(&self) -> usize {
        if self.rk {
            return self.pk - 1;
        } else {
            return (self.ram[self.src + 0xff] - 1).into();
        }
    }

    fn port(&self, id: u8, indev: &Device, outdev: &Device) {}

    // move to external?
    fn interrupt(&self) -> u8 {
        return 1;
    }

    pub fn eval(&mut self, mut pc: usize) -> u8 {

        let mut current_block: u64;

        // registers
        let mut a: u8 = 0;
        let mut b: u8 = 0;
        let mut c: u8 = 0;

        let mut kptr: u8 = 0;
        let sp:  u8 = 0;

        let mut limit: u64 = 0x40000;
        let mut errcode = 0;

        if pc == 0 || self.dev_get(0xf) != 0 {
            return 0;
        }

        let mut keep: bool = false;

        let mut instr: u8 = 0;

        loop {
            instr = self.ram[pc];
            pc = pc.wrapping_add(1);

            if instr == 0 {
                current_block = 13224773276258604431;
                break;
            }

            let fresh1 = limit;
            limit = limit.wrapping_sub(1);
            if fresh1 == 0 {
                if self.interrupt() == 0 {
                    errcode = 6;
                    current_block = 14562848177075201790;
                } else {
                    limit = 0x40000;
                }
            }

            self.r2 = instr & 0x20 != 0;
            self.rr = instr & 0x40 != 0;
            self.rk = instr & 0x80 != 0;

            // short-mode ?
            if self.r2 {
                self.bs = 1;
            } else {
                self.bs = 0;
            }

            // return-mode ?
            if self.rr {
                self.src = self.rst;
                self.dst = self.wst;
            } else {
                self.src = self.wst;
                self.dst = self.rst;
            }

            // keep-mode ?
            if self.rk {
                self.pk = self.ptr() as usize;
            }

            match Opcode::try_from(instr & MAX_INSTR) {
                Ok(Opcode::LIT) => {
                    self.PUSH( self.PEEK(pc) );
                    pc += 1 + self.bs
                }

                Ok(Opcode::INC) => {
                    let x = self.POP();
                    self.PUSH( x + 1 );
                }

                Ok(Opcode::POP) => {
                    self.POP();
                }

                Ok(Opcode::NIP) => {
                    a = self.POP();
                    self.POP();
                    self.PUSH(a);
                }

                Ok(Opcode::SWP) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(a);
                    self.PUSH(b);
                }

                Ok(Opcode::ROT) => {
                    a = self.POP();
                    b = self.POP();
                    c = self.POP();
                    self.PUSH(a);
                    self.PUSH(b);
                    self.PUSH(c);
                }

                Ok(Opcode::DUP) => {
                    a = self.POP();
                    self.PUSH(a);
                    self.PUSH(a);
                }

                Ok(Opcode::OVR) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b);
                    self.PUSH(a);
                    self.PUSH(b);    
                }

                Ok(Opcode::EQU) => {
                    a = self.POP();
                    b = self.POP();
                    if b == a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::NEQ) => {
                    a = self.POP();
                    b = self.POP();
                    if b != a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::GTH) => {
                    a = self.POP();
                    b = self.POP();
                    if b > a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::LTH) => {
                    a = self.POP();
                    b = self.POP();
                    if b < a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::JMP) => {
                    println!("JMP")
                }

                Ok(Opcode::JCN) => {
                    println!("JCN")
                }

                Ok(Opcode::JSR) => {
                    println!("JSR")
                }

                Ok(Opcode::STH) => {
                    println!("STH")
                }

                Ok(Opcode::LDZ) => {
                    println!("LDZ")
                }

                Ok(Opcode::STZ) => {
                    println!("STZ")
                }

                Ok(Opcode::LDR) => {
                    println!("LDR")
                }

                Ok(Opcode::STR) => {
                    println!("STR")
                }

                Ok(Opcode::LDA) => {
                    println!("LDA")
                }

                Ok(Opcode::STA) => {
                    println!("STA")
                }

                Ok(Opcode::DEI) => {
                    println!("DEI")
                }

                Ok(Opcode::DEO) => {
                    println!("DEO")
                }

                Ok(Opcode::ADD) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b + a);
                }

                Ok(Opcode::SUB) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b - a);
                }

                Ok(Opcode::MUL) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b * a);
                }

                Ok(Opcode::DIV) => {
                    println!("DIV")
                }

                Ok(Opcode::AND) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b & a);
                }

                Ok(Opcode::ORA) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b | a);
                }

                Ok(Opcode::EOR) => {
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b ^ a);
                }

                Ok(Opcode::SFT) => {
                    println!("SFT")
                }

                Err(_) => {
                    eprintln!("unknown instruction :(")
                }
            }

        }

        return 1;

    }

}