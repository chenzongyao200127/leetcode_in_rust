// 1253_Reconstruct_a_2-Row_Binary_Matrix
// https://leetcode.cn/problems/reconstruct-a-2-row-binary-matrix/


// 输入：upper = 2, lower = 1, colsum = [1,1,1]
// 输出：[[1,1,0],[0,0,1]]
// 解释：[[1,0,1],[0,1,0]] 和 [[0,1,1],[1,0,0]] 也是正确答案。

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; colsum.len()]; 2];
        for i in 0..colsum.len() {
            if colsum[i] == 2 {
                matrix[0][i] = 1;
                matrix[1][i] = 1;    
            }
        }

        Self::dfs(0, upper, lower, colsum, matrix);
        return matrix
    }

    fn dfs(cur_col: usize, upper: i32, lower: i32, colsum: &Vec<i32>, matrix: &mut Vec<Vec<I32>>) {
        if cur_col == colsum.len() {
            if colsum[cur_col] == 1 {
                colsum[0][cur_col] = 1;
                if is_valid(upper, lower, matrix) {
                    return
                }
                colsum[0][cur_col] = 0;
                colsum[1][cur_col] = 0;
                if is_valid(upper, lower, matrix) {
                    return
                }
            } else {
                if is_valid(upper, lower, matrix) {
                    return
                }
            }
        }
        for i in 0..=1 {
            colsum[0][cur_col] = 1;
            next_col = i+1;
            while next_col < colsum.len() && colsum[next_col] != 1 {
                next_col += 1;
            }
            dfs(next_col, upper, lower, colsum, matrix);
            colsum[0][cur_col] = 0;
        }
    }

    fn is_valid(upper: i32, lower: i32, matrix: &Vec<Vec<I32>>) -> bool {
        if matrix[0].iter().sum() == upper && matrix[1].iter().sum() == lower {
            return true;
        }
        false
    }
}