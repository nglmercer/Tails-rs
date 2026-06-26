use std::collections::HashSet;

pub struct GarbageCollector {
    allocated: usize,
    threshold: usize,
    marked: HashSet<usize>,
}

impl GarbageCollector {
    pub fn new() -> Self {
        Self {
            allocated: 0,
            threshold: 1024 * 1024, // 1MB
            marked: HashSet::new(),
        }
    }
    
    pub fn should_collect(&self) -> bool {
        self.allocated >= self.threshold
    }
    
    pub fn collect(&mut self) {
        self.marked.clear();
        // TODO: Implement mark-and-sweep
        self.allocated = 0;
        self.threshold = (self.allocated * 2).max(1024 * 1024);
    }
    
    pub fn allocate(&mut self, size: usize) {
        self.allocated += size;
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}
