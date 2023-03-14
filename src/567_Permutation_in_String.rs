// 567_Permutation_in_String
// https://leetcode.cn/problems/permutation-in-string/


impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false;
        }

        let mut p_count = [0; 26];
        for &i in s1.as_bytes() {
            p_count[(i - b'a') as usize] += 1;
        }

        let mut s_count = [0; 26];
        let bytes = s2.as_bytes();
        for i in 0..s2.len() {
            if i < s1.len() {
                s_count[(bytes[i] - b'a') as usize] += 1;
            } else {
                s_count[(bytes[i] - b'a') as usize] += 1;
                s_count[(bytes[i - s1.len()] - b'a') as usize] -= 1;
            }
            
            if s_count == p_count {
                return true;
            }
        }

        false
    }
}





fn u8_to_index(c: u8) -> usize {
    (c - ('a' as u8)) as usize
}

#[derive(Default)]
struct CharMap {
    storage: [usize; 26],
    len: usize,
}

impl CharMap {
    fn insert(&mut self, c: u8) -> usize {
        let index = u8_to_index(c);
        let ptr = &mut self.storage[index];
        if *ptr == 0 {
            self.len += 1;
        }

        *ptr += 1;
        *ptr
    }

    fn remove(&mut self, c: u8) -> usize {
        let index = u8_to_index(c);
        let ptr = &mut self.storage[index];
        if *ptr == 1 {
            self.len -= 1;
        }

        *ptr -= 1;
        *ptr
    }

    fn counts(&self, c: u8) -> usize {
        let index = u8_to_index(c);
        self.storage[index]
    }
}

struct Windows {
    pattern: CharMap,
    inner: CharMap,
    valid: usize,
}

impl Windows {
    fn insert(&mut self, c: u8) {
        let count = self.inner.insert(c);
        if count == self.pattern.counts(c) {
            self.valid += 1;
        }
    }

    fn remove(&mut self, c: u8) {
        let count = self.inner.remove(c);
        if count < self.pattern.counts(c) {
            self.valid -= 1;
        }
    }

    fn new(pattern: &str) -> Self {
        let pattern = pattern
            .as_bytes()
            .iter()
            .fold(CharMap::default(), |mut acc, x| {
                acc.insert(*x);
                acc
            });
        Self {
            pattern,
            inner: Default::default(),
            valid: 0,
        }
    }
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut slow = 0;
        let mut fast = 0;
        let bytes = s2.as_bytes();

        let mut windows = Windows::new(&s1);
        while fast < bytes.len() {
            windows.insert(bytes[fast]);

            // println!("{}", windows.valid);

            while windows.pattern.len == windows.valid {
                // println!("in {}", windows.valid);
                // println!("fast: {fast}, slow: {slow}");

                if fast - slow + 1 == s1.len() {
                    return true;
                }
                windows.remove(bytes[slow]);
                slow += 1;
            }

            fast += 1;
        }

        false
    }
}