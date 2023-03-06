// 48. Rotate Image
// https://leetcode.cn/problems/rotate-image/

// 48. 旋转图像
// 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
// 你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..len-1 {
            for j in i..len {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        matrix.iter_mut().for_each(|line| line.reverse());
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut map = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                map.insert((i, j), matrix[i][j]);
            }
        }
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = map[&(n - 1 - j, i)];
            }
        }
    }
}