pub fn main() {
    let ans = max_width_of_vertical_area(vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]]);
    assert_eq!(ans, 3);
}


use std::collections::HashSet;

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
