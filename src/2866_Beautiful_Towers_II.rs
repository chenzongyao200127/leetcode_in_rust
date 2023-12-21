// 2866_Beautiful_Towers_II
// https://leetcode.cn/problems/beautiful-towers-ii/description/

use std::collections::HashMap;

// 记忆化 + 单调栈 (超时)
impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let length = max_heights.len();
        let mut sorted_heights = max_heights.iter().enumerate().collect::<Vec<_>>();
        sorted_heights.sort_unstable_by(|&(_, a), &(_, b)| b.cmp(a));

        let mut ans = 0;
        let mut memo = HashMap::new();

        for &(index, &height) in &sorted_heights {
            let height = height as i64;

            // Pre-emptive check to potentially skip unnecessary calculations
            if height * length as i64 <= ans {
                continue;
            }

            let left_sum = Self::calculate_sum(&max_heights, index, -1, height, &mut memo);
            let right_sum = Self::calculate_sum(&max_heights, index, 1, height, &mut memo);

            ans = ans.max(left_sum + right_sum - height);
        }

        ans
    }

    fn calculate_sum(
        max_heights: &[i32],
        start: usize,
        dir: i32,
        initial_height: i64,
        memo: &mut HashMap<(usize, i32), i64>,
    ) -> i64 {
        if let Some(&sum) = memo.get(&(start, dir)) {
            return sum;
        }

        let mut sum = initial_height;
        let mut last_height = initial_height;
        let mut idx = start as i32 + dir;

        while idx >= 0 && (idx as usize) < max_heights.len() {
            let current_height = max_heights[idx as usize] as i64;
            if current_height <= last_height {
                sum += current_height;
            } else {
                sum += last_height;
            }
            last_height = last_height.min(current_height);
            idx += dir;
        }

        memo.insert((start, dir), sum);
        sum
    }
}

// DP + 单调栈
impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut res: i64 = 0;
        let mut prefix = vec![0i64; n];
        let mut suffix = vec![0i64; n];
        let mut stack1: Vec<usize> = Vec::new();
        let mut stack2: Vec<usize> = Vec::new();

        for i in 0..n {
            while !stack1.is_empty() && max_heights[i] < max_heights[*stack1.last().unwrap()] {
                stack1.pop();
            }
            prefix[i] = if stack1.is_empty() {
                (i as i64 + 1) * max_heights[i] as i64
            } else {
                let top = *stack1.last().unwrap();
                prefix[top] + (i as i64 - top as i64) * max_heights[i] as i64
            };
            stack1.push(i);
        }

        for i in (0..n).rev() {
            while !stack2.is_empty() && max_heights[i] < max_heights[*stack2.last().unwrap()] {
                stack2.pop();
            }
            suffix[i] = if stack2.is_empty() {
                (n as i64 - i as i64) * max_heights[i] as i64
            } else {
                let top = *stack2.last().unwrap();
                suffix[top] + (top as i64 - i as i64) * max_heights[i] as i64
            };
            stack2.push(i);
            res = std::cmp::max(res, prefix[i] + suffix[i] - max_heights[i] as i64);
        }

        res
    }
}
