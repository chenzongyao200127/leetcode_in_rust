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

pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
    let n = heights.len();
    let mut answer = vec![0; n]; // Initialize answer vector with zeros
    let mut stack: Vec<i32> = Vec::new();

    for (idx, &h) in heights.iter().enumerate().rev() {
        while !stack.is_empty() && *stack.last().unwrap() < h {
            stack.pop();
            answer[idx] += 1;
        }

        if !stack.is_empty() {
            answer[idx] += 1;
        }

        stack.push(h)
    }

    answer
}

fn main() {
    let ans = can_see_persons_count(vec![10, 6, 8, 5, 11, 9]);
    println!("{:?}", ans);
}
