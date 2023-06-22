// 面试题 16.19. 水域大小
// https://leetcode.cn/problems/pond-sizes-lcci/

impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let m = land.len();
        let n = land[0].len();

        fn dfs(x: usize, y: usize, land: &mut Vec<Vec<i32>>) -> i32 {
            let m = land.len();
            let n = land[0].len();

            if x >= m || y >= n || land[x][y] != 0 {
                return 0;
            }

            land[x][y] = -1;
            let mut res = 1;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 {
                        res += dfs(nx as usize, ny as usize, land);
                    }
                }
            }

            res
        }

        let mut res = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if land[i][j] == 0 {
                    res.push(dfs(i, j, &mut land));
                }
            }
        }

        res.sort();
        res
    }
}