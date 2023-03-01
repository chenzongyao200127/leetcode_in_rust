// 89. Gray Code
// https://leetcode.cn/problems/gray-code/

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = vec![0; 1 << n];
        for i in 0..ans.len() {
            ans[i] = (i>>1 ^ i) as i32
        }

        ans
    }
}