// 3_Longest_Substring_Without_Repeating_Characters
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = vec![0; 128];
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut rk = -1;
        let mut ans = 0;
        for i in 0..n {
            if i != 0 {
                set[s[i-1] as usize] = 0;
            }
            while (rk + 1) < n && set[s[rk + 1] as usize] == 0 {
                set[s[rk + 1] as usize] == 1;
                rk += 1;
            }
            ans = ans.max(rk - i + 1);
        }
        ans
    }
}







impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut ans = 0;
        let mut cache = vec![0; 128];
        s.chars().enumerate().for_each(|(i, c)| {
            l = l.max(cache[c as usize]);
            ans = ans.max(i as i32 - l + 1);
            cache[c as usize] = i as i32 + 1;
        });
        ans
    }
}