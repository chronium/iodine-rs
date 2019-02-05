#[derive(Debug)]
pub enum Opcode {
    Nop,
    LoadConst,
}

impl From<u8> for Opcode {
    fn from(val: u8) -> Opcode {
        match val {
            0x00 => Opcode::Nop,
            0x06 => Opcode::LoadConst,
            _ => panic!("Unknown opcode: 0x{:02x}", val),
        }
    }
}
