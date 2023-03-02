// 455. Assign Cookies
// https://leetcode.cn/problems/assign-cookies/

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        let mut ans = 0;
        g.sort_unstable();
        s.sort_unstable();
        let (mut i, mut j) = (0,0);
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                ans += 1;
                j += 1;
                i += 1;
            } else {
                j += 1;
            }
        }
        ans
    }
}


impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut i = 0;
        let mut j = 0;
        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();
        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                res += 1;
                i += 1;
            }
            j += 1;
        }

        res
    }
}