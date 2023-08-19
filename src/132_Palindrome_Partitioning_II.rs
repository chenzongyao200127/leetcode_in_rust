// 132_Palindrome_Partitioning_II
// https://leetcode.cn/problems/palindrome-partitioning-ii/

// Given a string s, partition s such that every substring 
// of the partition is a palindrome.

// Return the minimum cuts needed for a palindrome partitioning of s.

// 错误的思路
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let mut s1 = s.clone();
        let mut s2: String;
        let mut cnt = 0;
    
        loop {
            s2 = s1.chars().rev().collect();
            match longest_common_substring(&s1, &s2) {
                Some(common) => {
                    s1 = remove_substring_from(&s1, common);
                    cnt += 1;
                },
                None => break,
            }
        }
    
        cnt - 1
    }
}


pub fn longest_common_substring<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let mut longest_len = 0;
    let mut longest_pos = 0;

    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1_bytes[i-1] == s2_bytes[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
                if dp[i][j] > longest_len {
                    longest_len = dp[i][j];
                    longest_pos = i - longest_len;
                }
            }
        }
    }

    if longest_len == 0 {
        None
    } else {
        Some(&s1[longest_pos..longest_pos + longest_len])
    }
}

pub fn remove_substring_from(s: &str, sub: &str) -> String {
    s.replace(sub, "")
}



impl Solution {
    pub fn min_cut(s: String) -> i32 {
            
    }
}