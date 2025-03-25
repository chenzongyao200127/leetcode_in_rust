// 2711_Difference_of_Number_of_Distinct_Values_on_Diagonals
// https://leetcode.cn/problems/difference-of-number-of-distinct-values-on-diagonals/description/?envType=daily-question&envId=2025-03-25

// The cell answer[r][c] is calculated by looking at the diagonal values of the cell grid[r][c]:

// Let leftAbove[r][c] be the number of distinct values on the diagonal to the left and above the cell grid[r][c] not including the cell grid[r][c] itself.
// Let rightBelow[r][c] be the number of distinct values on the diagonal to the right and below the cell grid[r][c], not including the cell grid[r][c] itself.
// Then answer[r][c] = |leftAbove[r][c] - rightBelow[r][c]|.
use std::collections::HashSet;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![vec![0; n]; m];
        let mut left_top_counts = vec![vec![0; n]; m];
        let mut right_bottom_counts = vec![vec![0; n]; m];

        // Precompute distinct counts for left-top diagonals
        for i in 0..m {
            for j in 0..n {
                let mut cnt = HashSet::new();
                let (mut x, mut y) = (i as i32 - 1, j as i32 - 1);
                while x >= 0 && y >= 0 {
                    cnt.insert(grid[x as usize][y as usize]);
                    x -= 1;
                    y -= 1;
                }
                left_top_counts[i][j] = cnt.len() as i32;
            }
        }

        // Precompute distinct counts for right-bottom diagonals
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut cnt = HashSet::new();
                let (mut x, mut y) = (i + 1, j + 1);
                while x < m && y < n {
                    cnt.insert(grid[x][y]);
                    x += 1;
                    y += 1;
                }
                right_bottom_counts[i][j] = cnt.len() as i32;
            }
        }

        // Calculate the result using precomputed values
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = (left_top_counts[i][j] - right_bottom_counts[i][j]).abs();
            }
        }

        ans
    }
}
