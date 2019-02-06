#![feature(box_syntax)]

use std::{
    fs::File,
    io,
    io::prelude::*,
    io::{Error, ErrorKind},
    sync::Arc,
};

use byteorder::{LittleEndian, ReadBytesExt};
use leb128;

use iodine_runtime::{
    code_object, module, name, opcode::Opcode, string, virtual_machine::VirtualMachine,
    Instruction, IodineNull, IodineObject,
};

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

    let module = read_module(&mut file)?;

    let mut vm = VirtualMachine {
        frames: Vec::new(),
        stack_size: 0usize,
        frame_count: 0usize,
    };

    println!("\n-----Execution started-----\n");

    module.invoke(&mut vm, Vec::new());

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

fn read_module(file: &mut File) -> io::Result<IodineObject> {
    let name = read_string(file)?;
    file.read_u8()?;

    println!("Encountered module: {}", name);

    Ok(module!(name, box read_code_object(file)?))
}

fn read_code_object(file: &mut File) -> io::Result<IodineObject> {
    let mut code = code_object!();

    let instruction_count = file.read_u32::<LittleEndian>()?;

    println!("Module instruction count: {}", instruction_count);

    for _ in 0..instruction_count {
        code.push_instruction(read_instruction(file)?);
    }

    Ok(code)
}

fn read_constant(file: &mut File) -> io::Result<Arc<IodineObject>> {
    let iodine_type = DataType::from(file.read_u8()?);
    println!("Encountered type: {:?}", iodine_type);

    match iodine_type {
        DataType::StringObject => {
            return Ok(Arc::new(string!(read_string(file)?)));
        }
        DataType::NameObject => {
            return Ok(Arc::new(name!(read_string(file)?)));
        }
        DataType::NullObject => {
            return Ok(IodineNull.clone());
        }
        _ => unimplemented!(),
    }
}

fn read_instruction(file: &mut File) -> io::Result<Instruction> {
    println!();
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
        object: argument_obj,
    })
}

#[derive(Debug)]
enum DataType {
    CodeObject,
    NameObject,
    StringObject,
    IntObject,
    FloatObject,
    BoolObject,
    NullObject,
    BigIntObject,
}

impl From<u8> for DataType {
    fn from(iodine_type: u8) -> DataType {
        match iodine_type {
            0x00 => DataType::CodeObject,
            0x01 => DataType::NameObject,
            0x02 => DataType::StringObject,
            0x03 => DataType::IntObject,
            0x04 => DataType::FloatObject,
            0x05 => DataType::BoolObject,
            0x06 => DataType::NullObject,
            0x07 => DataType::BigIntObject,
            _ => unreachable!(),
        }
    }
}
