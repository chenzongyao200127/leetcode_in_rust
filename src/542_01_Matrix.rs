// 542_01_Matrix
// https://leetcode.cn/problems/01-matrix/description/

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const D: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut queue = std::collections::VecDeque::new();
        let (n, m) = (mat.len() as i32, mat[0].len() as i32);

        for (i, row) in mat.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if *col == 0 {
                    queue.push_back((i as i32, j as i32));
                } else {
                    *col = i32::MAX;
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for (dx, dy) in D {
                let x = i + dx;
                let y = j + dy;

                if x >= 0
                    && x < n
                    && y >= 0
                    && y < m
                    && mat[x as usize][y as usize] > mat[i as usize][j as usize] + 1
                {
                    mat[x as usize][y as usize] = mat[i as usize][j as usize] + 1;
                    queue.push_back((x, y));
                }
            }
        }

        mat
    }
}
