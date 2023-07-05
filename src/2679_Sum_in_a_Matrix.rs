// 2679_Sum_in_a_Matrix
// https://leetcode.cn/problems/sum-in-a-matrix/

// 给你一个下标从 0 开始的二维整数数组 nums 。一开始你的分数为 0 。你需要执行以下操作直到矩阵变为空：

// 矩阵中每一行选取最大的一个数，并删除它。如果一行中有多个最大的数，选择任意一个并删除。
// 在步骤 1 删除的所有数字中找到最大的一个数字，将它添加到你的 分数 中。
// 请你返回最后的 分数 。

impl Solution {
    pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        let mut new_nums = vec![];
        for vec in nums.into_iter() {
            let mut sort_vec = vec;
            sort_vec.sort_unstable();
            new_nums.push(sort_vec);
        }
    
        let mut ans = 0;
        for i in 0..new_nums[0].len() {
            let mut tmp = new_nums[0][i];
            for j in 1..new_nums.len() {
                tmp = tmp.max(new_nums[j][i]);
            }
            ans += tmp;
        }
        ans
    }
}


impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        for list in nums.iter_mut() {
            list.sort();
        }
        let mut res: i32 = 0;
        let (n, m) = (nums.len(), nums[0].len());
        for col in 0..m {
            res += nums.iter().map(|x| x[col]).max().unwrap();
        }
        res
    }
}
