// 688_Knight_Probability_in_Chessboard
// https://leetcode.cn/problems/knight-probability-in-chessboard/

// 在一个 n x n 的国际象棋棋盘上，一个骑士从单元格 (row, column) 开始，并尝试进行 k 次移动。
// 行和列是 从 0 开始 的，所以左上单元格是 (0,0) ，右下单元格是 (n - 1, n - 1) 。
// 象棋骑士有8种可能的走法，如下图所示。每次移动在基本方向上是两个单元格，然后在正交方向上是一个单元格。

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; k as usize + 1];
        dp[0][row as usize][column as usize] = 1.0;

        for i in 0..k {
            for j in 0..n {
                for l in 0..n {
                    if dp[i as usize][j as usize][l as usize] > 0.0 {
                        for &(dx, dy) in [(-2, -1), (-1, -2), (1, 2), (2, 1), 
                                          (-2, 1), (-1, 2), (1, -2), (2, -1)].iter() {
                            let new_x = j + dx;
                            let new_y = l + dy;
                            if new_x >= 0 && new_x < n && new_y >= 0 && new_y < n {
                                dp[i as usize + 1][new_x as usize][new_y as usize] += 
                                    dp[i as usize][j as usize][l as usize] / 8.0;
                            }
                        }
                    }
                }
            }
        }
        let mut stay_counts = 0.0;
        for i in 0..n {
            for j in 0..n {
                stay_counts += dp[k as usize][i as usize][j as usize];
            }
        }

        stay_counts
    }
}


impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let (n, k, row, column) = (n as usize, k as usize, row as usize, column as usize);
        let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; k as usize + 1];

        for i in 0..n {
            for j in 0..n {
                dp[0][i][j] = 1.0;
            }
        }

        for step in 1..=k {
            for i in 0..n {
                for j in 0..n {
                    for (x, y) in [
                        (i + 2, j + 1),
                        (i + 2, j - 1),
                        (i - 2, j + 1),
                        (i - 2, j - 1),
                        (i + 1, j + 2),
                        (i + 1, j - 2),
                        (i - 1, j + 2),
                        (i - 1, j - 2),
                    ] {
                        if x < n && y < n {
                            dp[step][i][j] += dp[step - 1][x][y] * 0.125;
                        }
                    }
                }
            }
        }
        
        dp[k][row][column]
    }
}