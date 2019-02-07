#![feature(box_patterns)]

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use lazy_static::*;

pub mod iodine_context;
pub mod iodine_types;
pub mod opcode;
pub mod stack_frame;
pub mod virtual_machine;

use crate::{opcode::Opcode, stack_frame::StackFrame, virtual_machine::VirtualMachine};

pub type BuiltinMethodDef =
    fn(&VirtualMachine, &IodineObject, Vec<Arc<IodineObject>>) -> Option<Arc<IodineObject>>;

fn create_type(name: &str) -> (String, AttributeDictionary) {
    let mut attribs = AttributeDictionary::new();

    attribs.insert("__name__".to_string(), string!(name.to_string()));

    (name.to_string(), attribs)
}

lazy_static! {
    static ref IodineTypes: IodineTypesDict = #[allow(non_snake_case)]
    {
        let mut types = IodineTypesDict::new();

        macro_rules! create_iodine_type {
            ($i:ident) => {
                let $i = create_type(stringify!($i));
                types.insert($i.0, $i.1);
            };
        }

        create_iodine_type!(Object);
        create_iodine_type!(Str);
        create_iodine_type!(Code);
        create_iodine_type!(Module);
        create_iodine_type!(Null);
        create_iodine_type!(Name);

        types
    };
    pub static ref IodineNull: Arc<IodineObject> = Arc::new(IodineObject::IodineNull {
        attribs: AttributeDictionary::new(),
    });
}

pub struct Instruction {
    pub opcode: Opcode,
    pub argument: i32,
    pub object: Arc<IodineObject>,
}

impl Instruction {
    fn get_string(&mut self) -> String {
        if let Some(IodineObject::IodineName { value, .. }) = Arc::get_mut(&mut self.object)
        {
            value.clone()
        } else {
            panic!("Tried to get string from non IodineName")
        }
    }
}

pub enum IodineObject {
    IodineString {
        attribs: AttributeDictionary,
        value: String,
    },
    CodeObject {
        attribs: AttributeDictionary,
        instructions: Vec<Instruction>,
    },
    IodineObject {
        attribs: AttributeDictionary,
    },
    IodineModule {
        attribs: AttributeDictionary,
        name: String,
        code: Mutex<Box<IodineObject>>,
    },
    IodineNull {
        attribs: AttributeDictionary,
    },
    IodineName {
        attribs: AttributeDictionary,
        value: String,
    },
    BuiltinMethodCallback {
        attribs: AttributeDictionary,
        callback: BuiltinMethodDef,
    },
}

unsafe impl Sync for IodineObject {}
unsafe impl Send for IodineObject {}

pub type AttributeDictionary = HashMap<String, Arc<IodineObject>>;
pub type IodineTypesDict = HashMap<String, AttributeDictionary>;

impl IodineObject {
    pub fn push_instruction(&mut self, instruction: Instruction) {
        match self {
            IodineObject::CodeObject {
                instructions,
                ..
            } => instructions.push(instruction),
            _ => panic!("Cannot push instruction on non CodeObject"),
        }
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        match self {
            IodineObject::CodeObject {
                instructions,
                ..
            } => instructions,
            _ => panic!("Cannot get instruction from non CodeObject"),
        }
    }

    pub fn get_instructions_mut(&mut self) -> &mut Vec<Instruction> {
        match self {
            IodineObject::CodeObject {
                instructions,
                ..
            } => instructions,
            _ => panic!("Cannot get instruction from non CodeObject"),
        }
    }

    fn get_type(&self) -> String {
        match self {
            IodineObject::IodineString {
                ..
            } => "Str".to_string(),
            IodineObject::CodeObject {
                ..
            } => "Code".to_string(),
            IodineObject::IodineObject { .. } => "Object".to_string(),
            IodineObject::IodineModule {
                ..
            } => "Module".to_string(),
            IodineObject::IodineNull { .. } => "Null".to_string(),
            IodineObject::IodineName {
                ..
            } => "Name".to_string(),
            IodineObject::BuiltinMethodCallback {
                ..
            } => "Builtin".to_string(),
        }
    }

    fn get_base(&self) -> String {
        match self {
            _ => "Object".to_string(),
        }
    }

    pub fn has_attribute(&self) -> Option<IodineObject> {
        None
    }
}

impl fmt::Debug for IodineObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IodineObject::IodineString { value, .. } => write!(f, "{}", value),
            IodineObject::CodeObject {
                ..
            } => write!(f, "Code"),
            IodineObject::IodineObject { .. } => write!(f, "Object"),
            IodineObject::IodineModule {
                ..
            } => write!(f, "Module"),
            IodineObject::IodineNull { .. } => write!(f, "Null"),
            IodineObject::IodineName { value, .. } => write!(f, "{}", value),
            IodineObject::BuiltinMethodCallback {
                ..
            } => write!(f, "Builtin"),
        }
    }
}
