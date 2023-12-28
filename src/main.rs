use std::collections::HashMap;

pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
    let n = nums.len();

    let mut s: Vec<i64> = (0..n).map(|i| i as i64 * x as i64).collect();
    for i in 0..n {
        let mut mn = nums[i];
        for j in i..(n + i) {
            mn = mn.min(nums[j % n]);
            s[j - i] += mn as i64;
        }
    }
    *s.iter().min().unwrap()
}

fn main() {
    // let solution = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]); // Example weights
    // println!("Picked index: {:?}", solution.pick());

    let ans = min_cost(vec![4, 3, 2, 1, 1, 2, 3, 1], 2);
    println!("{:?}", ans);
}
