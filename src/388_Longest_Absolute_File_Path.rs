// 388. Longest Absolute File Path
// https://leetcode.cn/problems/longest-absolute-file-path/
// 0 ms 100%
// 2 MB 50%
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut max = 0;
        let mut cur_path = vec![];
        let mut cur_depth = -1;

        input.split('\n').for_each(|s| {
            let s_pre = pre_tab_nums(s);
            if s_pre > cur_depth {
                cur_path.push(s);
                cur_depth = cur_depth + 1;
            } else if s_pre == cur_depth {
                cur_path.pop();
                cur_path.push(s);
            } else {
                for _ in 0..cur_depth-s_pre+1 {
                    cur_path.pop();
                }
                cur_path.push(s);
                cur_depth = s_pre;
            }
            if cur_path[cur_path.len()-1].contains(".") {
                max = max.max(("".to_string() + &cur_path.join("/")).len() as i32 - ((cur_depth+1)*cur_depth/2));
            }
        });

        max
    }
}

pub fn pre_tab_nums(s: &str) -> i32 {
    let mut n = 0;
    s.chars().into_iter().for_each(|ch| {
        if ch == '\t' {
            n = n + 1;
        }
    });

    n
}


// Others' Solutions
impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut prev = vec![];
        let mut prev_sum = 0;
        input.split('\n').fold(0, |mut ans, s| {
            let depth = s.chars().filter(|c| *c == '\t').count();
            let len = s.len() - depth;
            while prev.len() > depth {
                prev_sum -= prev.pop().unwrap_or(0);
            }
            if s.contains('.') {
                ans = ans.max(prev_sum + len);
            } else {
                prev.push(len + 1);
                prev_sum = prev_sum + len + 1;
            }
            ans
        }) as i32
    }
}
