// 2596_检查骑士巡视方案
// https://leetcode.cn/problems/check-knight-tour-configuration/description/

// 骑士在一张 n x n 的棋盘上巡视。在有效的巡视方案中，骑士会从棋盘的 左上角 出发，并且访问棋盘上的每个格子 恰好一次 。

// 给你一个 n x n 的整数矩阵 grid ，由范围 [0, n * n - 1] 内的不同整数组成
// 其中 grid[row][col] 表示单元格 (row, col) 是骑士访问的第 grid[row][col] 个单元格。骑士的行动是从下标 0 开始的。

// 如果 grid 表示了骑士的有效巡视方案，返回 true；否则返回 false。

// 注意，骑士行动时可以垂直移动两个格子且水平移动一个格子，或水平移动两个格子且垂直移动一个格子。下图展示了骑士从某个格子出发可能的八种行动路线。

// 输入：grid = [[0,11,16,5,20],[17,4,19,10,15],[12,1,8,21,6],[3,18,23,14,9],[24,13,2,7,22]]
// 输出：true
// 解释：grid 如上图所示，可以证明这是一个有效的巡视方案。
// 示例 2：


// 输入：grid = [[0,3,6],[5,8,1],[2,7,4]]
// 输出：false
// 解释：grid 如上图所示，考虑到骑士第 7 次行动后的位置，第 8 次行动是无效的。

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            return false;
        }

        // Constant directions for knight moves
        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -2), (-2, -1), (-1, 2), (-2, 1),
            (1, 2), (2, 1), (1, -2), (2, -1)
        ];

        #[inline]
        fn is_valid_move(x: usize, y: usize, board: &Vec<Vec<i32>>, next_value: i32) -> bool {
            x < board.len() && y < board.len() && board[x][y] == next_value
        }

        fn dfs(x: usize, y: usize, board: &Vec<Vec<i32>>, cur_value: i32) -> bool {
            if cur_value == (board.len() * board.len() - 1) as i32 {
                return true;
            }

            let mut valid_move_found = false;
            for &(dx, dy) in DIRECTIONS.iter() {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if new_x >= 0 && new_y >= 0 && is_valid_move(new_x as usize, new_y as usize, board, cur_value + 1) {
                    valid_move_found |= dfs(new_x as usize, new_y as usize, board, cur_value + 1);
                }
            }

            valid_move_found
        }

        dfs(0, 0, &grid, 0)
    }
}
