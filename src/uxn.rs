use crate::system::{Device, Opcode};
use crate:: devices::*;
use std::sync::mpsc;

use arrayvec::ArrayVec;

const MAX_INSTR: u8 = 0x1f;

pub struct UXN {
    pub ram: ArrayVec::<u8, 0x13000>,
    
    pub wst: usize,
    pub rst: usize,
    pub dev: usize,

    pub src: usize,
    pub dst: usize,

    pub bs: usize,
    pub pk: usize,

    pub r2: bool,
    pub rr: bool,
    pub rk: bool,

    pub a: u16,
    pub b: u16,
    pub c: u16,

    pub halted: bool,
    pub limit: u64,

    pub system: SystemDevice,
    pub console: ConsoleDevice,
    pub screen: ScreenDevice,
}

impl UXN {
    pub fn new(w: u32, h: u32) -> Self {
        UXN {
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

            // registers
            a: 0,
            b: 0,
            c: 0,

            halted: false,
            limit: 0x40000,

            system: SystemDevice::new(),
            console: ConsoleDevice::new(),
            screen: ScreenDevice::new(w, h),
        }
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

    pub fn dst_get(&self, index: usize) -> u8 {
        return self.ram[self.dst + index];
    } 

    pub fn ptr(&self) -> u8 {
        return self.get(0xff);
    }

    pub fn inc(&mut self) -> usize {
        let val = self.ram[self.src + 0xff].clone();

        self.ram[self.src + 0xff] = self.ram[self.src + 0xff] + 1;

        return val.into();
    }

    pub fn dst_inc(&mut self) -> usize {
        let val = self.ram[self.dst + 0xff];

        self.ram[self.dst + 0xff] = self.ram[self.dst + 0xff] + 1;

        return val.into();
    }

    pub fn dst_ptr(&self) -> u8 {
        return self.dst_get(0xff);
    }

    pub fn dec(&mut self) -> usize {
        if self.rk {
            self.pk = self.pk - 1;
            return self.pk;
        } else {
            self.ram[self.src + 0xff] = self.ram[self.src + 0xff] - 1;
            return (self.ram[self.src + 0xff]).into();
        }
    }

    // move to external?
    fn interrupt(&self) -> u8 {
        return 1;
    }

    fn reset(&mut self) {
        self.halted = false;
        self.limit = 0x40000;

        self.wst = 0x10000;
        self.rst = 0x11000;
        self.dev = 0x12000;

        self.src = 0;
        self.dst = 0;

        self.bs = 0;
        self.pk = 0;

        self.r2 = false;
        self.rr = false;
        self.rk = false;

        self.a = 0;
        self.b = 0;
        self.c = 0;
    }

    pub fn eval(&mut self, mut pc: usize) {
        if pc == 0 || self.dev_get(0xf) != 0 {
            return;
        }

        while !self.halted {
            pc = self.step(pc);
        }

        self.reset();
    }

    pub fn step(&mut self, mut pc: usize) -> usize {

        let debug = false;
    
        let instr = self.ram[pc];
        pc = pc.wrapping_add(1);

        if instr == 0 {
            self.halted = true;
        }

        let fresh1 = self.limit;
        self.limit = self.limit.wrapping_sub(1);
        if fresh1 == 0 {
            if self.interrupt() == 0 {
                self.halted = true;
            } else {
                self.limit = 0x40000;
            }
        }

        self.r2 = instr & 0x20 != 0;
        self.rr = instr & 0x40 != 0;
        self.rk = instr & 0x80 != 0;

        // short-mode
        if self.r2 {
            self.bs = 1;
        } else {
            self.bs = 0;
        }

        // return-mode
        if self.rr {
            self.src = self.rst;
            self.dst = self.wst;
        } else {
            self.src = self.wst;
            self.dst = self.rst;
        }

        // keep-mode
        if self.rk {
            self.pk = self.ptr() as usize;
        }

        // println!("rk: {:?}", self.rk);

        match Opcode::try_from(instr & MAX_INSTR) {
            Ok(Opcode::LIT) => {
                if debug {
                    println!("-> LIT");    
                }

                self.PUSH( self.PEEK(pc));
                pc = pc.wrapping_add(1).wrapping_add(self.bs);
            }

            Ok(Opcode::INC) => {
                if debug {
                    println!("-> INC");    
                }
                
                let x = self.POP();

                self.PUSH( x.wrapping_add(1).into() );
            }

            Ok(Opcode::POP) => {
                if debug {
                    println!("-> POP");    
                }
                
                self.POP();
            }

            Ok(Opcode::NIP) => {
                if debug {
                    println!("-> NIP");    
                }
                
                self.a = self.POP();
                self.POP();
                self.PUSH(self.a.into());
            }

            Ok(Opcode::SWP) => {
                if debug {
                    println!("-> SWP");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.a);
                self.PUSH(self.b);
            }

            Ok(Opcode::ROT) => {
                if debug {
                    println!("-> ROT");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.c = self.POP();
                self.PUSH(self.b);
                self.PUSH(self.a);
                self.PUSH(self.c);
            }

            Ok(Opcode::DUP) => {
                if debug {
                    println!("-> DUP");    
                }
                
                self.a = self.POP();
                self.PUSH(self.a);
                self.PUSH(self.a);
            }

            Ok(Opcode::OVR) => {
                if debug {
                    println!("-> OVR");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b);
                self.PUSH(self.a);
                self.PUSH(self.b);    
            }

            Ok(Opcode::EQU) => {
                if debug {
                    println!("-> EQU");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                if self.b == self.a {
                    self.PUSH8(1);
                } else {
                    self.PUSH8(0);
                }
            }

            Ok(Opcode::NEQ) => {
                if debug {
                    println!("-> NEQ");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                if self.b != self.a {
                    self.PUSH8(1);
                } else {
                    self.PUSH8(0);
                }
            }

            Ok(Opcode::GTH) => {
                if debug {
                    println!("-> GTH");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                if self.b > self.a {
                    self.PUSH8(1);
                } else {
                    self.PUSH8(0);
                }
            }

            Ok(Opcode::LTH) => {
                if debug {
                    println!("-> LTH");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                if self.b < self.a {
                    self.PUSH8(1);
                } else {
                    self.PUSH8(0);
                }
            }

            Ok(Opcode::JMP) => {
                if debug {
                    println!("-> JMP");    
                }
                
                let x = self.POP().into();
                pc = self.JUMP( x, pc );
            }

            Ok(Opcode::JCN) => {
                if debug {
                    println!("-> JCN");    
                }
                
                self.a = self.POP();
                if self.POP8() != 0 {
                    pc = self.JUMP(self.a.into(), pc);
                }
            }

            Ok(Opcode::JSR) => {
                if debug {
                    println!("-> JSR");    
                }
                
                self.DST_PUSH16(pc.try_into().unwrap());
                let x = self.POP().into();
                pc = self.JUMP( x, pc );
            }

            Ok(Opcode::STH) => {
                if debug {
                    println!("-> STH");    
                }
                
                if self.r2 {
                    let x = self.POP16();
                    self.DST_PUSH16(x);
                } else {
                    let x = self.POP8(); //here
                    self.DST_PUSH8(x);
                }
            }

            Ok(Opcode::LDZ) => {
                if debug {
                    println!("-> LDZ");    
                }
                
                let x = self.POP8();
                self.PUSH( self.PEEK( x.into() ) );
            }

            Ok(Opcode::STZ) => {
                if debug {
                    println!("-> STZ");    
                }
                
                let x = self.POP8();
                let y = self.POP();

                self.POKE(x.into(), y);
            }

            Ok(Opcode::LDR) => {
                if debug {
                    println!("-> LDR");    
                }
                
                let x = self.POP8();

                self.PUSH( self.PEEK( pc.wrapping_add( self.rel( x.into() ) ) ) );
            }

            Ok(Opcode::STR) => {
                if debug {
                    println!("-> STR");    
                }
                
                let x = self.POP8();
                let y = self.POP();

                self.POKE(pc.wrapping_add(self.rel(x.into())), y);
            }

            Ok(Opcode::LDA) => {
                if debug {
                    println!("-> LDA");    
                }
                
                let x = self.POP16();

                self.PUSH( self.PEEK( x.into() ) );
            }

            Ok(Opcode::STA) => {
                if debug {
                    println!("-> STA");    
                }
                
                let x = self.POP16();
                let y = self.POP();

                self.POKE(x.into(), y);
            }

            Ok(Opcode::DEI) => {
                if debug {
                    println!("-> DEI");    
                }
                
                let x = self.POP8();

                self.PUSH( self.DEVR( x.into() ).into() );
            }

            Ok(Opcode::DEO) => {
                if debug {
                    println!("-> DEO");    
                }
                
                let x = self.POP8();
                let y = self.POP();

                self.DEVW(x.into(), y);
            }

            Ok(Opcode::ADD) => {
                if debug {
                    println!("-> ADD");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b.wrapping_add(self.a));
            }

            Ok(Opcode::SUB) => {
                if debug {
                    println!("-> SUB");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b.wrapping_sub(self.a));
            }

            Ok(Opcode::MUL) => {
                if debug {
                    println!("-> MUL");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b.wrapping_mul(self.a));
            }

            Ok(Opcode::DIV) => {
                if debug {
                    println!("-> DIV");    
                }

                self.a = self.POP();
                self.b = self.POP();

                if self.a == 0 {
                    panic!("IMPOSIBLE DIVISION");
                }

                self.PUSH(self.b.wrapping_div(self.a));
                
            }

            Ok(Opcode::AND) => {
                if debug {
                    println!("-> AND");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b & self.a);
            }

            Ok(Opcode::ORA) => {
                if debug {
                    println!("-> ORA");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b | self.a);
            }

            Ok(Opcode::EOR) => {
                if debug {
                    println!("-> EOR");    
                }
                
                self.a = self.POP();
                self.b = self.POP();
                self.PUSH(self.b ^ self.a);
            }

            Ok(Opcode::SFT) => {
                if debug {
                    println!("-> SFT");    
                }

                self.a = self.POP8().into();
                self.b = self.POP();

                self.PUSH( self.b >> (self.a & 0x0f) << ((self.a & 0xf0) >> 4) );
                
            }

            Err(_) => {
                panic!("unknown instruction :(")
            }
        }

            // println!("ptr: {}", self.ptr());

            // let wst = &self.ram[self.wst..self.wst+20];
            // let rst = &self.ram[self.rst..self.rst+20];

            // println!("pc: {:#x?} instr: {:#x?}", pc, instr & MAX_INSTR);
            // println!("rr: {:?} r2: {:?}", self.rr, self.r2);
            // println!("wst: {:x?}", wst);
            // println!("rst: {:x?}", rst);
            // println!("debug_out: {}\n", debug_out as u8);

            // if pc >= 0x1e0 {
            //     break
            // }

        return pc;

    }

}