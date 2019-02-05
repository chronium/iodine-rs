pub enum Opcode {
    Nop,
}

impl From<u8> for Opcode {
    fn from(val: u8) -> Opcode {
        match val {
            0 => Opcode::Nop,
            _ => panic!("Unknown opcode: 0x{:02x}", val),
        }
    }
}
