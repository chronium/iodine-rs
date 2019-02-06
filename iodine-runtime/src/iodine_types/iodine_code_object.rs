use crate::{AttributeDictionary, IodineObject};

pub fn create() -> IodineObject {
    IodineObject::CodeObject {
        attribs: AttributeDictionary::new(),
        instructions: Vec::new(),
    }
}
