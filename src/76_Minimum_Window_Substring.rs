// 76_Minimum_Window_Substring
// https://leetcode.cn/problems/minimum-window-substring/


impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let len = s.len();
        let t: Vec<char> = t.chars().collect();
        let mut window = vec![-1; t.len()];
        for i in 0..len {
            if t.contains(&s[i]) {

            }
        }

    }
}