#![feature(box_syntax)]

use std::{
    fs::File,
    io,
    io::prelude::*,
    io::{Error, ErrorKind},
};

use byteorder::{LittleEndian, ReadBytesExt};
use leb128;

use iodine_runtime::{opcode::Opcode, Instruction, IodineObjects};

const MAGIC: [u8; 5] = [0x49, 0x4F, 0x57, 0x49, 0x5A];

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => panic!(err),
    }
}

struct CompiledHeader {
    version: [u8; 3],
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

    let header = CompiledHeader { version };

    println!("Expected iodine version: {}", header.ver_to_str());

    let module = read_module(&mut file);

    file.read_u8();

    Ok(())
}

fn read_string(file: &mut File) -> io::Result<String> {
    let str_len = leb128::read::unsigned(file).expect("Name length malformed");
    let mut str_bytes = Vec::<u8>::new();

    for _ in 0..str_len {
        str_bytes.push(file.read_u8().unwrap());
    }

    Ok(String::from_utf8(str_bytes).unwrap())
}

fn read_module(file: &mut File) -> io::Result<IodineObjects> {
    let name = read_string(file)?;
    file.read_u8()?;

    println!("Encountered module: {}", name);

    Ok(IodineObjects::IodineModule {
        name,
        code: box read_code_object(file)?,
    })
}

fn read_code_object(file: &mut File) -> io::Result<IodineObjects> {
    let mut code = IodineObjects::CodeObject {
        instructions: Vec::new(),
    };

    let instruction_count = file.read_u32::<LittleEndian>()?;

    println!("Module instruction count: {}", instruction_count);

    for _ in 0..instruction_count {
        code.push_instruction(read_instruction(file)?);
    }

    Ok(code)
}

fn read_constant(file: &mut File) -> io::Result<IodineObjects> {
    let iodine_type = DataType::from(file.read_u8()?);
    println!("Encountered type: {:?}", iodine_type);

    match iodine_type {
        DataType::StringObject => {
            return Ok(IodineObjects::IodineString {
                value: read_string(file)?,
            });
        }
        _ => unimplemented!(),
    }
}

fn read_instruction(file: &mut File) -> io::Result<Instruction> {
    let opcode = Opcode::from(file.read_u8()?);
    println!("Opcode: {:?}", opcode);

    let argument = file.read_i32::<LittleEndian>()?;
    println!("Argument: {}", argument);

    let argument_obj = read_constant(file)?;
    println!("Argument Object: {:?}", argument_obj);

    let _line = file.read_i32::<LittleEndian>();

    Ok(Instruction {
        opcode: opcode,
        argument: argument,
        object: IodineObjects::IodineNull,
    })
}

#[derive(Debug)]
enum DataType {
    StringObject,
}

impl From<u8> for DataType {
    fn from(iodine_type: u8) -> DataType {
        match iodine_type {
            0x02 => DataType::StringObject,
            _ => unimplemented!(),
        }
    }
}
