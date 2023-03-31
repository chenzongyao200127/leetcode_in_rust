// 2367_Number_of_Arithmetic_Triplets
// https://leetcode.cn/problems/number-of-arithmetic-triplets/

use std::collections::HashSet;
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        nums.into_iter().for_each(|num| { set.insert(num); });
        let mut ans = 0;
        for num in set.iter() {
            if set.contains(&(*num + diff)) && set.contains(&(*num + diff * 2)) && set.len() >= 3 {
                ans += 1;
            }
        }
    
        ans
    }
}

use std::collections::HashSet;
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut num_set = HashSet::<i32>::new();
        let mut valid_nums = HashSet::<i32>::new();
        nums.iter().for_each(|&num| {
            if num_set.contains(&(num - diff)) {
                valid_nums.insert(num);
            }
            num_set.insert(num);
        });
        valid_nums.iter().filter(|&&num| valid_nums.contains(&(num - diff))).count() as i32
    }
}
