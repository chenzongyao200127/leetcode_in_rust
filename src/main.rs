
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

fn main() {
    let ans = maximum_rows(
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
        2,
    );
    println!("{:?}", ans);
}
