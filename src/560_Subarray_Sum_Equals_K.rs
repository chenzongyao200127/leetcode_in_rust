// 560. Subarray Sum Equals K
// https://leetcode.cn/problems/subarray-sum-equals-k/

// 给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的连续子数组的个数 。

// 方法二：前缀和 + 哈希表优化
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut pre_sum = 0;
        let mut map: HashMap<i32, usize> = HashMap::new();
        map.insert(0, 1);
        for i in 0..nums.len() {
            pre_sum += nums[i];
            if let Some(val) = map.get(&(pre_sum - k)) {
                ans += val;
            }
            map.entry(pre_sum).and_modify(|cnt| *cnt+=1).or_insert(1);
        }

        ans as i32
    }
}



// 方法一：枚举
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in (0..=i).rev() {
                sum += nums[j];
                if (sum == k) {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_adds = vec![];
        let mut ans = 0;
        let mut tmp = 0;
        for i in 0..nums.len() {
            tmp += nums[i];
            pre_adds.push(tmp);
        }
        let mut minus = 0;
        for i in 0..nums.len() {
            minus = nums[i];
            for pre in pre_adds {
                if pre == k {
                    ans += 1;
                }
                pre -= minus;
            }
        }

        ans
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let prefix_sum: Vec<i32> = std::iter::once(0)
            .chain(nums.iter().scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            }))
            .collect();

        let mut hash = HashMap::new();
        hash.insert(0, 1);

        let mut ans = 0;
        for &s in prefix_sum.iter().skip(1) {
            ans += hash.get(&(s - k)).unwrap_or(&0);
            *hash.entry(s).or_insert(0) += 1;
        }
        ans
    }
}