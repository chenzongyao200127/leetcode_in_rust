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

use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut ans = Vec::new();
    let mut deque = VecDeque::new();

    for i in 0..n {
        // 移除超出窗口范围的元素
        while !deque.is_empty() && deque.front().unwrap() < &(i as i32 - k as i32 + 1) {
            deque.pop_front();
        }

        // 移除比当前元素小的元素，因为它们不可能成为最大值
        while !deque.is_empty() && nums[*deque.back().unwrap() as usize] < nums[i] {
            deque.pop_back();
        }

        deque.push_back(i as i32);

        if i >= k - 1 {
            ans.push(nums[*deque.front().unwrap() as usize]);
        }
    }

    ans
}

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    const ASCII_LOWER_A: u8 = b'a';
    const ASCII_LOWER_Z: u8 = b'z';
    let mut frequency = [0; 256];

    let input = s.as_bytes();

    // Count the frequency of each character in the input string
    for &byte in input {
        frequency[byte as usize] += 1;
    }

    let mut result = String::new();
    let mut last_char = 0;
    let mut repeat_count = 0;

    for _ in 0..input.len() {
        for char_code in (ASCII_LOWER_A..=ASCII_LOWER_Z).rev() {
            if frequency[char_code as usize] == 0 {
                continue;
            }

            if char_code == last_char {
                if repeat_count >= repeat_limit {
                    continue;
                }
                repeat_count += 1;
            } else {
                repeat_count = 1;
                last_char = char_code;
            }

            frequency[char_code as usize] -= 1;
            result.push(char_code as char);
            break;
        }
    }

    result
}

fn main() {
    let ans = max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    println!("{:?}", ans);
}
