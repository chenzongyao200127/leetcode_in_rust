use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Represents a single memory entry.
struct MemEntry {
    mem: Vec<u8>,
}

impl MemEntry {
    /// Allocates memory with the specified size.
    /// If the current capacity is insufficient, the memory is resized.
    fn alloc(&mut self, size: usize) {
        if self.mem.capacity() < size {
            self.mem.resize(size, 0);
        }
    }

    /// Releases the memory.
    fn free(&mut self) {
        self.mem.clear();
    }

    /// Returns a raw pointer to the memory.
    fn data(&self) -> *const u8 {
        self.mem.as_ptr()
    }
}

/// Represents the memory allocated to a user.
struct DataMem {
    seq: u64,
    size: usize,
    data: *const u8,
}

/// The main structure for the memory pool.
struct DataMemPool {
    free_seqs: HashMap<usize, HashSet<u64>>,
    mem_entries: HashMap<u64, MemEntry>,
    seq_counter: u64,
    total_size: u64,
}

impl DataMemPool {
    /// Returns a singleton instance of `DataMemPool`.
    fn instance() -> Arc<Mutex<Self>> {
        static INSTANCE: Lazy<Arc<Mutex<DataMemPool>>> = Lazy::new(|| {
            Arc::new(Mutex::new(DataMemPool {
                free_seqs: HashMap::new(),
                mem_entries: HashMap::new(),
                seq_counter: 0,
                total_size: 0,
            }))
        });
        INSTANCE.clone()
    }

    /// Allocates a `DataMem`. If the pool does not have enough memory, it is expanded.
    fn alloc(&mut self, size: usize) -> DataMem {
        let seq = self.seq_counter;
        self.seq_counter += 1;

        let mem_entry = self
            .mem_entries
            .entry(seq)
            .or_insert_with(|| MemEntry { mem: Vec::new() });

        mem_entry.alloc(size);

        self.total_size += size as u64;

        DataMem {
            seq,
            size,
            data: mem_entry.data(),
        }
    }

    /// Returns a `DataMem`, freeing the corresponding memory.
    fn free(&mut self, data_mem: DataMem) {
        if let Some(mem_entry) = self.mem_entries.remove(&data_mem.seq) {
            self.total_size -= data_mem.size as u64;
            // mem_entry is dropped here, and its memory is released.
        }
    }

    /// Returns the current total size of the memory pool in bytes.
    fn total_size(&self) -> u64 {
        self.total_size
    }

    /// Releases all resources of the memory pool.
    fn destroy(&mut self) {
        self.mem_entries.clear();
        self.total_size = 0;
        self.seq_counter = 0;
        self.free_seqs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_free() {
        let instance = DataMemPool::instance();
        let mut pool = instance.lock().unwrap();
        let data_mem = pool.alloc(1024);

        assert_eq!(data_mem.size, 1024);
        assert_eq!(pool.total_size(), 1024);

        pool.free(data_mem);
        assert_eq!(pool.total_size(), 0);
    }

    #[test]
    fn test_destroy_resets_seq_counter() {
        let instance = DataMemPool::instance();
        let mut pool = instance.lock().unwrap();
        pool.alloc(1024);
        pool.destroy();

        let data_mem = pool.alloc(512);
        // This should be `0`, not `1`, because `seq_counter` starts from 0 after `destroy`.
        assert_eq!(data_mem.seq, 0);
        assert_eq!(pool.total_size(), 512);
    }

    #[test]
    fn test_destroy() {
        let instance = DataMemPool::instance();
        let mut pool = instance.lock().unwrap();

        pool.alloc(1024); // Suppose this allocates a block and increases total size by 1024.
        pool.alloc(2048); // Another block, total size increases by 2048.

        // The total size should now be 1024 + 2048 = 3072.
        assert_eq!(pool.total_size(), 3072);

        pool.destroy();

        // After destroy, there should be no allocations, hence total size should be 0.
        assert_eq!(pool.total_size(), 0);
    }
}

fn main() {
    let pool = DataMemPool::instance();

    {
        let mut pool_guard = pool.lock().unwrap();
        let data_mem = pool_guard.alloc(1024);
        println!("Allocated {} bytes of memory.", data_mem.size);

        // Skipping operations on data_mem.data pointer as this is just an example.

        pool_guard.free(data_mem);
        println!(
            "Freed memory. Total size of pool is now {} bytes.",
            pool_guard.total_size()
        );
    }

    println!("Tests completed.");
}
