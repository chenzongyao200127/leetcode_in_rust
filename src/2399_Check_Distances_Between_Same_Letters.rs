// 2399_Check_Distances_Between_Same_Letters
// https://leetcode.cn/problems/check-distances-between-same-letters/

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut idx_vec: Vec<i32> = vec![-1; 26];
        let s_chars: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            if idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] == -1 {
                idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] = i as i32;
            } else {
                idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] = i as i32 - idx_vec[(s_chars[i] as u8 - 'a' as u8) as usize] - 1;
            }
        }
        // println!("{:?}", idx_vec);
        for i in 0..distance.len() {
            if idx_vec[i] == -1 {
                continue;
            } else {
                if idx_vec[i] != distance[i] {
                    return false;
                }
            }
        }

        true
    }
}


impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut real_distance = vec![usize::MAX; 26];
        for (i, b) in s.bytes().enumerate() {
            let idx = (b - b'a') as usize;
            if real_distance[idx] == usize::MAX {
                real_distance[idx] = i;
            } else {
                real_distance[idx] = i - real_distance[idx] - 1;
            }
        }

        for (i, _n) in real_distance.iter().enumerate() {
            if real_distance[i] != usize::MAX {
                if real_distance[i] != distance[i] as usize {
                    return false;
                }
            }
        }

        true
    }
}


impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut cache = vec![-1; 26];
        for (i, ch) in s.bytes().enumerate() {
            let j = (ch - b'a') as usize;
            if cache[j] != -1 && i as i32 - cache[j] - 1 != distance[j] { return false; }
            cache[j] = i as i32;
        }
        true
    }
}

