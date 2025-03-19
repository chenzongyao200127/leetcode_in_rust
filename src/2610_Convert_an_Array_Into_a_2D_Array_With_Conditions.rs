// 2610_Convert_an_Array_Into_a_2D_Array_With_Conditions
// https://leetcode.cn/problems/convert-an-array-into-a-2d-array-with-conditions/description/?envType=daily-question&envId=2025-03-19

// You are given an integer array nums. You need to create a 2D array from nums satisfying the following conditions:

// The 2D array should contain only the elements of the array nums.
// Each row in the 2D array contains distinct integers.
// The number of rows in the 2D array should be minimal.
// Return the resulting array. If there are multiple answers, return any of them.

// Note that the 2D array can have a different number of elements on each row.

use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut row_map: HashMap<i32, usize> = HashMap::new();

        for num in nums {
            let row_idx = *row_map.get(&num).unwrap_or(&0);

            if row_idx >= res.len() {
                res.push(vec![]);
            }

            res[row_idx].push(num);
            row_map.insert(num, row_idx + 1);
        }

        res
    }
}
