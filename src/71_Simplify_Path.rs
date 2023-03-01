// 71. Simplify Path
// https://leetcode.cn/problems/simplify-path/
// 0ms 100%
// 2.1 MB 82.35%
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        path.split('/').for_each(|s| {
            match s {
                "." | "" => (),
                ".." => { stack.pop(); },
                _ => { stack.push(s); },
            }
        });

        "/".to_string() + &stack.join("/")
    }
}