// 1253_Reconstruct_a_2-Row_Binary_Matrix
// https://leetcode.cn/problems/reconstruct-a-2-row-binary-matrix/

// 贪心
// 输入：upper = 2, lower = 1, colsum = [1,1,1]
// 输出：[[1,1,0],[0,0,1]]
// 解释：[[1,0,1],[0,1,0]] 和 [[0,1,1],[1,0,0]] 也是正确答案。
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; colsum.len()]; 2];
        let mut upper_remaining = upper;
        let mut lower_remaining = lower;

        for i in 0..colsum.len() {
            if colsum[i] == 2 {
                matrix[0][i] = 1;
                matrix[1][i] = 1;
                upper_remaining -= 1;
                lower_remaining -= 1;
            }
        }

        for i in 0..colsum.len() {
            if colsum[i] == 1 {
                if upper_remaining > 0 {
                    matrix[0][i] = 1;
                    upper_remaining -= 1;
                } else {
                    matrix[1][i] = 1;
                    lower_remaining -= 1;
                }
            }
        }

        if upper_remaining == 0 && lower_remaining == 0 {
            matrix
        } else {
            vec![]
        }
    }
}




impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let s: i32 = colsum.iter().sum();
        if s != upper + lower {
            return vec![];
        }

        let c1 = colsum.iter().filter(|&a| *a == 2).count() as i32;
        if c1 > lower || c1 > upper {
            return vec![];
        }

        let n = colsum.len();
        let mut ans = vec![vec![0; n]; 2];
        let mut diff = upper - lower;
        for i in 0..n {
            if colsum[i] == 2 {
                ans[0][i] = 1;
                ans[1][i] = 1;
            } else if colsum[i] == 1 {
                if diff < 0 {
                    diff += 1;
                    ans[1][i] = 1;
                } else {
                    diff -= 1;
                    ans[0][i] = 1;
                }
            }
        }
        ans
    }
}