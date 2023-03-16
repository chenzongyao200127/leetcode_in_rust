use std::collections::HashSet;
use std::collections::VecDeque;

pub fn main() {
    let ans = count_subarrays(vec![3,2,1,4,5], 4);
    assert_eq!(ans, 3);
}

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len() as i32;
    let max_len = (n-k).min(k) * 2 + 1;
    let mut k_idx = 0;
    for i in 0..nums.len() {
        if nums[i] == k {
            k_idx = i;
            break;
        }
    }
    let mut pre_sum = vec![0; nums.len()];
    let mut pre = 0;
    for i in 0..nums.len() {
        if nums[i] < k {
            pre -= 1;
        } else if nums[i] > k {
            pre += 1;
        }
        pre_sum[i] = pre;
    }

    let mut ans = 0;

    for left in ((k_idx as i32 - (max_len - 1)).max(0)) as usize ..=k_idx {
        for right in k_idx..=((k_idx as i32 + (max_len - 1)).min(n-1)) as usize {
            if (right - left + 1) as i32 > max_len {
                break;
            }
            if left > 0 {
                let tmp = pre_sum[right] - pre_sum[left-1];
                if tmp == 0 || tmp == 1 {
                    ans += 1;
                }
            } else {
                let tmp = pre_sum[right];
                if tmp == 0 || tmp == 1 {
                    ans += 1;
                }
            }
        }
    }

    ans
}
