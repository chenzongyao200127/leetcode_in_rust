// 2643_Row_With_Maximum_Ones
// https://leetcode.cn/problems/row-with-maximum-ones/description/?envType=daily-question&envId=2025-03-22

// Given a m x n binary matrix mat, find the 0-indexed position of the row that contains the maximum count of ones, and the number of ones in that row.

// In case there are multiple rows that have the maximum count of ones, the row with the smallest row number should be selected.

// Return an array containing the index of the row, and the number of ones in it.

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_ones = 0;
        let mut max_row = 0;
        for (i, row) in mat.iter().enumerate() {
            let ones = row.iter().filter(|&&x| x == 1).count();
            if ones > max_ones {
                max_ones = ones;
                max_row = i;
            }
        }
        vec![max_row as i32, max_ones as i32]
    }
}
