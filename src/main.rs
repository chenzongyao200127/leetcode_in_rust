use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() {
    let ans = rob(vec![2,7,9,3,1]);
    assert_eq!(ans, 12);
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut dp = vec![];
    dp.push(0);
    dp.push(nums[0]);
    for i in 1..nums.len() {
        dp.push((nums[i] + dp[i-1]).max(dp[i]));
    }
    println!("{:?}",dp);
    
    dp[dp.len()-1] as i32
}