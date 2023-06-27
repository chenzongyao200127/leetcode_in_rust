// 46_Permutations
// https://leetcode.cn/problems/permutations/submissions/

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(com: &mut Vec<i32>, nums: &[i32], n: usize, ans: &mut Vec<Vec<i32>>) {
            if com.len() == n {
                ans.push(com.clone());
                return;
            }

            for (i, &num) in nums.iter().enumerate() {
                com.push(num);
                let new_nums = [&nums[..i], &nums[i + 1..]].concat();
                dfs(com, &new_nums, n, ans);
                com.pop();
            }
        }

        let mut ans = Vec::new();
        let n = nums.len();

        dfs(&mut Vec::new(), &nums, n, &mut ans);
        ans
    }
}