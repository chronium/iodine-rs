use crate::{AttributeDictionary, IodineObject};

use std::rc::Rc;

pub struct StackFrame {
    pub stack: Vec<Rc<IodineObject>>,
    pub locals: AttributeDictionary,
    pub instruction_pointer: usize,
}

impl StackFrame {
    pub fn push(&mut self, obj: Option<Rc<IodineObject>>) {
        let push_obj = if let Some(obj) = obj {
            obj
        } else {
            Rc::new(IodineObject::IodineNull)
        };

        self.stack.push(push_obj.clone());
    }
}
