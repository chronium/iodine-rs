#[derive(Debug)]
pub enum Opcode {
    Nop,
    LoadConst,
    LoadGlobal,
    Invoke,
    Pop,
}

impl From<u8> for Opcode {
    fn from(val: u8) -> Opcode {
        match val {
            0x00 => Opcode::Nop,
            0x03 => Opcode::Pop,
            0x06 => Opcode::LoadConst,
            0x0D => Opcode::LoadGlobal,
            0x14 => Opcode::Invoke,
            _ => panic!("Unknown opcode: 0x{:02x}", val),
        }
    }
}
