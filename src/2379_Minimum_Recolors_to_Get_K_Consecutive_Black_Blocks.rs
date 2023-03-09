// 2379_Minimum_Recolors_to_Get_K_Consecutive_Black_Blocks
// https://leetcode.cn/problems/minimum-recolors-to-get-k-consecutive-black-blocks/


impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let s_chars: Vec<char> = blocks.chars().collect();
        let mut ans = 0;
        let mut tmp = 0;
        s_chars.windows(k as usize).fold(200, |acc, vec| {
            let black = vec.iter().filter(|&ch| *ch == 'W').count();
            acc.min(black as i32)
        })
    }
}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        blocks
            .into_bytes()
            .windows(k as usize)
            .fold(200, |acc, vec| {
                let black = vec.iter().filter(|&ch| *ch == b'W').count();
                acc.min(black as i32)
            })
    }
}


impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let (n, mut accu, mut res, blocks, k) = (blocks.len(), 0, 0, blocks.as_bytes(), k as usize);
        for i in 0..k {
            if blocks[i] as char == 'W' {
                accu += 1
            }
        }
        res = accu;
        for i in k..n {
            if blocks[i] as char == 'W' {
                accu += 1
            }
            if blocks[i-k] as char == 'W' {
                accu -= 1
            }
            res = std::cmp::min(res, accu)
        }
        res
    }
}


// class Solution:
//     def minimumRecolors(self, blocks: str, k: int) -> int:
//         ans = inf 
//         cnt = 0 
//         for i, ch in enumerate(blocks): 
//             if ch == 'W':
//                 cnt += 1
//             if i >= k and blocks[i-k] == 'W':
//                 cnt -= 1
//             if i >= k - 1:
//                 ans = min(ans, cnt)
//         return ans 