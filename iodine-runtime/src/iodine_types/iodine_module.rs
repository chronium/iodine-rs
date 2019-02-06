use crate::{AttributeDictionary, IodineObject};

use std::sync::Mutex;

pub fn create(name: String, code: Box<IodineObject>) -> IodineObject {
    IodineObject::IodineModule {
        attribs: AttributeDictionary::new(),
        name,
        code: Mutex::new(code),
    }
}
