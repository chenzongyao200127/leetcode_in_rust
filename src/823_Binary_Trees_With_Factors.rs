// 823_Binary_Trees_With_Factors
// https://leetcode.cn/problems/binary-trees-with-factors/

// 给出一个含有不重复整数元素的数组 arr ，每个整数 arr[i] 均大于 1。
// 用这些整数来构建二叉树，每个整数可以使用任意次数。其中：每个非叶结点的值应等于它的两个子结点的值的乘积。
// 满足条件的二叉树一共有多少个？答案可能很大，返回 对 109 + 7 取余 的结果。

// 示例 1:
// 输入: arr = [2, 4]
// 输出: 3
// 解释: 可以得到这些二叉树: [2], [4], [4, 2, 2]

// 示例 2:
// 输入: arr = [2, 4, 5, 10]
// 输出: 7
// 解释: 可以得到这些二叉树: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i64 {
        const MOD: i32 = 1_000_000_007;
        
        let mut arr = arr.clone();
        arr.sort();
        
        let index: HashMap<i32, usize> = arr.iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect();
        
        let mut dp = vec![1; arr.len()];
        
        for i in 0..arr.len() {
            for j in 0..i {
                if arr[i] % arr[j] == 0 {
                    let right_child = arr[i] / arr[j];
                    if let Some(&right_index) = index.get(&right_child) {
                        dp[i] = (dp[i] + dp[j] as i64 * dp[right_index] as i64 % MOD as i64) % MOD as i64;
                    }
                }
            }
        }
        
        dp.iter().sum::<i64>() % MOD as i64
    }
}