#![feature(box_patterns)]

use std::{collections::HashMap, fmt, rc::Rc};

use lazy_static::*;

pub mod iodine_context;
pub mod opcode;
pub mod stack_frame;
pub mod virtual_machine;

use crate::{opcode::Opcode, stack_frame::StackFrame, virtual_machine::VirtualMachine};

fn create_type(name: &str) -> (String, AttributeDictionary) {
    let mut attribs = AttributeDictionary::new();

    attribs.insert(
        "__name__".to_string(),
        IodineObject::IodineString {
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
    pub object: Rc<IodineObject>,
}

pub enum IodineObject {
    IodineString {
        value: String,
    },
    CodeObject {
        instructions: Vec<Instruction>,
    },
    IodineObject,
    IodineModule {
        name: String,
        code: Box<IodineObject>,
    },
    IodineNull,
    IodineName {
        value: String,
    },
}

unsafe impl Sync for IodineObject {}
unsafe impl Send for IodineObject {}

pub type AttributeDictionary = HashMap<String, IodineObject>;
pub type IodineTypesDict = HashMap<String, AttributeDictionary>;

impl IodineObject {
    pub fn push_instruction(&mut self, instruction: Instruction) {
        match self {
            IodineObject::CodeObject { instructions } => instructions.push(instruction),
            _ => panic!("Cannot push instruction on non CodeObject"),
        }
    }

    pub fn get_instructions(&self) -> &Vec<Instruction> {
        match self {
            IodineObject::CodeObject { instructions } => instructions,
            _ => panic!("Cannot get instruction from non CodeObject"),
        }
    }

    fn get_type(&self) -> String {
        match self {
            IodineObject::IodineString { value: _ } => "Str".to_string(),
            IodineObject::CodeObject { instructions: _ } => "Code".to_string(),
            IodineObject::IodineObject => "Object".to_string(),
            IodineObject::IodineModule { name: _, code: _ } => "Module".to_string(),
            IodineObject::IodineNull => "Null".to_string(),
            IodineObject::IodineName { value: _ } => "Name".to_string(),
        }
    }

    fn get_base(&self) -> String {
        match self {
            _ => "Object".to_string(),
        }
    }

    pub fn invoke(&self, vm: &mut VirtualMachine, arguments: Vec<IodineObject>) -> IodineObject {
        match self {
            IodineObject::IodineModule { name: _, code } => {
                vm.new_frame(StackFrame {
                    stack: Vec::new(),
                    locals: AttributeDictionary::new(),
                    instruction_pointer: 0usize,
                });

                let ret = vm.eval_code(code);

                vm.end_frame();

                ret
            }
            _ => unimplemented!(),
        }
    }
}

impl fmt::Debug for IodineObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IodineObject::IodineString { value } => write!(f, "{}", value),
            IodineObject::CodeObject { instructions: _ } => write!(f, "Code"),
            IodineObject::IodineObject => write!(f, "Object"),
            IodineObject::IodineModule { name: _, code: _ } => write!(f, "Module"),
            IodineObject::IodineNull => write!(f, "Null"),
            IodineObject::IodineName { value } => write!(f, "{}", value),
        }
    }
}
