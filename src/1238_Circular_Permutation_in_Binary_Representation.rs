// 1238. Circular Permutation in Binary Representation
// https://leetcode.cn/problems/circular-permutation-in-binary-representation/

// https://leetcode.cn/problems/gray-code/
// 格雷码（Gray code）
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut ans = vec![0; 1 << n];
        for i in 0..ans.len() {
            ans[i] = ((i >> 1) ^ i ^ start as usize) as i32;
        }
        ans
    }

}