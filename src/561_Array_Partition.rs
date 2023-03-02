// 561. Array Partition
// https://leetcode.cn/problems/array-partition/

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
    }
}

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() {
            if i & 1 == 0 {
                ans += nums[i];
            }
        }
        ans
    }
}


impl Solution {
    // counting sort
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let max: i32 = *nums.iter().max().unwrap();
        let min: i32 = *nums.iter().min().unwrap();
        let k: usize = (max - min + 1) as usize;

        let mut num_freq = Vec::<i32>::with_capacity(k);
        num_freq.resize(k, 0);

        for x in nums.iter() {
            num_freq[(x - min) as usize] += 1;
        }

        let mut is_even: bool = true;
        let mut sum: i32 = 0;
        for x in (0..k) {
            while num_freq[x] > 0 {
                if is_even { sum += x as i32 + min; }
                is_even = !is_even;
                num_freq[x] -= 1;
            }
        }

        sum
    }
}