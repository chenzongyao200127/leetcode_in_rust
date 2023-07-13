// 338_Counting_Bits
// https://leetcode.cn/problems/counting-bits/description/

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0];
        for i in 1..=n {
            if i % 2 == 0 {
                ans.push(ans[i as usize/2]);
            } else {
                ans.push(ans[i as usize/2] + 1); 
            }
        }
        ans
    }
}


impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        for i in 0..=num {
            vec.push(i.count_ones() as i32);
        }
        vec
    }
}


impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        (1..=num as usize).fold(vec![0], |mut v, i| {v.push(&v[i & (i - 1)] + 1); v})
    }
}
