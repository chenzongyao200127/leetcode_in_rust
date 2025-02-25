// 2502_设计内存分配器
// https://leetcode.cn/problems/design-memory-allocator/description/

struct Allocator {
    memo: Vec<i32>,
    // store 
    map: HashMap<i32, String>,
}

impl Allocator {
    /// Creates a new allocator with `n` units of memory.
    fn new(n: i32) -> Self {
        Allocator {
            memo: vec![-1; n as usize],
        }
    }

    /// Allocates a contiguous block of `size` units and assigns it to `m_id`.
    /// Returns the starting index of the block if successful, or `-1` if allocation fails.
    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let len = self.memo.len();

        let mut i = 0;
        while i + size <= len {
            // Check if there's a contiguous block of free memory of the required size
            if self.memo[i..i + size].iter().all(|&x| x == -1) {
                // Allocate the block by assigning `m_id` to all positions
                for k in i..i + size {
                    self.memo[k] = m_id;
                }
                return i as i32; // Return the starting index
            }
            i += 1;
        }
        -1 // Allocation failed
    }

    /// Frees all memory blocks assigned to `m_id` and returns the number of units freed.
    fn free_memory(&mut self, m_id: i32) -> i32 {
        let mut freed_count = 0;

        for val in &mut self.memo {
            if *val == m_id {
                *val = -1; // Mark memory as free
                freed_count += 1;
            }
        }

        freed_count
    }
}


/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free_memory(mID);
 */