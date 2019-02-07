use crate::{
    iodine_context::IodineContext, opcode::Opcode, stack_frame::StackFrame, AttributeDictionary,
    IodineNull, IodineObject,
};

use std::sync::{Arc, Mutex};

pub struct VirtualMachine {
    pub frames: Vec<StackFrame>,
    pub stack_size: usize,
    pub frame_count: usize,
    pub context: IodineContext,
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

    pub fn eval_code(&mut self, code: &mut Mutex<Box<IodineObject>>) -> Arc<IodineObject> {
        match *code.lock().unwrap() {
            box IodineObject::CodeObject {
                ..
            } => {}
            _ => panic!("Tried to evaluate non-code object"),
        }

        let ins_count = code.lock().unwrap().get_instructions().len();
        let mut pc = self.frames.last().unwrap().instruction_pointer as usize;

        while pc < ins_count {
            let instruction = &mut code.get_mut().unwrap().get_instructions_mut()[pc];
            pc += 1;

            match instruction.opcode {
                Opcode::LoadConst => {
                    println!("Load constant: {:?}", instruction.object);

                    self.frames
                        .last_mut()
                        .unwrap()
                        .push(Some(instruction.object.clone()));
                }
                Opcode::LoadGlobal => {
                    println!("Load global: {:?}", instruction.object);

                    let global = self.context.globals.get(&instruction.get_string());

                    if global.is_none() {
                        panic!("Cannot find global: {:?}", instruction.object);
                    }

                    self.frames
                        .last_mut()
                        .unwrap()
                        .push(Some(global.unwrap().clone()));
                }
                Opcode::Invoke => {
                    //let frame = &mut self.frames.last_mut().unwrap();
                    let target = &mut VirtualMachine::pop(self);
                    let mut args: Vec<Arc<IodineObject>> =
                        Vec::with_capacity(instruction.argument as usize);
                    for _ in 0..args.capacity() {
                        args.push(VirtualMachine::pop(self));
                    }

                    self.invoke(target, args);
                }
                _ => panic!("Unimplemented instruction: {:?}", instruction.opcode),
            }
        }

        IodineNull.clone()
    }

    fn pop(&mut self) -> Arc<IodineObject> {
        self.frames.last_mut().unwrap().pop()
    }

    pub fn invoke(&mut self, obj: &mut Arc<IodineObject>, arguments: Vec<Arc<IodineObject>>) {
        match Arc::get_mut(obj).unwrap() {
            IodineObject::IodineModule {
                code,
                ..
            } => {
                self.new_frame(StackFrame {
                    stack: Vec::new(),
                    locals: AttributeDictionary::new(),
                    instruction_pointer: 0usize,
                });

                let ret = self.eval_code(code);

                self.end_frame();

                self.frames.last_mut().unwrap().push(Some(ret));
                return;
            }
            IodineObject::BuiltinMethodCallback {
                callback,
                ..
            } => {
                let ret = callback(self, obj, arguments);

                if ret.is_some() {
                    self.frames.last_mut().unwrap().push(ret)
                }
                return;
            }
            _ => panic!("wtf"),
        }
        #[allow(unreachable_code)] {
            unimplemented!()
        }
    }
}
