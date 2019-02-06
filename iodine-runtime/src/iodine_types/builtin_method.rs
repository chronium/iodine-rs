use crate::{AttributeDictionary, BuiltinMethodDef, IodineObject};

use std::sync::Arc;

pub fn create(callback: BuiltinMethodDef) -> Arc<IodineObject> {
    Arc::new(IodineObject::BuiltinMethodCallback {
        attribs: AttributeDictionary::new(),
        callback,
    })
}
