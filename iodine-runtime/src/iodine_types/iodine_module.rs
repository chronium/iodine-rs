use crate::{AttributeDictionary, IodineObject};

pub fn create(name: String, code: Box<IodineObject>) -> IodineObject {
    IodineObject::IodineModule {
        attribs: AttributeDictionary::new(),
        name,
        code
    }
}
