// 52_N-Queens_II
// https://leetcode.cn/problems/n-queens-ii/

// n 皇后问题 研究的是如何将 n 个皇后放置在 n × n 的棋盘上，并且使皇后彼此之间不能相互攻击。
// 给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut board = vec![vec![false; n as usize]; n as usize];
        let mut ans = 0;
        Self::trace_back(n, 0, &mut board, &mut ans);
        ans
    }

    fn trace_back(n: i32, row: usize, board: &mut Vec<Vec<bool>>, ans: &mut i32) {
        if row >= n as usize {
            *ans += 1;
            return;
        }

        for col in 0..n as usize {
            if Self::is_safe(row, col, n, &board) {
                board[row][col] = true;
                Self::trace_back(n, row + 1, board, ans);
                board[row][col] = false;
            }
        }
    }

    #[inline]
    fn is_safe(row: usize, col: usize, n: i32, board: &Vec<Vec<bool>>) -> bool {
        // Check if there's a queen in the same column
        for i in 0..row {
            if board[i][col] {
                return false;
            }
        }

        // Check if there's a queen in the top-left diagonal
        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] {
                return false;
            }
            i -= 1;
            j -= 1;
        }

        // Check if there's a queen in the top-right diagonal
        i = row as i32 - 1;
        j = col as i32 + 1;
        while i >= 0 && j < n {
            if board[i as usize][j as usize] {
                return false;
            }
            i -= 1;
            j += 1;
        }

        true
    }
}



impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        const EMPTY_CELL: &str = ".";
        const QUEEN_CELL: &str = "Q";

        let n = n as usize;
        let mut board = vec![EMPTY_CELL.repeat(n); n];
        let mut res = 0;
        let mut stack = (0..n).rev().map(|j| (0, j, false)).collect::<Vec<_>>();
        let (mut col, mut diag_positive, mut diag_negative) = (
            vec![false; n],
            vec![false; 2 * n - 1],
            vec![false; 2 * n - 1],
        );

        while let Some((row, col_idx, has_queen)) = stack.pop() {
            if has_queen {
                board[row].replace_range(col_idx..=col_idx, EMPTY_CELL);
                col[col_idx] = false;
                diag_positive[n - 1 + row - col_idx] = false;
                diag_negative[row + col_idx] = false;
            } else {
                board[row].replace_range(col_idx..=col_idx, QUEEN_CELL);
                col[col_idx] = true;
                diag_positive[n - 1 + row - col_idx] = true;
                diag_negative[row + col_idx] = true;
                stack.push((row, col_idx, true));

                if row == n - 1 {
                    res += 1;
                    continue;
                }

                for j in (0..n).rev() {
                    if !col[j] && !diag_positive[n + row - j] && !diag_negative[row + 1 + j] {
                        stack.push((row + 1, j, false));
                    }
                }
            }
        }

        res
    }
}



impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn not_valid(cols: &Vec<usize>, cur: usize) -> bool {
            cols.iter().enumerate().any(|(r, &c)| {
                cur == c
                    || cols.len() == r
                    || (cur as i64 - c as i64).abs() == (cols.len() as i64 - r as i64).abs()
            })
        }
        fn dfs(n: usize, cols: &mut Vec<usize>, ans: &mut i32) {
            if cols.len() == n {
                *ans += 1;
                return;
            }
            for cur in 0..n {
                if not_valid(cols, cur) {
                    continue;
                }
                cols.push(cur);
                dfs(n, cols, ans);
                cols.pop();
            }
        }
        let mut ans = 0;
        dfs(n as usize, &mut vec![], &mut ans);
        ans
    }
}
