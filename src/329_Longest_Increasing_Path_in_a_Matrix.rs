// 329_Longest_Increasing_Path_in_a_Matrix
// https://leetcode.cn/problems/longest-increasing-path-in-a-matrix/description/

// 给定一个 m x n 整数矩阵 matrix ，找出其中 最长递增路径 的长度。

// 对于每个单元格，你可以往上，下，左，右四个方向移动。 
// 你 不能 在 对角线 方向上移动或移动到 边界外（即不允许环绕）。

// 回溯超时
use std::collections::HashSet;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
    
        #[inline]
        fn dfs(matrix: &Vec<Vec<i32>>, cur_x: i32, cur_y: i32, length: i32, memo: &mut HashSet<(i32, i32)>) -> i32 {
            let dirs = vec![(1,0), (0,1), (-1,0), (0,-1)];
            let mut max_length = length;
            for (dx, dy) in dirs {
                let new_x = cur_x + dx;
                let new_y = cur_y + dy;
    
                if new_x >= 0 && new_x < matrix.len() as i32 && new_y >= 0 && new_y < matrix[0].len() as i32 
                        && !memo.contains(&(new_x, new_y)) 
                        && matrix[new_x as usize][new_y as usize] > matrix[cur_x as usize][cur_y as usize] {
                    memo.insert((new_x, new_y));
                    max_length = max_length.max(dfs(matrix, new_x, new_y, length + 1, memo));
                    memo.remove(&(new_x, new_y));
                }
            }
            max_length
        }
    
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let mut memo: HashSet<(i32, i32)> = HashSet::new();
                memo.insert((i as i32, j as i32));
                ans = ans.max(dfs(&matrix, i as i32, j as i32, 1, &mut memo));
            }
        }
    
        ans
    }
}



// 记忆化搜索 + DP
use std::cmp::max;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols]; rows];
        let mut ans = 0;

        #[inline]
        fn dfs(matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
            // 记忆化
            if dp[x][y] != 0 {
                return dp[x][y];
            }

            let dx = vec![1, 0, -1, 0];
            let dy = vec![0, 1, 0, -1];
            let mut max_length = 1;

            for i in 0..4 {
                let new_x = x as i32 + dx[i];
                let new_y = y as i32 + dy[i];

                if new_x >= 0 && new_x < matrix.len() as i32 && new_y >= 0 && new_y < matrix[0].len() as i32 
                    && matrix[new_x as usize][new_y as usize] > matrix[x][y] {
                    max_length = max(max_length, dfs(matrix, dp, new_x as usize, new_y as usize) + 1);
                }
            }
            
            dp[x][y] = max_length;
            max_length
        }

        for i in 0..rows {
            for j in 0..cols {
                ans = max(ans, dfs(&matrix, &mut dp, i, j));
            }
        }

        ans
    }
}