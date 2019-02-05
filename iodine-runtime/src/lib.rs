use std::{collections::HashMap, fmt};

use lazy_static::*;

pub mod opcode;
use crate::opcode::Opcode;

fn create_type(name: &str) -> (String, AttributeDictionary) {
    let mut attribs = AttributeDictionary::new();

    attribs.insert(
        "__name__".to_string(),
        IodineObjects::IodineString {
            value: name.to_string(),
        },
    );

    (name.to_string(), attribs)
}

macro_rules! create_iodine_type {
    ($i:ident, $types:ident) => {
        let i_type = create_type(stringify!($i));
        $types.insert(i_type.0, i_type.1);
    };
}

lazy_static! {
    static ref IodineTypes: IodineTypesDict = {
        let mut types = IodineTypesDict::new();

        create_iodine_type!(Object, types);
        create_iodine_type!(Str, types);
        create_iodine_type!(Code, types);
        create_iodine_type!(Module, types);
        create_iodine_type!(Null, types);
        create_iodine_type!(Name, types);

        types
    };
}

pub struct Instruction {
    pub opcode: Opcode,
    pub argument: i32,
    pub object: IodineObjects,
}

pub trait IodineObject {
    fn get_type(&self) -> String;

    fn get_base(&self) -> String;
}

pub enum IodineObjects {
    IodineString {
        value: String,
    },
    CodeObject {
        instructions: Vec<Instruction>,
    },
    IodineObject,
    IodineModule {
        name: String,
        code: Box<IodineObjects>,
    },
    IodineNull,
    IodineName {
        value: String,
    },
}

unsafe impl Sync for IodineObjects {}
unsafe impl Send for IodineObjects {}

pub type AttributeDictionary = HashMap<String, IodineObjects>;
pub type IodineTypesDict = HashMap<String, AttributeDictionary>;

impl IodineObject for IodineObjects {
    fn get_type(&self) -> String {
        match self {
            IodineObjects::IodineString { value: _ } => "Str".to_string(),
            IodineObjects::CodeObject { instructions: _ } => "Code".to_string(),
            IodineObjects::IodineObject => "Object".to_string(),
            IodineObjects::IodineModule { name: _, code: _ } => "Module".to_string(),
            IodineObjects::IodineNull => "Null".to_string(),
            IodineObjects::IodineName { value: _ } => "Name".to_string(),
        }
    }

    fn get_base(&self) -> String {
        match self {
            _ => "Object".to_string(),
        }
    }
}

impl IodineObjects {
    pub fn push_instruction(&mut self, instruction: Instruction) {
        match self {
            IodineObjects::CodeObject { instructions } => instructions.push(instruction),
            _ => panic!("Cannot push instruction on non CodeObject"),
        }
    }
}

impl fmt::Debug for IodineObjects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IodineObjects::IodineString { value } => write!(f, "{}", value),
            IodineObjects::CodeObject { instructions: _ } => write!(f, "Code"),
            IodineObjects::IodineObject => write!(f, "Object"),
            IodineObjects::IodineModule { name: _, code: _ } => write!(f, "Module"),
            IodineObjects::IodineNull => write!(f, "Null"),
            IodineObjects::IodineName { value } => write!(f, "{}", value),
        }
    }
}
