use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::{Arc, Mutex};

struct MemEntry {
    capacity: usize,
    data: Vec<u8>,
}

impl MemEntry {
    fn new(size: usize) -> Self {
        MemEntry {
            capacity: size,
            data: vec![0; size],
        }
    }

    fn alloc(&mut self, size: usize) {
        if self.capacity < size {
            self.data.resize(size, 0);
            self.capacity = size;
        }
    }

    fn free(&mut self) {
        self.data.clear();
        self.capacity = 0;
    }
}

pub struct DataMem {
    pub seq: u64,
    pub size: usize,
    pub capacity: usize,
    pub data: Vec<u8>,
}

pub struct DataMemPool {
    free_seqs: BTreeMap<usize, HashSet<u64>>,
    mem_entries: HashMap<u64, MemEntry>,
}

impl DataMemPool {
    pub fn new() -> Self {
        DataMemPool {
            free_seqs: BTreeMap::new(),
            mem_entries: HashMap::new(),
        }
    }

    pub fn instance() -> Arc<Mutex<DataMemPool>> {
        static mut INSTANCE: Option<Arc<Mutex<DataMemPool>>> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        unsafe {
            INIT.call_once(|| {
                let instance = DataMemPool::new();
                INSTANCE = Some(Arc::new(Mutex::new(instance)));
            });

            INSTANCE.clone().unwrap()
        }
    }

    pub fn alloc(&mut self, size: usize) -> DataMem {
        let seq = self.get_next_seq(size);
        let entry = self.mem_entries.get_mut(&seq).unwrap();
        entry.alloc(size);

        self.free_seqs.get_mut(&entry.capacity).unwrap().remove(&seq);

        DataMem {
            seq,
            size,
            capacity: entry.capacity,
            data: entry.data.clone(),
        }
    }

    pub fn free(&mut self, data_mem: DataMem) {
        self.free_seqs.entry(data_mem.capacity).or_insert_with(HashSet::new).insert(data_mem.seq);
    }

    pub fn total_size(&self) -> usize {
        self.mem_entries.values().map(|entry| entry.capacity).sum()
    }

    fn get_next_seq(&mut self, size: usize) -> u64 {
        let available_seq = self.free_seqs.range(size..).next().and_then(|(_, seqs)| seqs.iter().next().cloned());
        if let Some(seq) = available_seq {
            seq
        } else {
            self.expand_mem_entries(size)
        }
    }

    fn expand_mem_entries(&mut self, size: usize) -> u64 {
        let seq = self.mem_entries.len() as u64;
        let entry = MemEntry::new(size);

        self.free_seqs.entry(size).or_insert_with(HashSet::new).insert(seq);
        self.mem_entries.insert(seq, entry);

        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_and_free() {
        let mut pool = DataMemPool::new();
        let mem = pool.alloc(1024);

        assert_eq!(mem.size, 1024);
        assert_eq!(mem.capacity, 1024);
        assert!(!mem.data.is_empty());

        let total_size_before_free = pool.total_size();
        pool.free(mem);

        let total_size_after_free = pool.total_size();
        assert_eq!(total_size_before_free, total_size_after_free);
    }
}

fn main() {
    // The main function is left intentionally blank because the logic is demonstrated
    // in the test case. In a real-world scenario, you would use the pool like this:
    // let pool = DataMemPool::instance();
    // let mut pool_guard = pool.lock().unwrap();
    // let mem = pool_guard.alloc(1024);
    // // Use the memory...
    // pool_guard.free(mem);
}