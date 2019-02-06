use crate::{opcode::Opcode, stack_frame::StackFrame, IodineNull, IodineObject};

use std::sync::Arc;

pub struct VirtualMachine {
    pub frames: Vec<StackFrame>,
    pub stack_size: usize,
    pub frame_count: usize,
}

impl VirtualMachine {
    pub fn new_frame(&mut self, frame: StackFrame) {
        self.frame_count += 1;
        self.stack_size += 1;

        self.frames.push(frame);
    }

    pub fn end_frame(&mut self) -> Option<StackFrame> {
        self.frame_count -= 1;
        self.stack_size -= 1;

        self.frames.pop()
    }

    pub fn eval_code(&mut self, code: &Box<IodineObject>) -> Arc<IodineObject> {
        match code {
            &box IodineObject::CodeObject {
                attribs: _,
                instructions: _,
            } => {}
            _ => panic!("Tried to evaluate non-code object"),
        }

        let ins_count = code.get_instructions().len();
        let mut pc = self.frames.last().unwrap().instruction_pointer as usize;

        while pc < ins_count {
            let instruction = &code.get_instructions()[pc];
            pc += 1;

            match instruction.opcode {
                Opcode::LoadConst => {
                    println!("Load constant: {:?}", instruction.object);

                    self.frames
                        .last_mut()
                        .unwrap()
                        .push(Some(instruction.object.clone()));
                }
                _ => panic!("Unimplemented instruction: {:?}", instruction.opcode),
            }
        }

        IodineNull.clone()
    }
}
