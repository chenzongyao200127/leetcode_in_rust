// 576_Out_of_Boundary_Paths
// https://leetcode.cn/problems/out-of-boundary-paths/

// 给你一个大小为 m x n 的网格和一个球。球的起始坐标为 [startRow, startColumn] 。
// 你可以将球移到在四个方向上相邻的单元格内（可以穿过网格边界到达网格之外）。你 最多 可以移动 maxMove 次球。
// 给你五个整数 m、n、maxMove、startRow 以及 startColumn ，找出并返回可以将球移出边界的路径数量。
// 因为答案可能非常大，返回对 109 + 7 取余 后的结果。

use std::collections::HashMap;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        Self::dfs(m, n , start_row, start_column, max_move, &mut HashMap::new())
    }

    fn dfs(m: i32, n: i32, x: i32, y: i32, k: i32, cache: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
        if x < 0 || x >= m || y < 0 || y >= n  {
            return 1;
        }

        if k == 0 {
            return 0;
        }

        if let Some(&v) = cache.get(&(x, y, k)) {
            return v;
        }

        let mut res = 0;

        for &(dx, dy) in &[(1,0), (-1,0), (0,1), (0,-1)] {
            let x1 = x + dx;
            let y1 = y + dy;

            res += Self::dfs(m, n, x1, y1, k - 1, cache);
            res %= 1000000007;
        }

        cache.insert((x, y, k), res);

        return res;
    }
}




impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![vec![0; n as usize]; m as usize]; max_move as usize + 1];
        dp[0][start_row as usize][start_column as usize] = 1;
        let mut out_counts = 0;
        let mod_num = 1_000_000_007;

        for i in 0..max_move {
            for j in 0..m {
                for k in 0..n {
                    if dp[i as usize][j as usize][k as usize] > 0 {
                        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                            let new_x = j + dx;
                            let new_y = k + dy;
                            if new_x >= 0 && new_x < m && new_y >= 0 && new_y < n {
                                dp[i as usize + 1][new_x as usize][new_y as usize] = 
                                    (dp[i as usize + 1][new_x as usize][new_y as usize] 
                                        + dp[i as usize][j as usize][k as usize]) % mod_num;
                            } else {
                                out_counts = (out_counts + dp[i as usize][j as usize][k as usize]) % mod_num;
                            }
                        }
                    }
                }
            }
        }
        out_counts
    }
}
