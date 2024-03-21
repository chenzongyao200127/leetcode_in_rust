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

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        fn calc(
            high: &str,
            min_sum: i32,
            max_sum: i32,
            cache: &mut HashMap<(usize, i32, bool), i32>,
        ) -> i32 {
            fn dfs(
                index: usize,
                sum: i32,
                is_limit: bool,
                high_chars: &Vec<char>,
                min_sum: i32,
                max_sum: i32,
                cache: &mut HashMap<(usize, i32, bool), i32>,
            ) -> i32 {
                if sum > max_sum {
                    return 0;
                }
                if index == high_chars.len() {
                    return (sum >= min_sum) as i32;
                }
                if let Some(&cached) = cache.get(&(index, sum, is_limit)) {
                    return cached;
                }

                let mut result = 0;
                let upper_bound = if is_limit {
                    high_chars[index].to_digit(10).unwrap() as i32
                } else {
                    9
                };
                for digit in 0..=upper_bound {
                    result += dfs(
                        index + 1,
                        sum + digit,
                        is_limit && digit == upper_bound,
                        high_chars,
                        min_sum,
                        max_sum,
                        cache,
                    );
                }

                cache.insert((index, sum, is_limit), result);
                result
            }

            let high_chars = high.chars().collect::<Vec<char>>();
            let mut cache = HashMap::new();
            dfs(0, 0, true, &high_chars, min_sum, max_sum, &mut cache)
        }

        let is_num1_valid = {
            let num1_sum: i32 = num1.chars().map(|c| c.to_digit(10).unwrap() as i32).sum();
            min_sum <= num1_sum && num1_sum <= max_sum
        };

        (calc(&num2, min_sum, max_sum, &mut HashMap::new())
            - calc(&num1, min_sum, max_sum, &mut HashMap::new())
            + is_num1_valid as i32)
            % 1_000_000_007
    }
}

struct FrequencyTracker {
    num_freq: HashMap<i32, i32>,
    freq_count: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            num_freq: HashMap::new(),
            freq_count: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let freq = self.num_freq.entry(number).or_insert(0);
        *self
            .freq_count
            .entry(*freq)
            .and_modify(|e| *e -= 1)
            .or_insert(0);
        *freq += 1;
        *self.freq_count.entry(*freq).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        if let Some(freq) = self.num_freq.get_mut(&number) {
            *self
                .freq_count
                .entry(*freq)
                .and_modify(|e| *e -= 1)
                .or_insert(0);
            *freq -= 1;
            if *freq > 0 {
                *self.freq_count.entry(*freq).or_insert(0) += 1;
            } else {
                self.num_freq.remove(&number);
            }
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.freq_count
            .get(&frequency)
            .map_or(false, |&count| count > 0)
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */

fn main() {
    let num1 = "123".to_string();
    let num2 = "456".to_string();
    let min_sum = 10;
    let max_sum = 100;
    let result = Solution::count(num1, num2, min_sum, max_sum);
    println!("Result: {}", result);
}
