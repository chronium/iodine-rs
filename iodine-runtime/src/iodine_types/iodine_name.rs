use crate::{AttributeDictionary, IodineObject};

pub fn create(value: String) -> IodineObject {
    IodineObject::IodineName {
        attribs: AttributeDictionary::new(),
        value,
    }
}
