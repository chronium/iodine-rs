use std::collections::HashMap;

use lazy_static::*;

mod opcode;
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

lazy_static! {
    static ref IodineTypes: IodineTypesDict = {
        let mut types = IodineTypesDict::new();

        let obj_type = create_type("Object");
        types.insert(obj_type.0, obj_type.1);

        let str_type = create_type("Str");
        types.insert(str_type.0, str_type.1);

        let code_type = create_type("Code");
        types.insert(code_type.0, code_type.1);

        let module_type = create_type("Module");
        types.insert(module_type.0, module_type.1);

        types
    };
}

pub struct Instruction {
    opcode: Opcode,
    argument: i32,
    object: IodineObjects,
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
        }
    }

    fn get_base(&self) -> String {
        match self {
            _ => "Object".to_string(),
        }
    }
}
