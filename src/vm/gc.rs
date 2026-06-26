use crate::objects::Value;
use crate::vm::interpreter::HeapValue;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct HeapSlot {
    pub value: HeapValue,
    pub marked: bool,
    pub free: bool,
}

impl HeapSlot {
    pub fn new(value: HeapValue) -> Self {
        Self {
            value,
            marked: false,
            free: false,
        }
    }
}

pub struct GarbageCollector {
    pub heap: Vec<HeapSlot>,
    free_list: VecDeque<usize>,
    allocation_count: usize,
    pub threshold: usize,
    pub collections_performed: usize,
    pub bytes_freed: usize,
}

impl GarbageCollector {
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            free_list: VecDeque::new(),
            allocation_count: 0,
            threshold: 1024, // Start small for testing; 1KB triggers GC
            collections_performed: 0,
            bytes_freed: 0,
        }
    }

    pub fn should_collect(&self) -> bool {
        self.allocation_count >= self.threshold
    }

    pub fn allocate(&mut self, value: HeapValue) -> usize {
        self.allocation_count += 1;

        if let Some(idx) = self.free_list.pop_front() {
            self.heap[idx] = HeapSlot::new(value);
            idx
        } else {
            let idx = self.heap.len();
            self.heap.push(HeapSlot::new(value));
            idx
        }
    }

    pub fn get(&self, idx: usize) -> Option<&HeapValue> {
        self.heap.get(idx).filter(|s| !s.free).map(|s| &s.value)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut HeapValue> {
        self.heap.get_mut(idx).filter(|s| !s.free).map(|s| &mut s.value)
    }

    pub fn mark(&mut self, idx: usize) {
        if let Some(slot) = self.heap.get_mut(idx) {
            if !slot.free {
                slot.marked = true;
            }
        }
    }

    pub fn is_marked(&self, idx: usize) -> bool {
        self.heap.get(idx).map_or(false, |s| s.marked && !s.free)
    }

    pub fn reset_marks(&mut self) {
        for slot in &mut self.heap {
            slot.marked = false;
        }
    }

    pub fn sweep(&mut self) -> usize {
        let mut freed = 0;
        for (i, slot) in self.heap.iter_mut().enumerate() {
            if !slot.free && !slot.marked {
                slot.free = true;
                self.free_list.push_back(i);
                freed += 1;
            }
        }
        self.allocation_count = self.heap.iter().filter(|s| !s.free).count();
        self.bytes_freed += freed;
        self.collections_performed += 1;
        freed
    }

    pub fn mark_value(&mut self, value: &Value) {
        match value {
            Value::Object(idx) | Value::Array(idx) | Value::Function(idx) | Value::Promise(idx) | Value::Proxy(idx) => {
                self.mark(*idx);
            }
            _ => {}
        }
    }

    pub fn collect(&mut self) -> usize {
        self.reset_marks();
        // Mark phase happens externally (roots identification)
        // This is called after marking is complete
        let freed = self.sweep();
        freed
    }

    pub fn live_count(&self) -> usize {
        self.heap.iter().filter(|s| !s.free).count()
    }

    pub fn total_count(&self) -> usize {
        self.heap.len()
    }

    pub fn free_count(&self) -> usize {
        self.free_list.len()
    }

    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::interpreter::{JsObject, JsArray, JsFunction};
    use std::collections::HashMap;

    #[test]
    fn test_gc_new() {
        let gc = GarbageCollector::new();
        assert_eq!(gc.live_count(), 0);
        assert_eq!(gc.total_count(), 0);
        assert_eq!(gc.collections_performed, 0);
    }

    #[test]
    fn test_gc_allocate() {
        let mut gc = GarbageCollector::new();
        let idx = gc.allocate(HeapValue::Object(JsObject::new()));
        assert_eq!(idx, 0);
        assert_eq!(gc.live_count(), 1);
    }

    #[test]
    fn test_gc_mark_and_sweep_removes_unmarked() {
        let mut gc = GarbageCollector::new();
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Object(JsObject::new()));

        gc.reset_marks();
        gc.mark(0); // Only mark first

        let freed = gc.sweep();
        assert_eq!(freed, 1);
        assert_eq!(gc.live_count(), 1);
        assert!(gc.is_marked(0));
    }

    #[test]
    fn test_gc_reuses_free_slots() {
        let mut gc = GarbageCollector::new();
        let _idx0 = gc.allocate(HeapValue::Object(JsObject::new()));
        let _idx1 = gc.allocate(HeapValue::Object(JsObject::new()));

        // Mark only first, sweep second
        gc.reset_marks();
        gc.mark(0);
        gc.sweep();

        // Allocate new - should reuse freed slot 1
        let idx2 = gc.allocate(HeapValue::Object(JsObject::new()));
        assert_eq!(idx2, 1); // Reused slot
        assert_eq!(gc.live_count(), 2);
    }

    #[test]
    fn test_gc_should_collect() {
        let mut gc = GarbageCollector::new();
        gc.set_threshold(3);
        assert!(!gc.should_collect());

        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Object(JsObject::new()));
        assert!(gc.should_collect());
    }

    #[test]
    fn test_gc_mark_value() {
        let mut gc = GarbageCollector::new();
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Array(JsArray { elements: vec![] }));

        gc.reset_marks();
        gc.mark_value(&Value::Object(0));
        assert!(gc.is_marked(0));
        assert!(!gc.is_marked(1));
    }

    #[test]
    fn test_gc_collect_resets_marks() {
        let mut gc = GarbageCollector::new();
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Object(JsObject::new()));

        gc.mark(0);
        gc.mark(1);
        gc.collect();

        assert!(!gc.is_marked(0));
        assert!(!gc.is_marked(1));
        assert_eq!(gc.collections_performed, 1);
    }

    #[test]
    fn test_gc_multiple_collections() {
        let mut gc = GarbageCollector::new();
        gc.set_threshold(2);

        // First round
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.mark(0);
        gc.collect();
        assert_eq!(gc.collections_performed, 1);

        // Second round
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.allocate(HeapValue::Object(JsObject::new()));
        gc.mark(2);
        gc.collect();
        assert_eq!(gc.collections_performed, 2);
        assert_eq!(gc.live_count(), 1);
    }
}
