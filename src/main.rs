use std::{
    fs::File,
    io,
    io::prelude::*,
    io::{Error, ErrorKind},
};

use byteorder::{LittleEndian, ReadBytesExt};
use leb128;

const MAGIC: [u8; 5] = [0x49, 0x4F, 0x57, 0x49, 0x5A];

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => panic!(err),
    }
}

struct CompiledHeader {
    version: [u8; 3],
    name: String,
}

impl CompiledHeader {
    fn ver_to_str(&self) -> String {
        format!(
            "{}.{}.{}",
            self.version[0], self.version[1], self.version[2]
        )
    }
}

fn run() -> io::Result<()> {
    let mut file = File::open("helloworld.bytecode")?;

    for i in 0..5 {
        if file.read_u8().unwrap() != MAGIC[i] {
            return Err(Error::new(ErrorKind::Other, "Magic unknown"));
        }
    }

    let mut version = [0; 3];
    file.read_exact(&mut version)?;
    file.read_u64::<LittleEndian>()?;

    let name_len = leb128::read::unsigned(&mut file).expect("Name length malformed");
    let mut name = Vec::<u8>::new();

    for _ in 0..name_len {
        name.push(file.read_u8().unwrap());
    }

    let header = CompiledHeader {
        version,
        name: String::from_utf8(name).unwrap(),
    };

    println!("Expected iodine version: {}", header.ver_to_str());
    println!("Filename: {}", header.name);

    Ok(())
}
