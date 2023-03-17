use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() {
    let ans = answer_queries(vec![4,5,2,1], vec![0,3,10,21]);
    assert_eq!(ans, vec![2, 2]);
}

pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut pre = vec![];
    pre.push(0);
    let mut pre_sum = 0;
    for n in nums {
        pre_sum += n;
        pre.push(pre_sum);
    }
    // println!("{:?}", pre);
    let mut ans = vec![];
    for i in 0..queries.len() {
        let mut idx = 0;
        while idx < pre.len() && pre[idx as usize] <= queries[i] {
            // println!("{:?}", (idx, pre[idx], queries[i]));
            idx += 1;
    
        }
        ans.push(idx as i32 - 1);
    }
    ans
}