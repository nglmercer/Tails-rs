use std::collections::VecDeque;
use crate::objects::Value;

pub struct AsyncRuntime {
    microtask_queue: VecDeque<Value>,
    macrotask_queue: VecDeque<Value>,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        Self {
            microtask_queue: VecDeque::new(),
            macrotask_queue: VecDeque::new(),
        }
    }
    
    pub fn enqueue_microtask(&mut self, task: Value) {
        self.microtask_queue.push_back(task);
    }
    
    pub fn enqueue_macrotask(&mut self, task: Value) {
        self.macrotask_queue.push_back(task);
    }
    
    pub fn run_microtasks(&mut self) {
        while let Some(_task) = self.microtask_queue.pop_front() {
            // TODO: Execute microtask
        }
    }
    
    pub fn run_macrotasks(&mut self) {
        while let Some(_task) = self.macrotask_queue.pop_front() {
            // TODO: Execute macrotask
        }
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new()
    }
}
