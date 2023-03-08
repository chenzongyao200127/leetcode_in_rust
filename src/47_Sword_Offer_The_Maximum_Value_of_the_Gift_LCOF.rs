// Sword_Offer_47_The_Maximum_Value_of_the_Gift_LCOF
// https://leetcode.cn/problems/li-wu-de-zui-da-jie-zhi-lcof/


// 在一个 m*n 的棋盘的每一格都放有一个礼物，每个礼物都有一定的价值（价值大于 0）。
// 你可以从棋盘的左上角开始拿格子里的礼物，并每次向右或者向下移动一格、直到到达棋盘的右下角。
// 给定一个棋盘及其上面的礼物的价值，请计算你最多能拿到多少价值的礼物？
// 示例 1:
// 输入: 
// [
//   [1,3,1],
//   [1,5,1],
//   [4,2,1]
// ]
// 输出: 12
// 解释: 路径 1→3→5→2→1 可以拿到最多价值的礼物

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut dp = vec![vec![0; m]; n];
        let mut pre_sum = 0;
        for i in 0..m {
            pre_sum += grid[0][i];
            dp[0][i] = pre_sum;
        }
        pre_sum = 0;
        for i in 0..n {
            pre_sum += grid[i][0];
            dp[i][0] = pre_sum;
        }
        for i in 1..n {
            for j in 1..m {
                dp[i][j] = dp[i][j-1].max(dp[i-1][j]) + grid[i][j];
                // println!("{:?}", dp);
            }
        }
        dp[n-1][m-1]
    }
}

/// 原地修改
/// 一维DP
impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid[0].len() + 1];
        for i in 1..=grid.len() {
            for j in 1..=grid[0].len() { dp[j] = dp[j].max(dp[j - 1]) + grid[i - 1][j - 1]; }
        }
        dp[grid[0].len()]
    }
}
// 作者：kyushu
// 链接：https://leetcode.cn/problems/li-wu-de-zui-da-jie-zhi-lcof/solution/rustjava-yuan-di-xiu-gai-or-yi-wei-dp-by-yibu/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。



impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        // (x,y)
        let mut matrix = vec![vec![0; n]; m];
        matrix[0][0] = grid[0][0];
        for x in 1..m {
            matrix[x][0] = grid[x][0] + matrix[x -1][0];
        }
        for y in 1..n {
            matrix[0][y] = grid[0][y] + matrix[0][y-1];
        }
        for x_ptr in 1..m {
            for y_ptr in 1..n {
                // println!("{}, {}", x_ptr, y_ptr);
                matrix[x_ptr][y_ptr] = std::cmp::max(matrix[x_ptr - 1][y_ptr], matrix[x_ptr][y_ptr - 1]) +grid[x_ptr][y_ptr];
            }
        }
        matrix[m - 1][n - 1]
    }
}