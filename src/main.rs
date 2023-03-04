use std::collections::{HashMap, HashSet};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    // let ans = candy(vec![1,2,87,87,87,2,1]);
    // assert_eq!(ans, 13);
    let ans = count_triplets(vec![0,0,0]);
    assert_eq!(ans, 12);
}

pub fn count_triplets(nums: Vec<i32>) -> i32 {
    let mut cnt = vec![0; 1<<16];
    for x in nums.iter() {
        for y in nums.iter() {
            cnt[(x & y) as usize] += 1;
        }
    }
    let mut ans = 0;
    for x in nums.iter() {
        let mut mask = 0;
        while mask < (1<<16) {
            if (x & mask) == 0 {
                ans += cnt[mask as usize];
            }
            mask += 1;
        }
    }
    ans
}