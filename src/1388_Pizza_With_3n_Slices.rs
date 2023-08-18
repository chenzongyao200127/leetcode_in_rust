// 1388_Pizza_With_3n_Slices
// https://leetcode.cn/problems/pizza-with-3n-slices/

// 给你一个披萨，它由 3n 块不同大小的部分组成，现在你和你的朋友们需要按照如下规则来分披萨：

// 你挑选 任意 一块披萨。
// Alice 将会挑选你所选择的披萨逆时针方向的下一块披萨。
// Bob 将会挑选你所选择的披萨顺时针方向的下一块披萨。
// 重复上述过程直到没有披萨剩下。
// 每一块披萨的大小按顺时针方向由循环数组 slices 表示。

// 请你返回你可以获得的披萨大小总和的最大值。


impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let t = n / 3;

        fn calc_max_slices(slices: &[i32], t: usize) -> i32 {
            let n = slices.len();
            let mut dp = vec![vec![0; t + 1]; n + 1];

            for i in 1..=n {
                for j in 1..=t {
                    if i == 1 {
                        dp[i][j] = slices[0]; 
                    } else {
                        dp[i][j] = std::cmp::max(
                            dp[i - 1][j],
                            dp[i - 2][j - 1] + slices[i - 1]
                        );
                    }
                }
            }
            
            dp[n][t]
        }

        std::cmp::max(
            calc_max_slices(&slices[..n-1], t), 
            calc_max_slices(&slices[1..], t)    
        )
    }
}
