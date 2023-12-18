// 2132_Stamping_the_Grid
// https://leetcode.cn/problems/stamping-the-grid/description/?envType=daily-question&envId=2023-12-14

impl Solution {
    // 二维前缀和 + 二维差分数组
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        // 初始化二维前缀和
        // 定义sum[i+1][j+1]为左上角为[0,0]，右下角为[i,j]的子矩阵元素和
        // 计算任意矩阵的元素和
        // 假设子矩阵的左上角为[r1, c1], 右下角为[r2-1, c2-1]
        // 则该子矩阵的和为 sum[r2][c2] - sum[r1][c2] - sum[r2][c1] + sum[r1][c1]
        let mut s = vec![vec![0; n + 1]; m + 1];
        for (i, row) in grid.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                s[i+1][j+1] = s[i + 1][j] + s[i][j + 1] - s[i][j] + x
            }
        }

        // 计算二维差分
        let mut d = vec![vec![0; n + 2]; m + 2];
        for i2 in stamp_height as usize..=m {
            for j2 in stamp_width as usize..=n {
                let i1 = i2 - stamp_height as usize + 1;
                let j1 = j2 - stamp_width as usize + 1;
                if s[i2][j2] - s[i2][j1 - 1] - s[i1 - 1][j2] + s[i1 - 1][j1 - 1] == 0 {
                    d[i1][j1] += 1;
                    d[i1][j2 + 1] -= 1;
                    d[i2 + 1][j1] -= 1;
                    d[i2 + 1][j2 + 1] += 1;
                }
            }
        }

        for (i, row) in grid.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                d[i + 1][j + 1] += d[i + 1][j] + d[i][j + 1] - d[i][j];
                if v == 0 && d[i + 1][j + 1] == 0 {
                    return false;
                }
            }
        }

        true
    }
}