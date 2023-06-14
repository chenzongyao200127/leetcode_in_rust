// 1375_Number_of_Times_Binary_String_Is_Prefix-Aligned
// https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned/

// 给你一个长度为 n 、下标从 1 开始的二进制字符串，所有位最开始都是 0 。我们会按步翻转该二进制字符串的所有位（即，将 0 变为 1）。
// 给你一个下标从 1 开始的整数数组 flips ，其中 flips[i] 表示对应下标 i 的位将会在第 i 步翻转。
// 二进制字符串 前缀一致 需满足：在第 i 步之后，在 闭区间 [1, i] 内的所有位都是 1 ，而其他位都是 0 。
// 返回二进制字符串在翻转过程中 前缀一致 的次数。

// 超时
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut s = String::new();
        let mut ans = 0;
        for _ in 0..flips.len() {
            s.push('0');
        }
        let mut s = s.chars().collect::<Vec<char>>();
        for i in 0..flips.len() { 
            s[flips[i] as usize - 1] = '1';
            if is_prefix_aligned(&s, i+1) {
                ans += 1;
            }
        }
        ans
    }    
}

pub fn is_prefix_aligned(s: &Vec<char>, idx: usize) -> bool {
    let mut s_tmp = String::new();
    for _ in 0..idx { 
        s_tmp.push('1');
    }
    for _ in idx..s.len() {
        s_tmp.push('0');
    }
    &s_tmp.chars().collect::<Vec<char>>() == s
}


impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut pre_sums = vec![];
        let mut compares = vec![];
        let mut pre_sum = 0;
        let mut compare = 0;
        for i in 0..flips.len() {
            pre_sum += flips[i] as usize;
            compare += i + 1;
            pre_sums.push(pre_sum);
            compares.push(compare);
        }
        let mut ans = 0;
        for i in 0..flips.len() {
            if pre_sums[i] as usize == compares[i] {
                ans += 1;
            }
        }
        ans
    }
}


impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut pre_sum = 0;
        let mut compare = 0;
        let mut ans = 0;
        
        for (i, flip) in flips.into_iter().enumerate() {
            pre_sum += flip as usize;
            compare += i + 1;
            if pre_sum == compare {
                ans += 1;
            }
        }
        
        ans
    }
}