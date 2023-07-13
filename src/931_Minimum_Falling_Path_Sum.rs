// 931. Minimum Falling Path Sum
// https://leetcode.cn/problems/minimum-falling-path-sum/description/


impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = matrix.clone();

        for i in 1..rows {
            for j in 0..cols {
                let left_up = if j > 0 { dp[i - 1][j - 1] } else { i32::MAX };
                let up = dp[i - 1][j];
                let right_up = if j < cols - 1 { dp[i - 1][j + 1] } else { i32::MAX };
                dp[i][j] += left_up.min(up).min(right_up);
            }
        }

        dp[rows - 1].iter().min().unwrap().clone()
    }
}