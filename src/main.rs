use std::collections::HashMap;


pub fn main() {
    // let ans = count_subgraphs_for_each_diameter(4, vec![vec![1,2],vec![2,3],vec![2,4]]);
    // assert_eq!(ans, vec![3,4,0]);

    let ans = min_sub_array_len(4, vec![1,4,4]);
    assert_eq!(ans, 2);
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    if nums.iter().sum::<i32>() < target {
        return 0;
    }
    let mut start = 0 as usize;
    let mut end = 0 as usize;
    let mut tmp_sum = nums[start];
    while tmp_sum < target {
        end += 1;
        tmp_sum += nums[end];
    }
    // println!("{:?}", (start, end));
    let mut ans = end - start + 1;
    while start < nums.len() {
        start += 1;
        tmp_sum -= nums[start-1];
        while tmp_sum < target && end < nums.len()-1 {
            end += 1;
            tmp_sum += nums[end];
        }
        // println!("{:?}", (start, end));
        if tmp_sum >= target {
            ans = ans.min(end - start + 1);
        }
        if ans == 1 {
            return 1;
        }
    }
    ans as i32
}