// 1091_Shortest_Path_in_Binary_Matrix
// https://leetcode.cn/problems/shortest-path-in-binary-matrix/


use std::collections::VecDeque;
use std::usize::MAX;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }

        let n = grid.len();
        let mut dist = vec![vec![MAX; n]; n];
        dist[0][0] = 1;

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];

        while let Some((x,y)) = queue.pop_front() {
            if x == n-1 && y == n-1 {
                return dist[x][y] as i32;
            }

            for dic in directions.iter() {
                let dx = x as i32 + dic.0;
                let dy = y as i32 + dic.1;
                
                if dx < 0 || dx >= n as i32 || dy < 0 || dy >= n as i32 {
                    continue;
                }

                let dx = dx as usize;
                let dy = dy as usize;

                if grid[dx][dy] == 1 || dist[dx][dy] <= dist[x][y] + 1 {
                    continue;
                }

                dist[dx][dy] = dist[x][y] + 1;
                queue.push_back((dx, dy));
            }
        }

        -1
    }
}


impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        if grid[n-1][m-1] == 1 || grid[0][0] == 1{
            return -1;
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back((0i32,0i32));
        grid[0][0] = 2;
        let dirs = [(0,1),(0,-1),(1,-1),(1,0),(1,1),(-1,-1),(-1,0),(-1,1)];
        while let Some((x,y)) = q.pop_front() {
            let td = grid[x as usize][y as usize];
            for &(dx,dy) in &dirs {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 && grid[nx as usize][ny as usize] == 0 {
                    grid[nx as usize][ny as usize] = td + 1;
                    q.push_back((nx,ny));
                }
            }
        }
        grid[n-1][m-1] - 1
    }
}