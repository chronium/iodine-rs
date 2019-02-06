use crate::{AttributeDictionary, IodineObject};

pub fn create(value: String) -> IodineObject {
    IodineObject::IodineString {
        attribs: AttributeDictionary::new(),
        value,
    }
}
