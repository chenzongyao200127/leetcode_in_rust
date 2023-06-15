// 1177_Can_Make_Palindrome_from_Substring
// https://leetcode.cn/problems/can-make-palindrome-from-substring/

// impl Solution {
//     pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
//         let mut ans = vec![];
//         for query in queries {
//             let left = query[0] as usize;
//             let right = query[1] as usize;
//             let k = query[2];
//             if k as usize >= helper(&s[left..=right]) / 2 {
//                 ans.push(true);
//             } else {
//                 ans.push(false);
//             }
//         }
    
//         ans
//     }    
// }

// pub fn helper(s: &str) -> usize {
//     let len = s.len();
//     let s1 = &s[0..len/2];
//     let mut s2 = &s[len/2..len];
//     if len % 2 == 1 { 
//         s2 = &s2[1..];
//     }
//     let mut diff = 0;
//     for i in 0..s.len()/2 {
//         if s1.chars().nth(i) != s2.chars().nth(s2.len()-1-i) {
//             diff += 1;
//         }
//     }
//     diff
// }




impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let pre = s.chars().fold(vec![0], |mut pre: Vec<i32>, c: char| {
            pre.push(pre.last().unwrap() ^ (1 << (c as u8 - b'a')));
            pre
        });
        queries.into_iter().map(|v| {
            let (l, r, k) = (v[0] as usize, v[1] as usize + 1, v[2]);
            let cnt = (pre[l] ^ pre[r]).count_ones();
            if (r - l) % 2 == 1 {
                cnt as i32 - 1 <= k * 2
            } else {
                cnt as i32 <= k * 2
            }
        }).collect()
    }
}