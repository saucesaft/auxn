use crate::system::{Device, Opcode};

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

    pub pc: usize,

    pub N: i32,
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

            pc: 0,

            N: 1,

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

    pub fn dst_get(&self, index: usize) -> u8 {
        return self.ram[self.dst + index];
    } 

    pub fn ptr(&self) -> u8 {
        return self.get(0xff);
    }

    pub fn inc(&mut self) -> usize {
        let val = self.ram[self.src + 0xff];

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

    fn port(&self, id: u8, indev: &Device, outdev: &Device) {}

    // move to external?
    fn interrupt(&self) -> u8 {
        return 1;
    }

    pub fn eval(&mut self, pc: usize) -> u8 {

        self.pc = pc;

        let mut current_block: u64;

        // registers
        let mut a: u16 = 0;
        let mut b: u16 = 0;
        let mut c: u16 = 0;

        let mut kptr: u8 = 0;
        let sp:  u8 = 0;

        let mut limit: u64 = 0x40000;
        let mut errcode = 0;

        if self.pc == 0 || self.dev_get(0xf) != 0 {
            return 0;
        }

        let mut keep: bool = false;

        let mut instr: u8 = 0;

        let debug = false;

        loop {
            instr = self.ram[self.pc];
            self.pc = self.pc.wrapping_add(1);

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

            let mut debug_out: u16 = 0;

            // print!("PC: {:?} ", self.pc);
            match Opcode::try_from(instr & MAX_INSTR) {
                Ok(Opcode::LIT) => {
                    if debug {
                        println!("-> LIT");    
                    }

                    debug_out = self.PEEK(self.pc);

                    self.PUSH( self.PEEK(self.pc));
                    self.pc = self.pc.wrapping_add(1).wrapping_add(self.bs);
                }

                Ok(Opcode::INC) => {
                    if debug {
                        println!("-> INC");    
                    }
                    
                    let x = self.POP();
                    self.PUSH( (x + 1).into() );
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
                    
                    a = self.POP();
                    self.POP();
                    self.PUSH(a.into());
                }

                Ok(Opcode::SWP) => {
                    if debug {
                        println!("-> SWP");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(a);
                    self.PUSH(b);
                }

                Ok(Opcode::ROT) => {
                    if debug {
                        println!("-> ROT");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    c = self.POP();
                    self.PUSH(b);
                    self.PUSH(a);
                    self.PUSH(c);
                }

                Ok(Opcode::DUP) => {
                    if debug {
                        println!("-> DUP");    
                    }
                    
                    a = self.POP();
                    self.PUSH(a);
                    self.PUSH(a);
                }

                Ok(Opcode::OVR) => {
                    if debug {
                        println!("-> OVR");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b);
                    self.PUSH(a);
                    self.PUSH(b);    
                }

                Ok(Opcode::EQU) => {
                    if debug {
                        println!("-> EQU");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    if b == a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::NEQ) => {
                    if debug {
                        println!("-> NEQ");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    if b != a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::GTH) => {
                    if debug {
                        println!("-> GTH");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    if b > a {
                        self.PUSH8(1);
                    } else {
                        self.PUSH8(0);
                    }
                }

                Ok(Opcode::LTH) => {
                    if debug {
                        println!("-> LTH");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    if b < a {
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
                    self.pc = self.JUMP( x );
                }

                Ok(Opcode::JCN) => {
                    if debug {
                        println!("-> JCN");    
                    }
                    
                    a = self.POP();
                    if self.POP8() != 0 {
                        self.pc = self.JUMP(a.into());
                    }
                }

                Ok(Opcode::JSR) => {
                    if debug {
                        println!("-> JSR");    
                    }
                    
                    self.DST_PUSH16(self.pc.try_into().unwrap());
                    let x = self.POP().into();
                    self.pc = self.JUMP( x );
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

                    self.PUSH( self.PEEK( self.pc + self.rel(x.into()) ) );
                }

                Ok(Opcode::STR) => {
                    if debug {
                        println!("-> STR");    
                    }
                    
                    let x = self.POP8();
                    let y = self.POP();

                    self.POKE(self.pc + self.rel(x.into()), y);
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
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b.wrapping_add(a));
                }

                Ok(Opcode::SUB) => {
                    if debug {
                        println!("-> SUB");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b.wrapping_sub(a));
                }

                Ok(Opcode::MUL) => {
                    if debug {
                        println!("-> MUL");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b.wrapping_mul(a));
                }

                Ok(Opcode::DIV) => {
                    if debug {
                        println!("-> DIV");    
                    }

                    a = self.POP();
                    b = self.POP();

                    if a == 0 {
                        panic!("IMPOSIBLE DIVISION");
                    }

                    self.PUSH(b.wrapping_div(a));
                    
                }

                Ok(Opcode::AND) => {
                    if debug {
                        println!("-> AND");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b & a);
                }

                Ok(Opcode::ORA) => {
                    if debug {
                        println!("-> ORA");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b | a);
                }

                Ok(Opcode::EOR) => {
                    if debug {
                        println!("-> EOR");    
                    }
                    
                    a = self.POP();
                    b = self.POP();
                    self.PUSH(b ^ a);
                }

                Ok(Opcode::SFT) => {
                    if debug {
                        println!("-> SFT");    
                    }

                    a = self.POP8().into();
                    b = self.POP();

                    // let x = b >> (a & 0x0f) << ((a & 0xf0) >> 4);

                    // println!("a: {:?}", a);
                    // println!("b: {:?}", b);
                    // println!("c: {:?}", x);

                    self.PUSH( b >> (a & 0x0f) << ((a & 0xf0) >> 4) );
                    
                }

                Err(_) => {
                    panic!("unknown instruction :(")
                }
            }

            // let wst = &self.ram[self.wst..self.wst+20];
            // let rst = &self.ram[self.rst..self.rst+20];

            // println!("pc: {:#x?} instr: {:#x?}", self.pc, instr & MAX_INSTR);
            // println!("rr: {:?} r2: {:?}", self.rr, self.r2);
            // println!("wst: {:x?}", wst);
            // println!("rst: {:x?}", rst);
            // println!("debug_out: {}\n", debug_out as u8);

            // // // 0x23e final pc

            // if self.pc >= 0x1e0 {
            //     break
            // }

        }

        return 1;

    }

}