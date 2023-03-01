// 1247. Minimum Swaps to Make Strings Equal
// https://leetcode.cn/problems/minimum-swaps-to-make-strings-equal/

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (mut x_total, mut y_total) = (0, 0);
        let s1_chars:Vec<char> = s1.chars().collect();
        let s2_chars:Vec<char> = s2.chars().collect();
    
        s1_chars.iter().chain(s2_chars.iter()).for_each(|c| {
            if *c == 'x' { x_total += 1 } else { y_total += 1 }
        });
        if x_total & 1 == 1 {
            return -1;
        }
        let mut diff_s1 = vec![];
        let mut diff_s2 = vec![];
        for i in 0..s1.len() {
            if s1_chars[i] != s2_chars[i] {
                diff_s1.push(s1_chars[i]);
                diff_s2.push(s2_chars[i]);
            }
        }
    
        let mut ans = 0;
        let mut cnt = 0;
        for i in (0..diff_s1.len()).filter(|i| (i%2 == 0)) {
            if (diff_s1[i] == 'x' && diff_s1[i+1] == 'x') || (diff_s1[i] == 'y' && diff_s1[i+1] == 'y') {
                ans += 1;
            } else {
                cnt += 1;
            }
        }
    
        if cnt & 1 == 1 {
            ans + cnt + 1
        } else {
            ans + cnt
        }
    }
}


impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let (mut x, mut y) = (0, 0);
        for (a, b) in s1.chars().zip(s2.chars()) {
            match (a, b) {
                ('x', 'y') => x += 1,
                ('y', 'x') => y += 1,
                _ => (),
            };
        }
    
        if (x + y) % 2 == 1 {
            -1
        } else {
            (x + 1) / 2 + (y + 1) / 2
        }
    }
}
