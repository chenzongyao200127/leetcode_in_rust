// 575. Distribute Candies
// https://leetcode.cn/problems/distribute-candies/

use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let len = candy_type.len();
        let mut set: HashSet<i32> = HashSet::new();
        for candy in candy_type {
            set.insert(candy);
        }
        (len/2).min(set.len()) as i32
    }
}

use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        (candy_type.len() / 2).min(candy_type.iter().collect::<HashSet<_>>().len()) as i32
    }
}

use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let set = candy_type.iter().collect::<HashSet<_>>();
        (candy_type.len() as i32 / 2).min(set.len() as i32)
    }
}
