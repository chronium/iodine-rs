use crate::{AttributeDictionary, IodineNull, IodineObject};

use std::sync::Arc;

pub struct StackFrame {
    pub stack: Vec<Arc<IodineObject>>,
    pub locals: AttributeDictionary,
    pub instruction_pointer: usize,
}

impl StackFrame {
    pub fn push(&mut self, obj: Option<Arc<IodineObject>>) {
        let push_obj = if let Some(obj) = obj {
            obj
        } else {
            IodineNull.clone()
        };

        self.stack.push(push_obj.clone());
    }
}
