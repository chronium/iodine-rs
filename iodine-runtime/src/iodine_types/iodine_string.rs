use crate::{AttributeDictionary, IodineObject};

use std::sync::Arc;

pub fn create(value: String) -> Arc<IodineObject> {
    Arc::new(IodineObject::IodineString {
        attribs: AttributeDictionary::new(),
        value,
    })
}
