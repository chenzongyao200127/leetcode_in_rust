use std::collections::HashMap;
use std::collections::VecDeque;

pub fn main() {
    // let ans = count_subgraphs_for_each_diameter(4, vec![vec![1,2],vec![2,3],vec![2,4]]);
    // assert_eq!(ans, vec![3,4,0]);

    let ans = restore_matrix(vec![14, 9], vec![6,9,8]);
    assert_eq!(ans, vec![vec![]]);
}


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