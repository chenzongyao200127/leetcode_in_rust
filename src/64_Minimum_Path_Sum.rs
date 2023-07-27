// https://leetcode.cn/problems/minimum-path-sum/
// 64. 最小路径和

// 给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

// 说明：每次只能向下或者向右移动一步。

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];

        let mut pre_sum = 0;
        for i in 0..m {
            pre_sum += grid[i][0];
            dp[i][0] = pre_sum;
        }
        pre_sum = 0;
        for j in 0..n {
            pre_sum += grid[0][j];
            dp[0][j] = pre_sum;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = grid[i][j] + dp[i-1][j].min(dp[i][j-1]);
            }
        }
    
        dp[m-1][n-1]
    }
}