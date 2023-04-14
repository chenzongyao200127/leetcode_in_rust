// 2404_Most_Frequent_Even_Element
// https://leetcode.cn/problems/most-frequent-even-element/

use std::collections::HashMap;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let nums: Vec<i32> = nums.into_iter().filter(|x| x & 1 == 0).collect(); // Use `into_iter()` instead of `iter()`

        if nums.len() == 0 {
            return -1;
        }

        let mut map: HashMap<i32, usize> = HashMap::new();

        for n in nums {
            map.entry(n).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let mut res = vec![];

        for (k, v) in map {
            res.push((k, v));
        }

        res.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        res = res.iter().filter(|&(k, v)| *v == res[0].1).cloned().collect(); // Assign the filtered results back to the `res` variable
        res.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        res[0].0
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        for &x in nums.iter() {
            if x % 2 == 0 {
                *cnt.entry(x).or_insert(0) += 1;
            }
        }

        let mut ans = -1;
        let mut mx = 0;

        for (&x, &v) in cnt.iter() {
            if mx < v || (mx == v && ans > x) {
                ans = x;
                mx = v;
            }
        }
        
        ans
    }
}