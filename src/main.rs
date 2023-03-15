use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() {
    let ans = find_duplicate(vec![1,3,4,2,2]);
    assert_eq!(ans, 2);

    let ans = find_duplicate(vec![4,3,2,5,1,3]);
    assert_eq!(ans, 3);
}

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = 0 as usize;
    let mut fast = 0 as usize;
    loop {
        fast = nums[nums[fast] as usize] as usize;
        slow = nums[slow] as usize;
        if fast == slow {
            break;
        }
    }
    let mut fast = 0 as usize;
    loop {
        fast = nums[fast] as usize;
        slow = nums[slow] as usize;
        if slow == fast {
            return slow as i32;
        }
    }
}