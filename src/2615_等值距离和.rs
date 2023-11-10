// https://leetcode.cn/problems/sum-of-distances/

use std::collections::HashMap;
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            map.entry(n).and_modify(|v| { v.push(i); }).or_insert_with(|| {
                let mut v = vec![];
                v.push(i);
                v
            });
        }

        let mut arr = vec![0; nums.len()];
        for a in map.values() {
            let len = a.len();
            let mut s = 0;
            for x in a {
                s += x - a[0]
            }
            arr[a[0]] = s as i64;

            for i in 1..len {
                if i* 2 >= len {
                    s += (i * 2 - len) * (a[i] - a[i-1]);
                } else {
                    s -= (len - i * 2) * (a[i] - a[i-1]);
                }
                arr[a[i]] = s as  i64
            }
        }

        arr
    }
}


use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        // Collect indices for each number
        for (i, &n) in nums.iter().enumerate() {
            map.entry(n).or_default().push(i);
        }

        let mut arr = vec![0; nums.len()];
        // Calculate distances
        for (i, &n) in nums.iter().enumerate() {
            if let Some(indices) = map.get(&n) {
                // Pre-calculate sum of indices and total occurrences
                let total_indices_sum: usize = indices.iter().sum();
                let total_occurrences = indices.len();
                // Calculate the absolute distance using the formula
                let abs_add = total_indices_sum as i64 - (i * total_occurrences) as i64
                              + (total_occurrences as i64 - 1) * i as i64;
                arr[i] = abs_add;
            }
        }

        arr
    }
}
