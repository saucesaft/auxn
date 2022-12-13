mod system;

use system::{Stack, Device, Opcode};

use arrayvec::ArrayVec;

const MAX_INSTR: u8 = 0x1f;

// 0x100 = 256
// 0x10000 = 65,536
struct UXN {
    dev: ArrayVec::<u8, 256>,
    ram: ArrayVec::<u8, 65536>,
    
    read_stack: Stack,
    write_stack: Stack,
}

impl UXN {
    fn new() -> Self {
        let u = UXN {
            ram: ArrayVec::<u8, 65536>::from([0; 65536]),
            read_stack: Stack::new(),
            write_stack: Stack::new(),
            dev: ArrayVec::<u8, 256>::from([0; 256]),
        };

        let (system_indev, system_outdev) = system::system_devices();
        let (none_indev, none_outdev) = system::none_devices();
        let (file_indev, file_outdev) = system::file_devices();

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

    fn load(&mut self, program: Vec<u8>) {
        for (i, instr) in program.iter().enumerate() {
            self.ram[0x100 + i] = *instr;
        }
    }

    fn port(&self, id: u8, indev: &Device, outdev: &Device) {}

    // move to external?
    fn interrupt(&self) -> u8 {
        return 1;
    }

    fn eval(&mut self, mut pc: usize) -> u8 {

        let mut current_block: u64;

        let mut bs: u8 = 0;

        let mut kptr: u8 = 0;
        let sp:  u8 = 0;
        let mut src: &mut Stack;
        let mut dst:  &Stack;

        let mut limit: u64 = 0x40000;
        let mut errcode = 0;

        if pc == 0 || self.dev[0xf] != 0 {
            return 0;
        }

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
                bs = 1;
            } else {
                bs = 0;
            }

            // return-mode ?
            if instr & 0x40 != 0 {
                src = &mut self.read_stack;
                dst = &self.write_stack;
            } else {
                src = &mut self.write_stack;
                dst = &self.read_stack;
            }

            // keep-mode ?
            if instr & 0x80 != 0 {
                src.keep(true);
            } else {
                src.keep(false);
            }

            println!("{}", instr);
            println!("{}", instr & MAX_INSTR);

            match Opcode::try_from(instr & MAX_INSTR) {
                Ok(Opcode::LIT) => {
                    println!("LIT")
                }

                Ok(Opcode::INC) => {
                    println!("INC")
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

fn main() {
    let rom = include_bytes!("../tests/arithmetic.rom").to_vec();
    // let rom = include_bytes!("../tests/literals.rom").to_vec();

    let mut vm = UXN::new();
    vm.load(rom);
    vm.eval(0x100);
}
