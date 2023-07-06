// 695_Max_Area_of_Island
// https://leetcode.cn/problems/max-area-of-island/

// 给你一个大小为 m x n 的二进制矩阵 grid 。

// 岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在 水平或者竖直的四个方向上 相邻。
// 你可以假设 grid 的四个边缘都被 0（代表水）包围着。

// 岛屿的面积是岛上值为 1 的单元格的数目。

// 计算并返回 grid 中最大的岛屿面积。如果没有岛屿，则返回面积为 0 。

use std::cmp::max;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut max_area = 0;

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 1 {
                    let mut area = 0;
                    let mut stack = vec![(i, j)];

                    while let Some((x, y)) = stack.pop() {
                        // 不用判断是否小于0，因为 usize -1 == usize::MAX
                        if x >= row || y >= col || grid[x][y] == 0 { 
                            continue 
                        };

                        area += 1;
                        grid[x][y] = 0;

                        stack.push((x + 1, y));
                        stack.push((x - 1, y));
                        stack.push((x, y + 1));
                        stack.push((x, y - 1));
                    }
                    max_area = max(area, max_area);
                }
            }
        }
        
        max_area
    }
}



pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();

    fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<i32>>, directions: &[(i32, i32)]) -> i32 {
        let mut size = 1;
        grid[x][y] = 0;
        for &(dx, dy) in directions.iter() {
            let nx = dx + x as i32;
            let ny = dy + y as i32;
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 && grid[nx as usize][ny as usize] == 1 {
                size += dfs(nx as usize, ny as usize, grid, directions);
            }
        }
        size
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans = ans.max(dfs(i, j, &mut grid.to_vec(), &directions));
            }
        }
    }

    ans
}