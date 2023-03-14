// 1605_Find_Valid_Matrix_Given_Row_and_Column_Sums
// https://leetcode.cn/problems/find-valid-matrix-given-row-and-column-sums/

// 贪心构造~
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; col_sum.len()]; row_sum.len()];
        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                if col_sum[j] == 0 || row_sum[i] == 0 {
                    ans[i][j] = 0;
                } else {
                    ans[i][j] = row_sum[i].min(col_sum[j]);
                    row_sum[i] -= ans[i][j];
                    col_sum[j] -= ans[i][j];
                }
            }
        }
        ans
    }
}

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let m = row_sum.len();
        let n = col_sum.len();
        let mut res: Vec<Vec<i32>> = vec![vec![0; n]; m];
        for (i, valuei) in row_sum.iter_mut().enumerate() {
            for (j, valuej) in col_sum.iter_mut().enumerate() {
                let v:i32 = if valuei < valuej {valuei.clone()} else {valuej.clone()};
                res[i][j] = v;
                *valuei -= v;
                *valuej -= v;
            }
        }
        res
    }
}
