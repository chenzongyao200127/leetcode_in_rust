// 2397_被列覆盖的最多行数
// https://leetcode.cn/problems/maximum-rows-covered-by-columns/description/?envType=daily-question&envId=2024-01-04

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = 0;

        for mask in 0u32..(1 << n) {
            if mask.count_ones() == num_select as u32 {
                let mut cnt = 0;
                for row in &matrix {
                    let mut row_valid = true;
                    for (j, &item) in row.iter().enumerate() {
                        if ((mask >> j) & 1) == 0 && item == 1 {
                            row_valid = false;
                            break; // 退出内层循环
                        }
                    }
                    if row_valid {
                        cnt += 1;
                    }
                }
                ans = ans.max(cnt);
            }
        }

        ans
    }
}

pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let n = matrix[0].len();
    let mut ans = 0;

    // Iterate through all combinations using bitmasks
    for mask in 0u32..(1 << n) {
        if mask.count_ones() == num_select as u32 {
            let mut cnt = 0;
            'outer: for row in &matrix {
                for (j, &item) in row.iter().enumerate() {
                    // If the column is not selected but the cell is 1, skip this row
                    if ((mask >> j) & 1) == 0 && item == 1 {
                        continue 'outer;
                        // break;
                    }
                }
                // Row is valid, increment count
                cnt += 1;
            }
            ans = ans.max(cnt);
        }
    }

    ans
}
