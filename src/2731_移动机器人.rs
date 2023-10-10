// 2731_移动机器人
// https://leetcode.cn/problems/movement-of-robots/description/

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let remainder = 1000_000_000 + 7;
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let dirs: Vec<i64> = s.chars().map(|x| {
            match x {
                'R' => 1,
                'L' => -1,
                _ => panic!("Unexpected character"),
            }
        }).collect();

        for i in 0..nums.len() {
            nums[i] = nums[i] + dirs[i] * d as i64;
        }

        nums.sort_unstable();

        let mut res = 0;
        let mut sum = 0;

        for (i, &n) in nums.iter().enumerate() {
            res = (res + i as i64 * n - sum) % remainder;
            sum += n;
        }

        res as i32
    }
}


// 超时
impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let remainder = 1000_000_000 + 7;
        let mut nums: Vec<f64> = nums.into_iter().map(|x| x as f64).collect();
        let mut dirs: Vec<i32> = s.chars().map(|x| {
            match x {
                'R' => 1,
                'L' => -1,
                _ => panic!("Unexpected character"),
            }
        }).collect();

        for _ in 0..(2 * d as usize) {
            for i in 0..nums.len() {
                nums[i] = nums[i] + dirs[i] as f64 * 0.5;
            }
            for i in 0..nums.len() - 1 {
                for j in i+1 ..nums.len() {
                    if (nums[i] - nums[j]).abs() < f64::EPSILON {
                        dirs[i] = dirs[i] * (-1);
                        dirs[j] = dirs[j] * (-1);        
                    }
                }
            }
        }

        let mut res = 0.0;

        for i in 0..nums.len() - 1 {
            for j in i+1 ..nums.len() {
                res += (nums[i] - nums[j]).abs();
            }
        }

        (res % (remainder as f64)) as i32
    }
}