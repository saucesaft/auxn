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
        let mut j: u8 = 0;
        let mut k: u8 = 0;

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

            // short-mode ?
            if instr & 0x20 != 0 {
                self.bs = 1;
            } else {
                self.bs = 0;
            }

            // return-mode ?
            if instr & 0x40 != 0 {
                self.src = self.rst;
                self.dst = self.wst;
            } else {
                self.src = self.wst;
                self.dst = self.rst;
            }

            // keep-mode ?
            if instr & 0x80 != 0 {
                kptr = 
            }

            match Opcode::try_from(instr & MAX_INSTR) {
                Ok(Opcode::LIT) => {
                    self.PUSH( self.PEEK(pc) );
                    pc += 1 + self.bs
                }

                Ok(Opcode::INC) => {
                    self.PUSH( self.POP() + 1 );
                }

                Ok(Opcode::POP) => {
                    println!("POP")
                }

                Ok(Opcode::NIP) => {
                    println!("NIP")
                }

                Ok(Opcode::SWP) => {
                    println!("SWP")
                }

                Ok(Opcode::ROT) => {
                    println!("ROT")
                }

                Ok(Opcode::DUP) => {
                    println!("DUP")
                }

                Ok(Opcode::OVR) => {
                    println!("OVR")
                }

                Ok(Opcode::EQU) => {
                    println!("EQU")
                }

                Ok(Opcode::NEQ) => {
                    println!("NEQ")
                }

                Ok(Opcode::GTH) => {
                    println!("GTH")
                }

                Ok(Opcode::LTH) => {
                    println!("LTH")
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
                    println!("ADD")
                }

                Ok(Opcode::SUB) => {
                    println!("SUB")
                }

                Ok(Opcode::MUL) => {
                    println!("MUL")
                }

                Ok(Opcode::DIV) => {
                    println!("DIV")
                }

                Ok(Opcode::AND) => {
                    println!("AND")
                }

                Ok(Opcode::ORA) => {
                    println!("ORA")
                }

                Ok(Opcode::EOR) => {
                    println!("EOR")
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