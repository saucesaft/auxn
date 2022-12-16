pub enum Opcode {
    // Stack
    LIT = 0x00,
    INC = 0x01,
    POP = 0x02,
    NIP = 0x03,
    SWP = 0x04,
    ROT = 0x05,
    DUP = 0x06,
    OVR = 0x07,

    // Logic
    EQU = 0x08,
    NEQ = 0x09,
    GTH = 0x0a,
    LTH = 0x0b,
    JMP = 0x0c,
    JCN = 0x0d,
    JSR = 0x0e,
    STH = 0x0f,

    // Memory
    LDZ = 0x10,
    STZ = 0x11,
    LDR = 0x12,
    STR = 0x13,
    LDA = 0x14,
    STA = 0x15,
    DEI = 0x16,
    DEO = 0x17,

    // Arithmetic
    ADD = 0x18,
    SUB = 0x19,
    MUL = 0x1a,
    DIV = 0x1b,
    AND = 0x1c,
    ORA = 0x1d,
    EOR = 0x1e,
    SFT = 0x1f,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(val: u8) -> Result<Opcode, ()> {
        match val {
            // Stack
            0x00 => Ok(Opcode::LIT),
            0x01 => Ok(Opcode::INC),
            0x02 => Ok(Opcode::POP),
            0x03 => Ok(Opcode::NIP),
            0x04 => Ok(Opcode::SWP),
            0x05 => Ok(Opcode::ROT),
            0x06 => Ok(Opcode::DUP),
            0x07 => Ok(Opcode::OVR),

            // Logic
            0x08 => Ok(Opcode::EQU),
            0x09 => Ok(Opcode::NEQ),
            0x0a => Ok(Opcode::GTH),
            0x0b => Ok(Opcode::LTH),
            0x0c => Ok(Opcode::JMP),
            0x0d => Ok(Opcode::JCN),
            0x0e => Ok(Opcode::JSR),
            0x0f => Ok(Opcode::STH),

            // Memory
            0x10 => Ok(Opcode::LDZ),
            0x11 => Ok(Opcode::STZ),
            0x12 => Ok(Opcode::LDR),
            0x13 => Ok(Opcode::STR),
            0x14 => Ok(Opcode::LDA),
            0x15 => Ok(Opcode::STA),
            0x16 => Ok(Opcode::DEI),
            0x17 => Ok(Opcode::DEO),

            // Arithmetic
            0x18 => Ok(Opcode::ADD),
            0x19 => Ok(Opcode::SUB),
            0x1a => Ok(Opcode::MUL),
            0x1b => Ok(Opcode::DIV),
            0x1c => Ok(Opcode::AND),
            0x1d => Ok(Opcode::ORA),
            0x1e => Ok(Opcode::EOR),
            0x1f => Ok(Opcode::SFT),

            _ => Err(()),
        }
    }
}