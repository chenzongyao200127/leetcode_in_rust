use std::collections::{HashSet, HashMap};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = four_sum(vec![0], 0);
    assert_eq!(ans, vec![vec![]]);
}

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut set: HashSet<Vec<i32>> = HashSet::new();
    for i in 0..nums.len()-3 {
        for j in i+1..nums.len()-2 {
            for m in j+1..nums.len()-1 {
                for n in m+1..nums.len() {
                    if nums[i]+nums[j]+nums[m]+nums[n] == target {
                        set.insert(vec![nums[i],nums[j],nums[m],nums[n]]);
                    }
                }
            }
        }
    }
    for item in set {
        ans.push(item);
    }
    ans
}