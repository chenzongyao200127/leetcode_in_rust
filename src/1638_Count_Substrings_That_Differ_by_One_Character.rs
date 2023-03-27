// 1638_Count_Substrings_That_Differ_by_One_Character
// https://leetcode.cn/problems/count-substrings-that-differ-by-one-character/

/// 暴力
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let mut count = 0;

        for i in 0..s.len() {
            for j in i + 1..=s.len() {
                let s_sub = &s[i..j];
                for k in 0..=t.len() - s_sub.len() {
                    let t_sub = &t[k..k + s_sub.len()];
                    if is_diff_by_one(s_sub, t_sub) {
                        count += 1;
                    }
                }
            }
        }
    
        count
    }
}

pub fn is_diff_by_one(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut diff_count = 0;
    let s1_chars = s1.chars().collect::<Vec<char>>();
    let s2_chars = s2.chars().collect::<Vec<char>>();

    for i in 0..s1.len() {
        if s1_chars[i] != s2_chars[i] {
            diff_count += 1;
        }
        if diff_count > 1 {
            return false;
        }
    }

    diff_count == 1
}




/// 官解
/// class Solution:
// def countSubstrings(self, s: str, t: str) -> int:
    // ans = 0
    // for i in range(len(s)):
    //     for j in range(len(t)):
    //         diff = 0
    //         k = 0
    //         while i + k < len(s) and j + k < len(t):
    //             if s[i + k] != t[j + k]:
    //                 diff += 1
    //             if diff == 1:
    //                 ans += 1
    //             elif diff > 1:
    //                 break
    //             k += 1
    // return ans

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut ans = 0;
        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut diff = 0;
                let mut k = 0;
                while (i + k) < s.len() && (j + k) < t.len() {
                    if s[i + k] != t[j + k] {
                        diff += 1;
                    }
                    if diff == 1 {
                        ans += 1;
                    } else if diff > 1 {
                        break
                    }
                    k += 1;
                }
            }
        }
        
        ans
    }
}

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let mut dpl = vec![vec![0; t.len()+1]; s.len()+1];
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        for i in 0..s.len() {
            for j in 0..t.len() {
                if s[i] == t[j] {
                    dpl[i+1][j+1] = dpl[i][j] + 1;
                } else {
                    dpl[i+1][j+1] = 0;
                }
            }
        }
        let mut dpr = vec![vec![0; t.len()+1]; s.len()+1];
        for i in (0..s.len()).rev() {
            for j in (0..t.len()).rev() {
                if s[i] == t[j] {
                    dpr[i][j] = dpr[i+1][j+1] + 1;
                } else {
                    dpr[i][j] = 0;
                }
            }
        }
        let mut ans = 0;
        for i in 0..s.len() {
            for j in 0..t.len() {
                if s[i] != t[j] {
                    ans += (dpl[i][j] + 1) * (dpr[i+1][j+1] + 1);
                }
            }
        }
        ans
    }
}